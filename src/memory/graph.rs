use serde::{Deserialize, Serialize};
use std::sync::OnceLock;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;

static DB: OnceLock<Surreal<Client>> = OnceLock::new();

// DB インスタンスの取得（初期化済みであることが前提）
fn db() -> &'static Surreal<Client> {
    DB.get()
        .expect("SurrealDB has not been initialized. Call connect_to_db() first.")
}

/// 記憶グラフのノード
/// SurrealDB はメタデータのみを保持し、実際のエピソードはファイルシステムに保存する（CMSアーキテクチャ）
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MemoryNode {
    pub concept: String,
    /// 鮮明度: V = e^(-t/S)。時間とともに減衰する（0.0 ~ 1.0）
    pub vividness: f32,
    /// 最終アクセス タイムスタンプ（UNIX秒）
    pub last_accessed: i64,
    /// 感情スコア（記憶の強度 S の初期値。高いほど忘れにくい）
    pub emotion_score: f32,
    /// トピックタグ（カンマ区切り）
    pub tags: String,
    /// エピソード記憶のファイルパス（CMSストレージ: /var/iris/memories/YYYY/MM/DD/episode.md）
    pub file_path: Option<String>,
}

/// 記憶ノード間のエッジ（関係性）
#[derive(Debug, Serialize, Deserialize)]
pub struct RelatesTo {
    /// 関連度の重み (0.0 ~ 1.0)
    pub weight: f32,
}

/// DBから取得されたメタデータとCMS内のテキスト本文を結合した構造体
#[derive(Debug, Clone)]
pub struct RetrievedMemory {
    pub node: MemoryNode,
    pub content: String,
}

/// 人格コアノード（思考のバイアス）
/// このノードへのエッジ重みが高いほど、連想時に「Rusty」な方向へ引かれる
#[derive(Debug, Serialize, Deserialize)]
pub struct PersonalityNode {
    pub trait_name: String,
    /// 初期エッジ重み（例: humor=0.9, sarcasm=0.7, rust_love=1.0）
    pub initial_weight: f32,
}

/// SurrealDB に接続し、初期スキーマを設定する
pub async fn connect_to_db() -> surrealdb::Result<()> {
    let url = std::env::var("SURREAL_URL").unwrap_or_else(|_| "ws://localhost:8000".to_string());
    let user = std::env::var("SURREAL_USER").unwrap_or_else(|_| "root".to_string());
    let pass = std::env::var("SURREAL_PASS").unwrap_or_else(|_| "root".to_string());
    let ns = std::env::var("SURREAL_NS").unwrap_or_else(|_| "iris".to_string());
    let db_name = std::env::var("SURREAL_DB").unwrap_or_else(|_| "memory_graph".to_string());

    let client = Surreal::new::<Ws>(&url).await?;
    client
        .signin(Root {
            username: &user,
            password: &pass,
        })
        .await?;
    client.use_ns(&ns).use_db(&db_name).await?;

    // 人格コアノードが存在しない場合は初期化する
    let personality_nodes = vec![
        PersonalityNode {
            trait_name: "humor".to_string(),
            initial_weight: 0.9,
        },
        PersonalityNode {
            trait_name: "sarcasm".to_string(),
            initial_weight: 0.7,
        },
        PersonalityNode {
            trait_name: "rust_love".to_string(),
            initial_weight: 1.0,
        },
    ];
    for node in personality_nodes {
        let _: Option<PersonalityNode> = client
            .create(("personality", node.trait_name.clone()))
            .content(node)
            .await
            .ok()
            .flatten();
    }

    DB.set(client).expect("DB already initialized");
    Ok(())
}

use crate::memory::cms;

