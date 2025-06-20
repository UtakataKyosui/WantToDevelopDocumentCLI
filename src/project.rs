use crate::config::WtdConfig;

pub fn select_project(project_name: &str) -> anyhow::Result<()> {
    let mut config = WtdConfig::load_config()?;
    config.selected_project = Some(project_name.to_string());
    WtdConfig::save_config(&config)?;
    println!("✅ プロジェクト '{}' を選択しました。", project_name);
    Ok(())
}

pub fn status() -> anyhow::Result<()> {
	let config = WtdConfig::load_config().expect("Load Config Error");
	if let Some(name) = config.selected_project {
        println!("📌 現在選択中のプロジェクト: {}", name);
    } else {
        println!("⚠️  選択中のプロジェクトはありません。`wtd select <name>` を使用してください。");
    }
    Ok(())

}
