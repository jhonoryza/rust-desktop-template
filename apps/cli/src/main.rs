//! Headless driver: proves the engine works without the GUI.

use anyhow::bail;
use clap::{Parser, Subcommand};
use serde_json::json;
use template_core::Document;
use template_core::command::Registry;

#[derive(Parser)]
#[command(
    name = "template-cli",
    about = "Headless control for the template engine"
)]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    /// List all registered command ids (feature parity catalog).
    Commands,
    /// Execute a command by id with a JSON params string.
    Exec {
        id: String,
        #[arg(default_value = "{}")]
        params: String,
    },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let reg = Registry::new();
    let mut doc = Document::default();
    match cli.cmd {
        Cmd::Commands => {
            for id in reg.ids() {
                println!("{id}");
            }
        }
        Cmd::Exec { id, params } => {
            let v: serde_json::Value = serde_json::from_str(&params)?;
            if reg.execute(&mut doc, &id, &v).is_err() {
                bail!("command failed: {id}");
            }
            println!("{}", json!({"ok": true, "text": doc.text}));
        }
    }
    Ok(())
}
