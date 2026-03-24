use chrono::Local;
use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use uuid::Uuid;

/// 環境変数からMEMORIES_PATHを取得し、絶対パスを返す
pub fn get_memories_base_path() -> PathBuf {
    let path_str = env::var("MEMORIES_PATH").unwrap_or_else(|_| "/var/iris/memories".to_string());
    PathBuf::from(path_str)
}

/// 現在日時に基づくディレクトリパス（YYYY/MM/DD）を生成し、ディレクトリが存在しなければ作成する
pub fn create_daily_directory() -> io::Result<PathBuf> {
    let base = get_memories_base_path();
    let now = Local::now();
    let daily_path = base.join(now.format("%Y/%m/%d").to_string());

    if !daily_path.exists() {
        fs::create_dir_all(&daily_path)?;
    }

    Ok(daily_path)
}

/// 指定したMarkdownのテキスト内容をファイルに保存し、その絶対パスを返す
pub fn save_markdown(content: &str) -> io::Result<String> {
    let dir = create_daily_directory()?;
    let file_id = Uuid::new_v4().to_string();
    let file_path = dir.join(format!("{}.md", file_id));

    fs::write(&file_path, content)?;

    // SurrealDBに保存しやすくするためStringとして返す
    Ok(file_path.to_string_lossy().into_owned())
}

/// 指定したパスのMarkdownファイルを読み込んでテキストを返す
pub fn load_markdown(path: impl AsRef<Path>) -> io::Result<String> {
    fs::read_to_string(path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use tempfile::tempdir;

    #[test]
    fn test_cms_lifecycle() {
        // テスト用の一時ディレクトリを作成
        let temp_dir = tempdir().expect("Failed to create temp dir");
        let temp_path_str = temp_dir.path().to_str().unwrap().to_string();

        // 環境変数 MEMORIES_PATH を一時的に上書き
        // (注意: cargo test はマルチスレッドで動くため環境変数操作は競合の可能性あり)
        env::set_var("MEMORIES_PATH", &temp_path_str);

        // 1. パス解決テスト
        let base_path = get_memories_base_path();
        assert_eq!(base_path.to_str().unwrap(), temp_path_str);

        // 2. ディレクトリ生成テスト
        let daily_dir = create_daily_directory().expect("Failed to create daily directory");
        assert!(daily_dir.exists());
        let now = Local::now();
        let expected_suffix = now.format("%Y/%m/%d").to_string();
        // Windows環境やLinux環境でもパスセパレータの違いを吸収するため変換してアサート
        let expected_path_part: PathBuf = expected_suffix.split('/').collect();
        assert!(daily_dir.ends_with(expected_path_part));

        // 3. Markdown保存テスト
        let content = "# CMS Test\nThis is a unit test for episode memory CMS.";
        let saved_path_str = save_markdown(content).expect("Failed to save markdown");
        let saved_path = PathBuf::from(&saved_path_str);
        assert!(saved_path.exists());
        assert_eq!(saved_path.extension().unwrap().to_str().unwrap(), "md");

        // 4. Markdown読み込みテスト
        let loaded_content = load_markdown(&saved_path).expect("Failed to load markdown");
        assert_eq!(loaded_content, content);
    }
}
