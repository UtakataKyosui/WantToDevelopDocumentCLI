use std::{fs, path::PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WtdConfig {
    /// 現在選択中のプロジェクト（セッション管理用）
    pub selected_project: Option<String>,

    /// デフォルト出力先（例: "~/Vault/WTD"）
    pub default_output_dir: Option<String>,

    /// デフォルトで使用するテンプレート名
    pub default_template: Option<String>,

    /// 作成者名（WantやDocsに埋め込む場合に使用）
    pub author: Option<String>,
}

impl WtdConfig {

	pub fn config_path() -> anyhow::Result<PathBuf> {
		let path = dirs::config_dir()
			.ok_or_else(|| anyhow::anyhow!("設定ディレクトリが見つかりません"))?
			.join("wtd")
			.join("config.json");
		Ok(path)
	}

	/// 設定ファイルを読み込む
    pub fn load_config() -> anyhow::Result<Self> {
        let path = Self::config_path()?;
        if !path.exists() {
            return Ok(Self::default());
        }
        let content = fs::read_to_string(&path)?;
        let config: Self = serde_json::from_str(&content)?;
        Ok(config)
    }

	/// 設定ファイルを保存する
    pub fn save_config(config: &Self) -> anyhow::Result<()> {
        let path = Self::config_path()?;
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let content = serde_json::to_string_pretty(config)?;
        fs::write(path, content)?;
        Ok(())
    }

    /// 任意のキーだけ変更して保存（例: selected_projectのみ変更）
    pub fn update<F>(update_fn: F) -> anyhow::Result<()>
    where
        F: FnOnce(&mut Self),
    {
        let mut config = Self::load_config()?;
        update_fn(&mut config);
        Self::save_config(&config)
    }

}