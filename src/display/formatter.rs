use colored::Colorize;
use crate::character::Character;

pub struct Formatter;

impl Formatter {
    pub fn print_status(character: &Character) {
        // Get avatar for this class and level
        let avatar = character.class.get_avatar(character.level);

        println!();
        println!("{}", "╔════════════════════════════════════════════════════════════════╗".cyan());
        println!("{}", format!("║ {} - {} {}, Level {} ║",
            character.name,
            character.class,
            if let Some(title) = &character.active_title {
                format!("\"{}\"", title)
            } else {
                String::new()
            },
            character.level
        ).cyan().bold());
        println!("{}", "╠════════════════════════════════════════════════════════════════╣".cyan());

        // Display avatar
        for line in avatar.lines() {
            if !line.trim().is_empty() {
                println!("║ {:^62} ║", line);
            }
        }
        println!("{}", "╠════════════════════════════════════════════════════════════════╣".cyan());

        // Stats column
        println!("║ {:12} {:>6}  │  Progression:                           ║",
            "Stats:".bold(), "");
        println!("║   STR: {:>3}        │  Level: {}                               ║",
            character.stats.strength.floor() as u16, character.level);
        println!("║   DEX: {:>3}        │  XP: {}/{} ({:.1}%)              ║",
            character.stats.dexterity.floor() as u16,
            character.total_xp,
            character.total_xp + character.xp_to_next_level(),
            character.xp_progress_percent()
        );
        println!("║   CON: {:>3}        │  Gold: {} 💰                            ║",
            character.stats.constitution.floor() as u16, character.gold);
        println!("║   INT: {:>3}        │                                         ║",
            character.stats.intelligence.floor() as u16
        );
        println!("║   WIS: {:>3}        │  Next Level: {} XP                      ║",
            character.stats.wisdom.floor() as u16, character.xp_to_next_level());
        println!("║   CHA: {:>3}        │  Tasks Completed: {} ✓                  ║",
            character.stats.charisma.floor() as u16, character.tasks_completed);

        println!("{}", "╠════════════════════════════════════════════════════════════════╣".cyan());

        // Progress bar
        let bar_width = 50;
        let filled = ((character.xp_progress_percent() / 100.0) * bar_width as f64) as usize;
        let bar: String = "█".repeat(filled) + &"░".repeat(bar_width - filled);
        println!("║ XP Progress: [{}] ║", bar.green());

        println!("{}", "╚════════════════════════════════════════════════════════════════╝".cyan());
        println!();
    }

    pub fn print_stats(character: &Character) {
        // Helper function to create a stat bar
        fn stat_bar(value: u16, max: u16) -> String {
            let bar_width = 20;
            let filled = ((value as f64 / max as f64) * bar_width as f64).min(bar_width as f64) as usize;
            let bar = "█".repeat(filled) + &"░".repeat(bar_width - filled);

            // Color based on value
            if value >= max {
                bar.green().to_string()
            } else if value >= max / 2 {
                bar.yellow().to_string()
            } else {
                bar.red().to_string()
            }
        }

        // Find the highest stat for relative comparison (convert f64 to u16)
        let stats_array = [
            character.stats.strength.floor() as u16,
            character.stats.dexterity.floor() as u16,
            character.stats.constitution.floor() as u16,
            character.stats.intelligence.floor() as u16,
            character.stats.wisdom.floor() as u16,
            character.stats.charisma.floor() as u16,
        ];
        let max_stat = stats_array.iter().max().unwrap_or(&10).max(&100u16); // At least 100 for scaling

        let str_val = character.stats.strength.floor() as u16;
        let dex_val = character.stats.dexterity.floor() as u16;
        let con_val = character.stats.constitution.floor() as u16;
        let int_val = character.stats.intelligence.floor() as u16;
        let wis_val = character.stats.wisdom.floor() as u16;
        let cha_val = character.stats.charisma.floor() as u16;

        println!();
        println!("{}", "╔══════════════════════════════════════════════════════╗".yellow());
        println!("{}", "║              DETAILED STATISTICS                     ║".yellow().bold());
        println!("{}", "╠══════════════════════════════════════════════════════╣".yellow());
        println!("║ Character: {}                                  ║", character.name);
        println!("║ Class: {}                                      ║", character.class);
        println!("║ Level: {}                                           ║", character.level);
        println!("{}", "╠══════════════════════════════════════════════════════╣".yellow());
        println!("║ ABILITY SCORES:                                      ║");
        println!("║   Strength:     {:>3} [{}] ║", str_val, stat_bar(str_val, *max_stat));
        println!("║   Dexterity:    {:>3} [{}] ║", dex_val, stat_bar(dex_val, *max_stat));
        println!("║   Constitution: {:>3} [{}] ║", con_val, stat_bar(con_val, *max_stat));
        println!("║   Intelligence: {:>3} [{}] ║", int_val, stat_bar(int_val, *max_stat));
        println!("║   Wisdom:       {:>3} [{}] ║", wis_val, stat_bar(wis_val, *max_stat));
        println!("║   Charisma:     {:>3} [{}] ║", cha_val, stat_bar(cha_val, *max_stat));
        println!("{}", "╠══════════════════════════════════════════════════════╣".yellow());
        println!("║ PROGRESSION:                                         ║");
        println!("║   Total XP:     {:>6}                                ║", character.total_xp);
        println!("║   Current XP:   {:>6}                                ║",
            character.total_xp - crate::character::LevelSystem::xp_for_level(character.level));
        println!("║   Next Level:   {:>6} XP                             ║", character.xp_to_next_level());

        // XP Progress bar
        let xp_bar_width = 30;
        let xp_percent = character.xp_progress_percent() / 100.0;
        let xp_filled = (xp_percent * xp_bar_width as f64) as usize;
        let xp_bar: String = "█".repeat(xp_filled) + &"░".repeat(xp_bar_width - xp_filled);
        println!("║   XP Progress:  [{}] {:.1}% ║", xp_bar.cyan(), character.xp_progress_percent());

        println!("║   Total Gold:   {:>6} 💰                             ║", character.gold);
        println!("║   Tasks Done:   {:>6} ✓                              ║", character.tasks_completed);
        println!("{}", "╚══════════════════════════════════════════════════════╝".yellow());
        println!();
    }
}
