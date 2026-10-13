# rust-desktop-template

A starter repo for Rust-native desktop apps — **egui + eframe (wgpu)** — with
the agent-friendly conventions of [storytold's craft suite](https://github.com/storytold)
(GridCraft/WordCraft/FilmCraft): never-crash rules, command-first architecture,
headless UI verification, and CI gates.

## Layout

- `crates/core` — engine: domain model + command registry (no UI deps)
- `crates/ui-egui` — egui front end (swappable)
- `apps/app` — desktop binary
- `apps/cli` — headless CLI (`commands`, `exec`)
- `xtask` — `cargo xtask ci` (fmt + clippy + tests + layer check)
- `AGENTS.md` / `CLAUDE.md` — rules for humans and AI agents

## Quick start

```sh
cargo run -p template-app          # desktop app
cargo run -p template-cli -- commands
cargo run -p template-cli -- exec text.insert '{"text":"Hello"}'
cargo xtask ci
```

## Making it yours

1. Rename `template-*` crates and ids.
2. Replace `Document` with your model; add `CommandSpec`s for every feature.
3. Build your UI in `crates/ui-egui` — keep it thin; everything runs through commands.
