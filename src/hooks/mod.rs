use anyhow::Result;
use std::io::{self, Read};
use crate::taskwarrior::{TaskData, TaskwarriorIntegration};

pub fn on_add_hook() -> Result<()> {
    // Read task from stdin
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    // Set default challenge if not set
    let mut task_data: serde_json::Value = serde_json::from_str(&input)?;
    if task_data.get("challenge").is_none() {
        task_data["challenge"] = serde_json::json!(5);
    }

    // Output modified task
    println!("{}", serde_json::to_string(&task_data)?);

    Ok(())
}

pub fn on_modify_hook() -> Result<()> {
    // Read original and modified task from stdin
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let lines: Vec<&str> = input.lines().collect();
    if lines.len() < 2 {
        eprintln!("Error: Expected 2 lines of input (original and modified task)");
        return Ok(());
    }

    let _original: TaskData = serde_json::from_str(lines[0])?;
    let modified: TaskData = serde_json::from_str(lines[1])?;

    // Check if task was completed
    if modified.is_completed() {
        if let Err(e) = TaskwarriorIntegration::process_completion(&modified) {
            eprintln!("Warning: Failed to process task completion: {}", e);
        }
    }

    // Output modified task unchanged
    println!("{}", lines[1]);

    Ok(())
}

pub fn on_exit_hook() -> Result<()> {
    // This hook runs after task command completes
    // Can be used for git auto-commit, notifications, etc.
    Ok(())
}
