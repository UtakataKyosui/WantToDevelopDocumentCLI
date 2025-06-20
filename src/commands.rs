use std::path::PathBuf;
use anyhow::Context;
use clap::Args;
use crate::config::WtdConfig;

pub trait Runnable {
    fn run(&self) -> anyhow::Result<()>;
}


// 初期化構造体
#[derive(Debug, Args)]
pub struct Init {
	/// プロジェクト名
	pub name: String,
	/// 出力先ディレクトリ（省略時はカレント）
	#[arg(short, long)]
	pub output: Option<String>,
}

// プロジェクト選択
#[derive(Debug, Args)]
pub struct Select {
	/// プロジェクト名
	pub name: String
}

#[derive(Debug, Args)]
pub struct Status;

// テンプレート一覧表示
#[derive(Debug, Args)]
pub struct TemplateList;

// 新しいテンプレート作成
#[derive(Debug, Args)]
pub struct TemplateNew {
	/// テンプレート名
	pub name: String,
}

// テンプレート削除
#[derive(Debug, Args)]
pub struct TemplateDelete {
	/// テンプレート名
	pub name: String,
}

impl Runnable for Init {
	fn run(&self) -> anyhow::Result<()> {
		let config = WtdConfig::load_config()?;

		let output_root = if let Some(cli_path) = &self.output {
			PathBuf::from(cli_path)
		} else if let Some(ref default_path) = config.default_output_dir {
			PathBuf::from(shellexpand::tilde(default_path).to_string())
		} else {
			std::env::current_dir().context("カレントディレクトリ取得失敗")?
		};


        println!("🚀 Init: 出力先 → {}", output_root.display());

        // 残りの処理
        let template_root = dirs::config_dir()
            .context("設定ディレクトリが見つかりません")?
            .join("wtd")
            .join("templates");

        let selected_path = crate::template::Template::select_template(&template_root)?;
        let template = crate::template::Template::load_template(&selected_path)?;

        crate::template::Template::create_project_files(
            &selected_path,
            &template,
            &self.name,
            &output_root,
        )?;

        Ok(())
	}
}

impl Runnable for Select {
	fn run(&self) -> anyhow::Result<()> {
		let mut cfg = WtdConfig::load_config()?;
		cfg.selected_project = Some(self.name.to_string());
		WtdConfig::save_config(&cfg)?;
		println!("✅ プロジェクト '{}' を選択しました", self.name);
		Ok(())
	}
}

impl Runnable for Status {
	fn run(&self) -> anyhow::Result<()> {
		let cfg = WtdConfig::load_config()?;
		if let Some(name) = cfg.selected_project {
			println!("📌 現在選択中のプロジェクト: {}", name);
		} else {
			println!("⚠️  プロジェクトが選択されていません。`wtd select` を使ってください。");
		}
		Ok(())
	}
}

impl Runnable for TemplateList {
	fn run(&self) -> anyhow::Result<()> {
		crate::template::Template::list()
	}
}

impl Runnable for TemplateNew {
	fn run(&self) -> anyhow::Result<()> {
		let template_dir = dirs::config_dir()
			.context("設定ディレクトリが見つかりません")?
			.join("wtd")
			.join("templates")
			.join(&self.name);

		if template_dir.exists() {
			anyhow::bail!("テンプレート '{}' は既に存在します", self.name);
		}

		std::fs::create_dir_all(&template_dir)?;
		
		// デフォルトのtemplate.yamlを作成
		let template_yaml = template_dir.join("template.yaml");
		let default_yaml = format!(r#"name: "{}"
description: "カスタムテンプレート"
want:
  filename: "{{{{slug}}}}.md"
  template: "want.md"
docs:
  - filename: "README.md"
    template: "docs/readme.md"
tags: ["custom"]
"#, self.name);
		std::fs::write(&template_yaml, default_yaml)?;

		// デフォルトのwant.mdを作成
		let want_md = template_dir.join("want.md");
		let default_want = r#"# {{title}}

- 作成日: {{date}}

## 概要

## 動機

## 想定機能

- [ ] 機能1
- [ ] 機能2
"#;
		std::fs::write(&want_md, default_want)?;

		// docs ディレクトリとREADME.mdを作成
		let docs_dir = template_dir.join("docs");
		std::fs::create_dir_all(&docs_dir)?;
		let readme_md = docs_dir.join("readme.md");
		let default_readme = r#"# 開発ドキュメント

## 要件

## 設計方針

## 今後の課題
"#;
		std::fs::write(&readme_md, default_readme)?;

		println!("✅ テンプレート '{}' を作成しました", self.name);
		println!("📁 {}", template_dir.display());
		Ok(())
	}
}

impl Runnable for TemplateDelete {
	fn run(&self) -> anyhow::Result<()> {
		let template_dir = dirs::config_dir()
			.context("設定ディレクトリが見つかりません")?
			.join("wtd")
			.join("templates")
			.join(&self.name);

		if !template_dir.exists() {
			anyhow::bail!("テンプレート '{}' が見つかりません", self.name);
		}

		// 確認プロンプト
		let confirmation = dialoguer::Confirm::new()
			.with_prompt(format!("テンプレート '{}' を削除しますか？", self.name))
			.default(false)
			.interact()?;

		if confirmation {
			std::fs::remove_dir_all(&template_dir)?;
			println!("✅ テンプレート '{}' を削除しました", self.name);
		} else {
			println!("❌ 削除をキャンセルしました");
		}
		Ok(())
	}
}