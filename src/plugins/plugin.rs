/// Minimal plugin boundary for nost.
///
/// This is the seam that lets a feature area (like `work`) be developed,
/// tested, and eventually **extracted** without the core `main.rs` knowing
/// anything about its internals.
///
/// Today `main.rs` imports `start_work`, `end_work`, `work_stats` directly —
/// a hard compile-time coupling. With this trait, the core only knows about
/// the `Plugin` interface and a registry; each plugin declares which commands
/// it owns and how to run them.
pub trait Plugin {
    /// Short plugin name (for help/diagnostics), e.g. `"work"`.
    fn name(&self) -> &'static str;

    /// Returns true if this plugin handles the given command token
    /// (e.g. `"start-work"`, `"sw"`).
    fn handles(&self, command: &str) -> bool;

    /// Execute the command. `args` is the full process argv (args[1] is the
    /// command token). Returns `Ok(())` on success, or an error the core can
    /// surface uniformly.
    fn run(&self, command: &str, args: &[String]) -> Result<(), String>;
}

/// A tiny registry the core owns. To drop or extract a plugin, you add/remove
/// exactly one line here — `main.rs` routing stays untouched.
pub struct PluginRegistry {
    plugins: Vec<Box<dyn Plugin>>,
}

impl PluginRegistry {
    pub fn new() -> Self {
        PluginRegistry {
            plugins: Vec::new(),
        }
    }

    pub fn register(mut self, plugin: Box<dyn Plugin>) -> Self {
        self.plugins.push(plugin);
        self
    }

    /// Try to dispatch a command to a plugin. Returns:
    ///   - `Some(Ok/Err)` if a plugin claimed the command,
    ///   - `None` if no plugin handles it (core falls back to built-ins).
    pub fn dispatch(&self, command: &str, args: &[String]) -> Option<Result<(), String>> {
        for plugin in &self.plugins {
            if plugin.handles(command) {
                log::debug!(
                    "Command '{}' handled by plugin '{}'",
                    command,
                    plugin.name()
                );
                return Some(plugin.run(command, args));
            }
        }
        None
    }
}

impl Default for PluginRegistry {
    fn default() -> Self {
        Self::new()
    }
}
