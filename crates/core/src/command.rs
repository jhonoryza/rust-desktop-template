//! Command registry — "everything is a command".
//!
//! Every user-visible behavior is a `CommandSpec` with a stable id. The GUI,
//! CLI, control channel and tests all dispatch by id, so features can be
//! verified headlessly without clicking anything.

use crate::{Document, Result};
use serde_json::Value;
use std::collections::BTreeMap;
use std::sync::Arc;

pub type CommandFn = Arc<dyn Fn(&mut Document, &Value) -> Result<()> + Send + Sync>;

#[derive(Clone)]
pub struct CommandSpec {
    pub id: &'static str,
    pub label: &'static str,
    pub run: CommandFn,
}

/// Registry of all commands; the single source of truth for feature parity.
#[derive(Default)]
pub struct Registry {
    commands: BTreeMap<&'static str, CommandSpec>,
}

impl Registry {
    #[must_use]
    pub fn new() -> Self {
        let mut r = Self::default();
        r.register(CommandSpec {
            id: "text.insert",
            label: "Insert Text",
            run: Arc::new(|doc: &mut Document, params: &Value| {
                let text = params.get("text").and_then(Value::as_str).ok_or_else(|| {
                    crate::Error::BadParams {
                        command: "text.insert".into(),
                        message: "missing string field `text`".into(),
                    }
                })?;
                let at = params
                    .get("at")
                    .and_then(Value::as_u64)
                    .map_or(doc.text.len(), |v| v as usize);
                doc.insert_text(at, text);
                Ok(())
            }),
        });
        r.register(CommandSpec {
            id: "text.clear",
            label: "Clear Text",
            run: Arc::new(|doc: &mut Document, _params: &Value| {
                doc.text.clear();
                Ok(())
            }),
        });
        r
    }

    fn register(&mut self, spec: CommandSpec) {
        self.commands.insert(spec.id, spec);
    }

    /// Execute a command by id with JSON params.
    pub fn execute(&self, doc: &mut Document, id: &str, params: &Value) -> Result<()> {
        let spec = self
            .commands
            .get(id)
            .ok_or_else(|| crate::Error::UnknownCommand(id.to_string()))?;
        (spec.run)(doc, params)
    }

    #[must_use]
    pub fn ids(&self) -> Vec<&'static str> {
        self.commands.keys().copied().collect()
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]
    use super::*;
    use serde_json::json;

    #[test]
    fn insert_and_clear_via_registry() {
        let reg = Registry::new();
        let mut doc = Document::default();
        reg.execute(&mut doc, "text.insert", &json!({"text": "hello"}))
            .unwrap();
        reg.execute(&mut doc, "text.insert", &json!({"text": " world", "at": 5}))
            .unwrap();
        assert_eq!(doc.text, "hello world");
        reg.execute(&mut doc, "text.clear", &json!({})).unwrap();
        assert!(doc.text.is_empty());
    }

    #[test]
    fn hostile_params_never_panic() {
        let reg = Registry::new();
        let mut doc = Document::default();
        for junk in [
            json!({}),
            json!({"text": null}),
            json!({"text": 42}),
            json!({"text": "x", "at": -1}),
            json!({"text": "x", "at": u64::MAX}),
        ] {
            let _ = reg.execute(&mut doc, "text.insert", &junk);
        }
        assert!(reg.execute(&mut doc, "nope.nope", &json!({})).is_err());
    }
}
