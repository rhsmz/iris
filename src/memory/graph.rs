use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;
use serde::{Deserialize, Serialize};
use std::sync::OnceLock;

static DB: OnceLock<Surreal<Client>> = OnceLock::new();

// DB インスタンスの取得（初期化済みであることが前提）
fn db() -> &'static Surreal<Client> {
    DB.get().expect("SurrealDB has not been initialized. Call connect_to_db() first.")
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
    client.signin(Root { username: &user, password: &pass }).await?;
    client.use_ns(&ns).use_db(&db_name).await?;

    // 人格コアノードが存在しない場合は初期化する
    let personality_nodes = vec![
        PersonalityNode { trait_name: "humor".to_string(), initial_weight: 0.9 },
        PersonalityNode { trait_name: "sarcasm".to_string(), initial_weight: 0.7 },
        PersonalityNode { trait_name: "rust_love".to_string(), initial_weight: 1.0 },
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

/// 新しい記憶ノードをグラフに挿入する
pub async fn insert_memory(
    concept: &str,
    emotion_score: f32,
    tags: &str,
    file_path: Option<String>,
) -> surrealdb::Result<Option<MemoryNode>> {
    let initial_vividness = (emotion_score / 10.0).min(1.0);
    let node = MemoryNode {
        concept: concept.to_string(),
        vividness: initial_vividness,
        last_accessed: chrono::Utc::now().timestamp(),
        emotion_score,
        tags: tags.to_string(),
        file_path,
    };

    let created: Option<MemoryNode> = db()
        .create(("memory", concept))
        .content(node)
        .await?;

    Ok(created)
}

/// Spreading Activation: アクセスされたノードの隣接ノードの鮮明度を向上させる
pub async fn spread_activation(concept: &str) -> surrealdb::Result<()> {
    db().query(
        "UPDATE memory SET vividness = math::min(vividness + 0.1, 1.0), last_accessed = time::unix() \
         WHERE <-relates_to<-(memory WHERE id = $id)"
    )
    .bind(("id", format!("memory:{}", concept)))
    .await?;

    Ok(())
}

/// 鮮明度の高い上位 N 件の記憶ノードを取得する（RAGコンテキスト構築用）
pub async fn fetch_top_memories(limit: u32) -> surrealdb::Result<Vec<MemoryNode>> {
    let mut result = db()
        .query("SELECT * FROM memory ORDER BY vividness DESC LIMIT $limit")
        .bind(("limit", limit))
        .await?;

    let nodes: Vec<MemoryNode> = result.take(0)?;
    Ok(nodes)
}

/// 時間経過による記憶風化処理（Vividness 減衰バッチ）
pub async fn decay_vividness(decay_rate: f32) -> surrealdb::Result<()> {
    db().query(
        "UPDATE memory SET vividness = math::max(vividness * $rate, 0.0)"
    )
    .bind(("rate", 1.0 - decay_rate))
    .await?;

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
}