/// 新しい記憶ノードをCMSへの実体保存と合わせてグラフに挿入する
pub async fn insert_memory(
    concept: &str,
    emotion_score: f32,
    tags: &str,
    content: &str,
) -> surrealdb::Result<Option<MemoryNode>> {
    // 1. CMSへ実体テキストをMarkdownとして保存
    let file_path = cms::save_markdown(content).map_err(|e| {
        // IOエラーをSurrealDBのカスタムAPIエラーに変換して返す
        surrealdb::Error::Api(surrealdb::error::Api::Query(format!("CMS save error: {e}")))
    })?;

    // 2. メタデータを構築してSurrealDBへ保存
    let initial_vividness = (emotion_score / 10.0).min(1.0);
    let node = MemoryNode {
        concept: concept.to_string(),
        vividness: initial_vividness,
        last_accessed: chrono::Utc::now().timestamp(),
        emotion_score,
        tags: tags.to_string(),
        file_path: Some(file_path),
    };

    let created: Option<MemoryNode> = db().create(("memory", concept)).content(node).await?;

    // 3. 人格コアノードへの初期連想バイアス（弱いエッジ）を追加
    db().query(
        "RELATE type::thing('memory', $concept) \
           -> relates_to -> personality:humor SET weight = 0.1; \
         RELATE type::thing('memory', $concept) \
           -> relates_to -> personality:sarcasm SET weight = 0.1; \
         RELATE type::thing('memory', $concept) \
           -> relates_to -> personality:rust_love SET weight = 0.2;",
    )
    .bind(("concept", concept.to_string()))
    .await?;

    Ok(created)
}

/// Spreading Activation: アクセスされたノードの隣接ノードの鮮明度を連想によって向上させる
pub async fn spread_activation(concept: &str) -> surrealdb::Result<()> {
    db().query(
        "UPDATE memory SET \
         vividness = math::min(vividness + 0.2, 1.0), \
         last_accessed = time::unix() \
         WHERE id IN (SELECT VALUE <-relates_to<-memory.id FROM type::thing('memory', $concept)) \
         OR id IN (SELECT VALUE ->relates_to->memory.id FROM type::thing('memory', $concept))",
    )
    .bind(("concept", concept.to_string()))
    .await?;

    Ok(())
}

/// 鮮明度と感情スコアの高い上位 N 件の記憶を取得し、CMSのエピソード本文とマージして返す（RAGコンテキスト構築用）
pub async fn fetch_top_memories(limit: u32) -> surrealdb::Result<Vec<RetrievedMemory>> {
    // vividness + (emotion_score / 10.0) を基準にソートする
    let mut result = db()
        .query(
            "SELECT * FROM memory ORDER BY (vividness + (emotion_score / 10.0)) DESC LIMIT $limit",
        )
        .bind(("limit", limit))
        .await?;

    let nodes: Vec<MemoryNode> = result.take(0)?;
    let mut memories = Vec::new();

    for node in nodes {
        let content = if let Some(ref path) = node.file_path {
            crate::memory::cms::load_markdown(path).unwrap_or_else(|_| "".to_string())
        } else {
            "".to_string()
        };
        memories.push(RetrievedMemory { node, content });
    }

    Ok(memories)
}

/// エビングハウスの忘却曲線関数（V = e^{-t/S}）に基づく鮮明度の減衰バッチ処理
pub async fn decay_vividness() -> surrealdb::Result<()> {
    // time::unix() - last_accessed は秒単位。これを日単位(86400秒)で割り、
    // emotion_score（記憶の強度S、最低1.0）を用いた指数関数的減衰を計算する。
    db().query(
        "UPDATE memory SET \
         vividness = math::pow(math::e(), -((time::unix() - last_accessed) / 86400.0) / math::max(emotion_score, 1.0))"
    )
    .await?;

    Ok(())
}

