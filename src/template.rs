use std::fs;
use std::path::{Path, PathBuf};

use dialoguer::Select;
use serde::Deserialize;
use tera::{Context, Tera};
use anyhow::Context as AnyhowContext;

#[derive(Debug,Deserialize)]
pub struct Template {
	pub name: String,
	pub description: Option<String>,
	pub want: WantEntry,
	pub docs: Vec<DocEntry>,
	pub tags: Option<Vec<String>>
}

#[derive(Debug, Deserialize)]
pub struct WantEntry {
    pub filename: String,
    pub template: String,
}

#[derive(Debug, Deserialize)]
pub struct DocEntry {
    pub filename: String,
    pub template: String,
}

impl Template {
	pub fn load_template(path: &Path) -> anyhow::Result<Template>{ 
		let yaml_path = path.join("template.yaml");
		let yaml_content = std::fs::read_to_string(&yaml_path)
			.with_context(|| format!("template.yaml の読み込みに失敗しました: {}", yaml_path.display()))?;
		let template: Template = serde_yaml::from_str(&yaml_content)
			.with_context(|| format!("template.yaml のパースに失敗しました: {}", yaml_path.display()))?;
		Ok(template)
	}

	pub fn render_template(template_path: &Path,context: &Context) -> anyhow::Result<String>{
		let content = std::fs::read_to_string(template_path).expect("Read Error");
		let result = Tera::one_off(&content,&context, false)
			.expect("one_off Error");
		Ok(result)
	}
	pub fn list() -> anyhow::Result<()> {
		let template_dir = dirs::config_dir()
			.ok_or_else(|| anyhow::anyhow!("設定ディレクトリが見つかりません"))?
			.join("wtd")
			.join("templates");

		if !template_dir.exists() {
			println!("⚠️ テンプレートフォルダが存在しません: {}", template_dir.display());
			return Ok(());
		}

		println!("📦 使用可能なテンプレート一覧:");
		for entry in fs::read_dir(template_dir)? {
			let entry = entry?;
			let path = entry.path();
			if path.is_dir() {
				let yaml = path.join("template.yaml");
				if yaml.exists() {
					let content = fs::read_to_string(yaml)?;
					let meta: Template = serde_yaml::from_str(&content)?;
					println!("• {} - {}", meta.name, path.file_name().unwrap().to_string_lossy());
				}
			}
		}

		Ok(())
	}

	pub fn select_template(template_dir: &Path) -> anyhow::Result<PathBuf> {
		if !template_dir.exists() {
		anyhow::bail!(
				"テンプレートディレクトリが見つかりません: {}\n
				次のコマンドでテンプレートを作成してください:\n\n wtd template new <name>",
				template_dir.display()
			);
		}
		let mut entries = vec![];

		for entry in std::fs::read_dir(template_dir)
			.with_context(|| format!("テンプレートディレクトリの読み込みに失敗しました: {}", template_dir.display()))?
		{
			let entry = entry?;
			let path = entry.path();

			if path.is_dir() {
				let yaml_path = path.join("template.yaml");
				if yaml_path.exists() {
					let content = std::fs::read_to_string(&yaml_path)
						.with_context(|| format!("template.yaml の読み込み失敗: {}", yaml_path.display()))?;
					let meta: Template = serde_yaml::from_str(&content)
						.with_context(|| format!("template.yaml のパース失敗: {}", yaml_path.display()))?;
					entries.push((meta.name, path.clone()));
				}
			}
		}

		if entries.is_empty() {
			anyhow::bail!(
				"テンプレートが見つかりません。\n\
				`wtd template new <name>` で作成してください。"
			);
		}

		let items: Vec<String> = entries.iter().map(|(name, _)| name.clone()).collect();
		let selection = Select::new()
			.with_prompt("使用するテンプレートを選択してください")
			.items(&items)
			.default(0)
			.interact()?;

		let (_name, selected_path) = &entries[selection];
		Ok(selected_path.clone())
	}

	pub fn create_project_files(
		template_path: &Path,
		template: &Template,
		project_name: &str,
		output_root: &Path,
	) -> anyhow::Result<()> {
		let slug = slug::slugify(project_name);
		let date = chrono::Local::now().format("%Y-%m-%d").to_string();

		let mut ctx = Context::new();
		ctx.insert("project", project_name);
		ctx.insert("slug", &slug);
		ctx.insert("title", project_name);
		ctx.insert("date", &date);

		let base_output_dir = output_root.join("WTD").join(project_name);

		// Wantファイル生成
		let want_template_path = template_path.join(&template.want.template);
		let want_content = Self::render_template(&want_template_path, &ctx)?;
		let want_output_path = base_output_dir
			.join("Want")
			.join(template.want.filename.replace("{{slug}}", &slug));
		fs::create_dir_all(want_output_path.parent().unwrap())?;
		fs::write(&want_output_path, want_content)?;

		// Docs生成
		for doc in &template.docs {
			let doc_template_path = template_path.join(&doc.template);
			let doc_content = Self::render_template(&doc_template_path, &ctx)?;
			let doc_output_path = base_output_dir.join("Develop-Docs").join(&doc.filename);
			fs::create_dir_all(doc_output_path.parent().unwrap())?;
			fs::write(&doc_output_path, doc_content)?;
		}

		println!("✅ プロジェクト '{}' を生成しました。", project_name);
		println!("📁 {}", base_output_dir.display());

		Ok(())
	}

}

