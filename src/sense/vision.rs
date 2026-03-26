use serde::{Deserialize, Serialize};
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::time::sleep;

// -----------------------------------------------
// Vision イベント定義
// -----------------------------------------------

/// Vision サブシステムが発火するイベント
#[derive(Debug, Clone, PartialEq)]
pub enum VisionEvent {
    /// ユーザー（主人）が検出された
    UserDetected {
        /// 検出された顔の数
        face_count: usize,
        /// 最も信頼度の高い顔領域
        primary_face: Option<FaceRegion>,
    },
    /// ユーザーがフレームから消えた
    UserLeft,
}

/// 検出された顔のバウンディングボックスと信頼度
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FaceRegion {
    /// 左上 X 座標（ピクセル）
    pub x: u32,
    /// 左上 Y 座標（ピクセル）
    pub y: u32,
    /// 幅（ピクセル）
    pub width: u32,
    /// 高さ（ピクセル）
    pub height: u32,
    /// 信頼度スコア (0.0 ~ 1.0)
    pub confidence: f32,
}

// -----------------------------------------------
// VisionEngine Trait（モック可能な抽象化）
// -----------------------------------------------

/// カメラキャプチャと顔検出を抽象化する Trait
/// モック実装と本番実装（OpenCV 等）を差し替え可能にする
#[async_trait::async_trait]
pub trait VisionEngine: Send + Sync {
    /// カメラからフレームをキャプチャする
    /// 戻り値は生のバイト列（JPEG/PNG 等のエンコード済みデータ）
    async fn capture_frame(&self) -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>>;

    /// フレームデータから顔を検出する
    fn detect_faces(&self, frame_data: &[u8]) -> Vec<FaceRegion>;
}

// -----------------------------------------------
// モック実装（開発・テスト用）
// -----------------------------------------------

/// テスト・開発用のモック Vision エンジン
/// `user_present` フラグで検出結果を制御可能
pub struct MockVisionEngine {
    /// true の場合、常に顔検出を返す
    pub user_present: bool,
}

impl MockVisionEngine {
    pub fn new(user_present: bool) -> Self {
        Self { user_present }
    }
}

#[async_trait::async_trait]
impl VisionEngine for MockVisionEngine {
    async fn capture_frame(&self) -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>> {
        // モックでは空のフレームを返す
        Ok(vec![0u8; 64])
    }

    fn detect_faces(&self, _frame_data: &[u8]) -> Vec<FaceRegion> {
        if self.user_present {
            vec![FaceRegion {
                x: 100,
                y: 80,
                width: 120,
                height: 150,
                confidence: 0.95,
            }]
        } else {
            vec![]
        }
    }
}

// -----------------------------------------------
// Vision ループ（非同期タスク）
// -----------------------------------------------

/// Vision サブシステムのメインループ
/// カメラからフレームを定期キャプチャし、顔検出結果をイベントとして送出する
///
/// # Arguments
/// * `engine` - VisionEngine の実装（モックまたは本番）
/// * `tx` - VisionEvent を送出するチャネル
/// * `interval_secs` - フレームキャプチャ間隔（秒）
pub async fn start_vision_loop(
    engine: Box<dyn VisionEngine>,
    tx: mpsc::Sender<VisionEvent>,
    interval_secs: u64,
) {
    println!("👁️  Vision サブシステムを起動しています...");

    let mut was_present = false;

    loop {
        sleep(Duration::from_secs(interval_secs)).await;

        // 1. フレームキャプチャ
        let frame = match engine.capture_frame().await {
            Ok(f) => f,
            Err(e) => {
                eprintln!("⚠️ フレームキャプチャエラー: {e}");
                continue;
            }
        };

        // 2. 顔検出
        let faces = engine.detect_faces(&frame);
        let is_present = !faces.is_empty();

        // 3. 状態変化時にイベントを発火（連続発火を防ぐ）
        if is_present && !was_present {
            let primary = faces.iter().max_by(|a, b| {
                a.confidence
                    .partial_cmp(&b.confidence)
                    .unwrap_or(std::cmp::Ordering::Equal)
            });
            let event = VisionEvent::UserDetected {
                face_count: faces.len(),
                primary_face: primary.cloned(),
            };
            println!("👤 主人を検出しました（顔数: {}）", faces.len());
            if tx.send(event).await.is_err() {
                eprintln!("⚠️ Vision イベントチャネルが閉じています。ループを終了します。");
                break;
            }
        } else if !is_present && was_present {
            println!("👻 主人がフレームから消えました");
            if tx.send(VisionEvent::UserLeft).await.is_err() {
                break;
            }
        }

        was_present = is_present;
    }
}

// -----------------------------------------------
// テスト
// -----------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn モックエンジンがプレゼンスイベントを正しく発火する() {
        let engine = Box::new(MockVisionEngine::new(true));
        let (tx, mut rx) = mpsc::channel(10);

        // ループを短いインターバルで起動し、1イベント受信後にキャンセルする
        let handle = tokio::spawn(async move {
            start_vision_loop(engine, tx, 0).await;
        });

        let event = tokio::time::timeout(Duration::from_secs(2), rx.recv())
            .await
            .expect("タイムアウト: イベントが発火しませんでした")
            .expect("チャネルが閉じています");

        match event {
            VisionEvent::UserDetected {
                face_count,
                primary_face,
            } => {
                assert_eq!(face_count, 1);
                assert!(primary_face.is_some());
                let face = primary_face.unwrap();
                assert!(face.confidence > 0.9);
            }
            _ => panic!("期待されるイベントは UserDetected でした"),
        }

        handle.abort();
    }

    #[tokio::test]
    async fn 顔未検出時はユーザー検出イベントが発火しない() {
        let engine = Box::new(MockVisionEngine::new(false));
        let (tx, mut rx) = mpsc::channel(10);

        let handle = tokio::spawn(async move {
            start_vision_loop(engine, tx, 0).await;
        });

        // 短い待機時間で受信を試みる（イベントが来ないことを確認）
        let result = tokio::time::timeout(Duration::from_millis(500), rx.recv()).await;
        assert!(
            result.is_err(),
            "顔未検出時にイベントが発火すべきではない"
        );

        handle.abort();
    }

    #[test]
    fn フェイスリージョンのシリアライズが正しく行われる() {
        let face = FaceRegion {
            x: 10,
            y: 20,
            width: 100,
            height: 120,
            confidence: 0.88,
        };
        let json = serde_json::to_string(&face).unwrap();
        assert!(json.contains("\"confidence\":0.88"));

        let deserialized: FaceRegion = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.width, 100);
    }
}
