use anyhow::Result;
use chrono::Utc;
use super::parser::TaskData;
use crate::character::Character;
use crate::progression::{XPCalculator, GoldCalculator, LootSystem, LootDrop};
use crate::storage::{get_data_dir, safe_write, load_with_backup};
use crate::achievements::AchievementTracker;

pub struct TaskwarriorIntegration;

impl TaskwarriorIntegration {
    /// Process a task completion
    pub fn process_completion(task: &TaskData) -> Result<()> {
        if !task.is_completed() {
            return Ok(());
        }

        let data_dir = get_data_dir()?;
        let character_path = data_dir.join("character.json");
        let achievements_path = data_dir.join("achievements.json");

        // Load character
        let mut character: Character = load_with_backup(&character_path)?;

        // Load achievement tracker
        let mut tracker = AchievementTracker::load(&achievements_path)
            .unwrap_or_else(|_| AchievementTracker::new());

        // Calculate rewards
        let challenge = task.get_challenge();
        let urgency = task.get_urgency();

        // Determine timing
        let completion_time = task.get_completion_date().unwrap_or_else(Utc::now);
        let timing = XPCalculator::determine_timing(task.get_due_date(), completion_time);

        // Calculate XP and gold
        let xp = XPCalculator::calculate(challenge, urgency, timing);
        let base_gold = GoldCalculator::calculate(challenge);

        // Roll for loot
        let loot_drop = LootSystem::roll_for_loot(challenge);
        let mut bonus_gold = 0;
        let mut loot_info = None;

        if let Some(ref loot) = loot_drop {
            match loot {
                LootDrop::Gold(amount) => {
                    bonus_gold = *amount;
                }
                LootDrop::Reward { tier, name } => {
                    loot_info = Some((*tier, name.clone()));
                }
            }

            // Track loot drop in achievements
            let new_achievements = tracker.record_loot_drop(loot, &character);
            if !new_achievements.is_empty() {
                Self::print_achievements(&new_achievements);
            }
        }

        // Update character
        let total_gold = base_gold + bonus_gold;
        character.add_xp(xp);
        character.add_gold(total_gold);

        // Get stats from task (if specified)
        let stat1 = task.get_stat1();
        let stat2 = task.get_stat2();
        character.complete_task(challenge, stat1, stat2);

        // Track quest completion in achievements
        let project = task.get_project();
        let new_achievements = tracker.record_quest_completion(
            &character,
            challenge,
            timing,
            project.as_deref(),
        );

        // Save everything
        safe_write(&character_path, &character)?;
        safe_write(&achievements_path, &tracker)?;

        // Print reward notification
        Self::print_rewards(xp, base_gold, bonus_gold, &character, loot_info.as_ref());

        // Print achievement notifications
        if !new_achievements.is_empty() {
            Self::print_achievements(&new_achievements);
        }

        Ok(())
    }

    fn print_rewards(
        xp: u32,
        base_gold: u32,
        bonus_gold: u32,
        character: &Character,
        loot: Option<&(crate::progression::RewardTier, String)>,
    ) {
        use colored::Colorize;

        println!();
        println!("{}", "╔════════════════════════════════════════╗".cyan());
        println!("{}", "║       ⚔️  QUEST COMPLETE! ⚔️            ║".cyan().bold());
        println!("{}", "╠════════════════════════════════════════╣".cyan());

        if bonus_gold > 0 {
            println!("║ {} XP  │  {} Gold (+ {} bonus!)  ║",
                format!("+{}", xp).green().bold(),
                format!("+{}", base_gold).yellow().bold(),
                bonus_gold.to_string().yellow()
            );
        } else {
            println!("║ {} XP  │  {} Gold                  ║",
                format!("+{}", xp).green().bold(),
                format!("+{}", base_gold).yellow().bold()
            );
        }

        // Display loot drop if present
        if let Some((tier, name)) = loot {
            println!("{}", "╠════════════════════════════════════════╣".cyan());
            let tier_color = match tier {
                crate::progression::RewardTier::Normal => "green",
                crate::progression::RewardTier::Heroic => "blue",
                crate::progression::RewardTier::Epic => "magenta",
                crate::progression::RewardTier::Legendary => "yellow",
            };
            println!("║ 💎 LOOT DROP! {} [{:?}]       ║",
                name.color(tier_color).bold(),
                tier
            );
        }

        println!("{}", "╠════════════════════════════════════════╣".cyan());
        println!("║ Level: {}  │  XP: {}/{}           ║",
            character.level,
            character.total_xp,
            character.total_xp + character.xp_to_next_level()
        );
        println!("║ Gold: {}                               ║", character.gold);
        println!("{}", "╚════════════════════════════════════════╝".cyan());
        println!();
    }

    fn print_achievements(achievements: &[&crate::achievements::Achievement]) {
        use colored::Colorize;

        for achievement in achievements {
            println!();
            println!("{}", "╔════════════════════════════════════════╗".yellow());
            println!("{}", "║      🏆 ACHIEVEMENT UNLOCKED! 🏆       ║".yellow().bold());
            println!("{}", "╠════════════════════════════════════════╣".yellow());

            let tier_str = achievement.tier.name();
            let color = achievement.tier.color_code();

            println!("║ {} {} - [{}]                    ║",
                achievement.icon,
                achievement.title.color(color).bold(),
                tier_str.color(color)
            );
            println!("║ {}                             ║", achievement.description);
            println!("{}", "╚════════════════════════════════════════╝".yellow());
            println!();
        }
    }
}
