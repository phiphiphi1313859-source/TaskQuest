mod character;
mod progression;
mod storage;
mod taskwarrior;
mod display;
mod hooks;
mod achievements;
mod shop;
mod sync;

use anyhow::Result;
use clap::Parser;
use display::CLI;

fn main() -> Result<()> {
    // Check if being run as a hook
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 0 {
        let program_name = std::path::Path::new(&args[0])
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("");

        match program_name {
            "on-add-taskquest" => return hooks::on_add_hook(),
            "on-modify-taskquest" => return hooks::on_modify_hook(),
            "on-exit-taskquest" => return hooks::on_exit_hook(),
            _ => {}
        }
    }

    // Run CLI
    let cli = CLI::parse();
    cli.run()
}
