use std::fs;
use anyhow::{Context,Result};
use clap::Args;
use crate::commands::Runnable;

#[derive(Debug, Args)]
pub struct Setup;

impl Runnable for Setup {
	fn run(&self) -> Result<()> {
        let config_dir = dirs::config_dir()
            .context("ユーザー設定ディレクトリが見つかりません")?
            .join("wtd")
            .join("templates")
            .join("default");

        let docs_dir = config_dir.join("docs");
        fs::create_dir_all(&docs_dir)
            .with_context(|| format!("ディレクトリ作成失敗: {}", docs_dir.display()))?;

        // template.yaml
        let template_yaml = config_dir.join("template.yaml");
        if !template_yaml.exists() {
            fs::write(&template_yaml, DEFAULT_TEMPLATE_YAML)?;
        }

        // want.md
        let want_md = config_dir.join("want.md");
        if !want_md.exists() {
            fs::write(&want_md, DEFAULT_WANT_MD)?;
        }

        // docs/readme.md
        let docs_readme = docs_dir.join("readme.md");
        if !docs_readme.exists() {
            fs::write(&docs_readme, DEFAULT_DOC_README)?;
        }

        println!("✅ セットアップ完了！");
        println!("📁 テンプレート格納先: {}", config_dir.display());

        Ok(())
    }
}


// ↓ デフォルトテンプレートの中身（定数）
const DEFAULT_TEMPLATE_YAML: &str = r#"---
name: default
description: デフォルトのテンプレート
want:
  filename: "{{slug}}.md"
  template: "want.md"
docs:
  - filename: "README.md"
    template: "docs/readme.md"
tags: ["default"]
"#;

const DEFAULT_WANT_MD: &str = r#"# {{title}}

- 作成日: {{date}}

## 概要

このツールは、こういうことをしたい！を実現するものです。

## 動機

## 想定機能

- [ ] 〇〇を入力として受け取る
- [ ] △△を処理して出力する
"#;

const DEFAULT_DOC_README: &str = r#"# 開発ドキュメント

## 要件

## 設計方針

## 今後の課題
"#;