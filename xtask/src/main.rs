//! xtask — `cargo xtask ci` runs every quality gate, plus a layer check that
//! keeps the engine free of UI dependencies.

use anyhow::{Context as _, bail};
use std::process::Command;

fn main() -> anyhow::Result<()> {
    let task = std::env::args().nth(1).unwrap_or_else(|| "ci".into());
    match task.as_str() {
        "ci" => ci(),
        "layers" => layers(),
        other => bail!("unknown task: {other} (try: ci, layers)"),
    }
}

fn run(desc: &str, mut cmd: Command) -> anyhow::Result<()> {
    println!("==> {desc}");
    let status = cmd.status().context("spawn")?;
    if !status.success() {
        bail!("{desc} failed");
    }
    Ok(())
}

fn ci() -> anyhow::Result<()> {
    let mut c = Command::new("cargo");
    c.args(["fmt", "--all", "--check"]);
    run("fmt", c)?;

    let mut c = Command::new("cargo");
    c.args([
        "clippy",
        "--workspace",
        "--all-targets",
        "--",
        "-D",
        "warnings",
    ]);
    run("clippy", c)?;

    let mut c = Command::new("cargo");
    c.args(["test", "--workspace"]);
    run("tests", c)?;

    layers()
}

fn layers() -> anyhow::Result<()> {
    println!("==> layers (engine must not depend on the UI)");
    let manifest = std::fs::read_to_string("crates/core/Cargo.toml")?;
    for banned in ["egui", "eframe", "winit", "wgpu", "template-ui-egui"] {
        if manifest
            .lines()
            .any(|l| !l.trim_start().starts_with('#') && l.contains(banned))
        {
            bail!("crates/core must not depend on `{banned}` — the engine stays UI-free");
        }
    }
    println!("    layers ok");
    Ok(())
}