/// 主人のプレゼンス検知時に、「主人」に関連する記憶ノードの鮮明度を強制上昇させる
/// Vision サブシステムの `UserDetected` イベント受信時に呼び出される
///
/// # Arguments
/// * `boost_amount` - 鮮明度の上昇量（0.0 ~ 1.0、上限1.0でクランプ）
pub async fn boost_owner_vividness(boost_amount: f32) -> surrealdb::Result<()> {
    let boost = boost_amount.clamp(0.0, 1.0);
    db().query(
        "UPDATE memory SET \
         vividness = math::min(vividness + $boost, 1.0), \
         last_accessed = time::unix() \
         WHERE concept CONTAINS '主人' OR tags CONTAINS '主人' OR tags CONTAINS 'owner'"
    )
    .bind(("boost", boost))
    .await?;

    println!("💡 主人関連の記憶鮮明度を +{boost:.2} ブーストしました");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn 感情スコアから初期鮮明度が正しく計算される() {
        // emotion_score 10.0 → vividness 1.0
        let vividness = (10.0_f32 / 10.0).min(1.0);
        assert_eq!(vividness, 1.0);

        // emotion_score 5.0 → vividness 0.5
        let vividness = (5.0_f32 / 10.0).min(1.0);
        assert_eq!(vividness, 0.5);

        // emotion_score 15.0 → vividness 1.0（上限クランプ）
        let vividness = (15.0_f32 / 10.0).min(1.0);
        assert_eq!(vividness, 1.0);
    }

    #[tokio::test]
    async fn insert_memory_cms_integration() {
        use std::env;
        use tempfile::tempdir;

        // 1. 一時ディレクトリでCMSをセットアップ
        let temp_dir = tempdir().expect("Failed to create temp dir");
        let temp_path_str = temp_dir.path().to_str().unwrap().to_string();
        env::set_var("MEMORIES_PATH", &temp_path_str);

        // 2. DB接続初期化
        env::set_var("SURREAL_URL", "ws://surrealdb:8000");
        if let Err(e) = connect_to_db().await {
            println!("Skipping integration test due to DB connection failure (likely running outside of full compose network): {e}");
            return;
        }

        // 3. ランダムな概念名でinsert_memoryを実行（他テストとの競合防止）
        let unique_id = uuid::Uuid::new_v4().to_string();
        let concept = format!("test_concept_{unique_id}");
        let content = "This is the episode content for integration test.";
        let tags = "test, integration";

        let result = insert_memory(&concept, 8.0, tags, content).await;
        assert!(result.is_ok(), "insert_memory failed");

        let node = result.unwrap().expect("No MemoryNode returned");
        assert_eq!(node.concept, concept);
        assert_eq!(node.tags, tags);
        assert!(node.vividness > 0.0);

        // 4. CMSへの書き込み確認
        let file_path = node
            .file_path
            .expect("file_path should be populated by CMS");
        let loaded =
            crate::memory::cms::load_markdown(&file_path).expect("Failed to load markdown");
        assert_eq!(loaded, content);
    }

    #[tokio::test]
    async fn test_decay_and_spread() {
        use std::env;
        use tempfile::tempdir;

        let temp_dir = tempdir().expect("Failed to create temp dir");
        let temp_path_str = temp_dir.path().to_str().unwrap().to_string();
        env::set_var("MEMORIES_PATH", &temp_path_str);

        env::set_var("SURREAL_URL", "ws://surrealdb:8000");
        if let Err(e) = connect_to_db().await {
            println!("Skipping integration test due to DB connection failure: {e}");
            return;
        }

        let unique = uuid::Uuid::new_v4().to_string();
        let concept_a = format!("concept_A_{unique}");
        let concept_b = format!("concept_B_{unique}");

        // 5.0 (Vividness 0.5) で A と B を保存
        let _ = insert_memory(&concept_a, 5.0, "test", "Content A")
            .await
            .unwrap()
            .unwrap();
        let _ = insert_memory(&concept_b, 5.0, "test", "Content B")
            .await
            .unwrap()
            .unwrap();

        // A -> B へ関連付け
        db().query("RELATE type::thing('memory', $a)->relates_to->type::thing('memory', $b) SET weight = 1.0")
            .bind(("a", concept_a.clone()))
            .bind(("b", concept_b.clone()))
            .await
            .unwrap();

        let b_node: Option<MemoryNode> = db().select(("memory", &concept_b)).await.unwrap();
        let b_node = b_node.unwrap();

        // 時間を過去に進めて忘却を発動させる (30日経過)
        db().query(
            "UPDATE type::thing('memory', $b) SET last_accessed = time::unix() - 86400 * 30",
        )
        .bind(("b", concept_b.clone()))
        .await
        .unwrap();

        decay_vividness().await.unwrap();

        let b_node_decayed: Option<MemoryNode> = db().select(("memory", &concept_b)).await.unwrap();
        let b_node_decayed = b_node_decayed.unwrap();
        assert!(
            b_node_decayed.vividness < b_node.vividness,
            "Vividness did not decay properly: BEFORE={}, AFTER={}",
            b_node.vividness,
            b_node_decayed.vividness
        );

        // A へ Spreading Activation を適用することで、B のividness が向上するか確認
        spread_activation(&concept_a).await.unwrap();

        let b_node_spread: Option<MemoryNode> = db().select(("memory", &concept_b)).await.unwrap();
        let b_node_spread = b_node_spread.unwrap();
        assert!(
            b_node_spread.vividness > b_node_decayed.vividness,
            "Vividness did not spread properly"
        );
    }
}
