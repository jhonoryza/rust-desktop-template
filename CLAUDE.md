# Working instructions

Start every session by reading `AGENTS.md` (hard rules) and `ROADMAP.md`.

1. Pick the next task from `ROADMAP.md`; keep it small and commit per arc.
2. Follow the layering table in `AGENTS.md` §1; add features as commands first,
   then wire the UI button to the command id.
3. Run the quality gates in `AGENTS.md` §2 before every commit.
4. UI changes: add an automation id, extend `crates/ui-egui/tests/scripted.rs`,
   and snapshot when the visuals matter.
