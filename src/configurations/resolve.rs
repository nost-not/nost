use std::env;

use crate::configurations::get::get_value_from_config;

/// Resolve the base `not` directory for the current nost instance.
///
/// Resolution order (first match wins):
///   1. `NOT_PATH` environment variable — lets each instance (tests, CI, a
///      throwaway sandbox, a second checkout…) point at its own notes folder
///      without touching the shared `config.toml`.
///   2. `not_path` in `config.toml` — the persistent, per-machine default.
///
/// This is the single source of truth for "where do the notes live?", so
/// test runs never accidentally read or write the production notes folder:
/// just set `NOT_PATH=/tmp/nost-test` for that instance.
pub fn resolve_not_path() -> Result<String, Box<dyn std::error::Error>> {
    if let Ok(path) = env::var("NOT_PATH") {
        if !path.trim().is_empty() {
            return Ok(path);
        }
    }

    get_value_from_config("not_path")
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;

    #[test]
    #[serial]
    fn env_var_takes_precedence() {
        env::set_var("NOT_PATH", "/tmp/nost-test-instance");
        assert_eq!(resolve_not_path().unwrap(), "/tmp/nost-test-instance");
        env::remove_var("NOT_PATH");
    }
}
