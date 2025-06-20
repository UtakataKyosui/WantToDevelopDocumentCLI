use anyhow::Ok;
use clap::Parser;

use crate::{commands::Runnable, parser::{Cli, Commands}};

mod template;
mod create;
mod config;
mod project;
mod parser;
mod commands;
mod setup;

fn main() -> anyhow::Result<()> {
    if let Err(err) = cli_process() {
        eprintln!("❌ エラー: {}", err);
        if let bt = err.backtrace().status() {
            eprintln!("🔍 バックトレース: {:?}", bt);
        }
        std::process::exit(1);
    }
    Ok(())
}

fn cli_process() -> anyhow::Result<()>{
	let cli = Cli::parse();

	match cli.command {
		Commands::Init(init) => {
			init.run()
				.expect("Execute Error");
		},
		Commands::Select(select) => {
			select.run()
				.expect("Execute Error");
		},
		Commands::Status(status) => {
			status.run().expect("Execute Error");
		},
		Commands::Setup(setup) => setup.run()?,
		Commands::TemplateList(template_list) => {
			template_list.run().expect("Execute Error");
		},
		Commands::TemplateNew(template_new) => {
			template_new.run().expect("Execute Error");
		},
		Commands::TemplateDelete(template_delete) => {
			template_delete.run().expect("Execute Error");
		},
	}
	Ok(())
}