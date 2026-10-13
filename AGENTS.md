# Template — rules for agents and contributors

Rust-native desktop app template (egui + eframe/wgpu), inspired by storytold's
craft-suite conventions. This file holds the rules that must never be broken.

## 0. Never crash

A crash loses the user's work. **This outranks feature work.**

1. Fail with `Result<T, E>`; propagate with `?`; add context instead of discarding.
   In the UI, report the error in the status line and carry on.
2. No `unwrap()`, `expect()`, `panic!`, `unreachable!`, `todo!`, `unimplemented!`
   outside tests. Workspace lints deny them (`[workspace.lints.clippy]`).
3. No `unsafe` (`unsafe_code = "forbid"` is inherited by default; if ever needed,
   isolate it in a `platform` crate with SAFETY comments and a safe public API).
4. Every input-derived number is hostile: index with `get()`, slice strings only
   at char boundaries, use checked/saturating arithmetic, cap allocations.
5. Every crash fix lands with a regression test that panicked before the fix.
   Keep `crates/core/src/command.rs::tests::hostile_params_never_panic` green and
   extend it for every new command.

## 1. Architecture

| Layer | Crate | Job |
|---|---|---|
| engine | `crates/core` | domain model + command registry (no UI deps) |
| ui | `crates/ui-egui` | egui front end — swappable, thin, renders state |
| apps | `apps/app`, `apps/cli` | desktop binary, headless CLI |

- **Everything is a command.** User-visible behaviour = a `CommandSpec` in
  `crates/core/src/command.rs` (id, label, run) + tests. UI buttons, the CLI and
  headless tests all dispatch by id — features are verifiable without the GUI.
- **Layering:** nothing in `crates/core` may depend on egui/eframe/winit.
- **Rust only.** No handwritten JS.

## 2. Quality gates (before every commit)

```sh
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo run -p template-cli -- commands        # parity catalog
```

## 3. UI verification (the craft-suite way)

- UI elements carry automation ids (`id_salt(...)`).
- Headless UI tests live in `crates/ui-egui/tests/scripted.rs` using `egui_kittest`:
  drive the app by id, assert state, and `snapshot(...)` for golden-image diffs.
- For visual work, look at the result: run the app, screenshot, compare.
- Parallel agents: use a separate `CARGO_TARGET_DIR` per agent.

## 4. Assets

Every committed image/font/icon needs a license and an attribution row in
`ATTRIBUTION.md`. Prefer drawing icons in code (`ui-egui/src/icons.rs`).
Never commit third-party proprietary artwork, fonts or screenshots.
