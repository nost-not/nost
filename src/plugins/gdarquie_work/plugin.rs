use crate::plugins::gdarquie_work::commands::end_work::end_work;
use crate::plugins::gdarquie_work::commands::start_work::start_work;
use crate::plugins::gdarquie_work::commands::work_stats::work_stats;
use crate::plugins::plugin::Plugin;

/// The `work` feature packaged as a self-contained plugin.
///
/// It owns its command tokens and their aliases. The core `main.rs` never
/// names `start_work`/`end_work`/`work_stats` directly anymore — it only talks
/// to the `Plugin` trait. That is the seam that would let this whole
/// `gdarquie_work` module move into its own crate later.
pub struct WorkPlugin;

impl Plugin for WorkPlugin {
    fn name(&self) -> &'static str {
        "work"
    }

    fn handles(&self, command: &str) -> bool {
        matches!(
            command,
            "start-work" | "sw" | "end-work" | "ew" | "work-stats" | "ws"
        )
    }

    fn run(&self, command: &str, args: &[String]) -> Result<(), String> {
        match command {
            "start-work" | "sw" => start_work(args.to_vec()),
            "end-work" | "ew" => end_work(),
            "work-stats" | "ws" => work_stats(args.to_vec()),
            other => return Err(format!("WorkPlugin cannot handle '{}'", other)),
        }
        Ok(())
    }
}
