use clap::{Parser,Subcommand};

use crate::commands::{Init, Select, Status, TemplateList, TemplateNew, TemplateDelete};
use crate::setup::Setup;

#[derive(Parser)]
#[command(name = "wtd",version,about="Want-Driven Development CLI")]
pub struct Cli {
	#[command(subcommand)]
	pub command: Commands
}

#[derive(Debug,Subcommand)]
pub enum Commands {
	/// 初期化と生成
	Init(Init),
	/// プロジェクト選択
	Select(Select),

	/// 状態確認
	Status(Status),

	/// テンプレート一覧を表示する
    TemplateList(TemplateList),

    /// 新しいテンプレートを作成する
    TemplateNew(TemplateNew),

    /// テンプレートを削除する
    TemplateDelete(TemplateDelete),
	/// 初期セットアップ
	Setup(Setup)
}