use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use std::io::{self, Write};
use std::str::FromStr;

use crate::character::{Character, Class};
use crate::storage::{get_data_dir, safe_write, load_with_backup};
use crate::taskwarrior::UDAManager;
use crate::display::Formatter;
use crate::shop::RewardStore;
use crate::achievements::AchievementTracker;

#[derive(Parser)]
#[command(name = "taskquest")]
#[command(about = "Gamified RPG system for Taskwarrior", long_about = None)]
pub struct CLI {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Initialize TaskQuest (setup wizard)
    Init {
        /// Skip wizard and use existing data
        #[arg(long)]
        skip_wizard: bool,
    },
    /// View character status
    #[command(alias = "stat")]
    #[command(alias = "char")]
    Status,
    /// View detailed stats
    Stats,
    /// Set character name
    Name { name: String },
    /// Set character class
    Class { class: String },
    /// List available shop rewards
    Shop,
    /// Purchase a reward from the shop
    Buy {
        /// Reward ID or name to purchase
        reward: String,
    },
    /// View unlocked achievements
    Achievements,
    /// Add a custom reward to the shop
    AddReward {
        /// Reward name
        name: String,
        /// Gold cost
        cost: u32,
        /// Reward description
        description: String,
        /// Tier (normal/heroic/epic/legendary)
        #[arg(default_value = "normal")]
        tier: String,
        /// Cooldown in hours (optional)
        #[arg(long, default_value = "0")]
        cooldown: u32,
    },
    /// Remove a reward from the shop
    RemoveReward {
        /// Reward ID or name to remove
        reward: String,
    },
    /// Sync commands for git-based synchronization
    Sync {
        #[command(subcommand)]
        action: SyncAction,
    },
}

#[derive(Subcommand)]
pub enum SyncAction {
    /// Initialize git repository for syncing
    Init {
        /// Optional remote URL (GitHub/GitLab)
        #[arg(long)]
        remote: Option<String>,
    },
    /// Push changes to remote
    Push,
    /// Pull changes from remote
    Pull,
    /// Show sync status
    Status,
    /// Show sync history
    History {
        /// Number of commits to show
        #[arg(default_value = "10")]
        limit: usize,
    },
}

impl CLI {
    pub fn run(self) -> Result<()> {
        match self.command {
            Commands::Init { skip_wizard } => Self::init(skip_wizard),
            Commands::Status => Self::status(),
            Commands::Stats => Self::stats(),
            Commands::Name { name } => Self::set_name(name),
            Commands::Class { class } => Self::set_class(class),
            Commands::Shop => Self::list_shop(),
            Commands::Buy { reward } => Self::buy_reward(reward),
            Commands::Achievements => Self::list_achievements(),
            Commands::AddReward { name, cost, description, tier, cooldown } =>
                Self::add_reward(name, cost, description, tier, cooldown),
            Commands::RemoveReward { reward } => Self::remove_reward(reward),
            Commands::Sync { action } => Self::handle_sync(action),
        }
    }

    fn init(skip_wizard: bool) -> Result<()> {
        let data_dir = get_data_dir()?;
        let character_path = data_dir.join("character.json");

        if character_path.exists() && skip_wizard {
            println!("Character already exists. Use 'taskquest init' without --skip-wizard to recreate.");
            return Ok(());
        }

        // Configure UDAs
        println!("Configuring Taskwarrior UDAs...");
        UDAManager::configure()?;

        if skip_wizard {
            return Ok(());
        }

        // Setup wizard
        println!("\n╔══════════════════════════════════════════╗");
        println!("║   ⚔️  Welcome to TaskQuest! ⚔️           ║");
        println!("╠══════════════════════════════════════════╣");
        println!("║  Transform your tasks into an epic RPG  ║");
        println!("╚══════════════════════════════════════════╝\n");

        // Get hero name
        print!("What is your hero's name? ");
        io::stdout().flush()?;
        let mut name = String::new();
        io::stdin().read_line(&mut name)?;
        let name = name.trim().to_string();

        // Choose class
        println!("\nChoose your class (for flavor and roleplay):");
        for (i, class) in Class::all().iter().enumerate() {
            println!("  {}. {} - {}",
                i + 1,
                class,
                class.description()
            );
        }
        print!("\nYour choice (1-5): ");
        io::stdout().flush()?;
        let mut choice = String::new();
        io::stdin().read_line(&mut choice)?;
        let class_idx: usize = choice.trim().parse::<usize>()
            .context("Invalid choice")? - 1;
        let class = Class::all()[class_idx];

        // Create character
        let character = Character::new(name, class);

        // Save character
        safe_write(&character_path, &character)?;

        println!("\n✅ Character created successfully!");
        println!("\nYour hero: {}, the {}, Level {}",
            character.name, character.class, character.level);
        println!("\n💡 Tip: Set stat1 and stat2 on tasks to train specific stats!");
        println!("   Example: task add \"Code review\" challenge:6 stat1:int stat2:wis");
        println!("\nStart completing tasks to earn XP, gold, and loot!");
        println!("Use 'taskquest status' to view your progress.\n");

        Ok(())
    }

    fn status() -> Result<()> {
        let data_dir = get_data_dir()?;
        let character_path = data_dir.join("character.json");

        let character: Character = load_with_backup(&character_path)
            .context("Character not found. Run 'taskquest init' first")?;

        Formatter::print_status(&character);

        Ok(())
    }

    fn stats() -> Result<()> {
        let data_dir = get_data_dir()?;
        let character_path = data_dir.join("character.json");

        let character: Character = load_with_backup(&character_path)
            .context("Character not found. Run 'taskquest init' first")?;

        Formatter::print_stats(&character);

        Ok(())
    }

    fn set_name(name: String) -> Result<()> {
        let data_dir = get_data_dir()?;
        let character_path = data_dir.join("character.json");

        let mut character: Character = load_with_backup(&character_path)?;
        character.name = name.clone();
        safe_write(&character_path, &character)?;

        println!("✅ Character name set to: {}", name);

        Ok(())
    }

    fn set_class(class_str: String) -> Result<()> {
        let data_dir = get_data_dir()?;
        let character_path = data_dir.join("character.json");

        let class = Class::from_str(&class_str)?;
        let mut character: Character = load_with_backup(&character_path)?;
        character.class = class;
        safe_write(&character_path, &character)?;

        println!("✅ Character class set to: {}", class);

        Ok(())
    }

    fn list_shop() -> Result<()> {
        use colored::Colorize;
        use crate::progression::RewardTier;

        let data_dir = get_data_dir()?;
        let character_path = data_dir.join("character.json");
        let shop_path = data_dir.join("shop.json");

        let character: Character = load_with_backup(&character_path)?;
        let shop = RewardStore::load(&shop_path)?;

        println!();
        println!("{}", "╔════════════════════════════════════════════════════╗".cyan());
        println!("{}", "║              🏪  REWARD SHOP  🏪                   ║".cyan().bold());
        println!("{}", "╠════════════════════════════════════════════════════╣".cyan());
        println!("║ Your Gold: {} 💰                                    ║", character.gold.to_string().yellow().bold());
        println!("{}", "╚════════════════════════════════════════════════════╝".cyan());
        println!();

        let available = shop.available_rewards();

        if available.is_empty() {
            println!("No rewards are currently available (all on cooldown).");
            return Ok(());
        }

        for reward in available {
            let tier_color = match reward.tier {
                RewardTier::Normal => "white",
                RewardTier::Heroic => "blue",
                RewardTier::Epic => "magenta",
                RewardTier::Legendary => "yellow",
            };

            let affordable = if character.gold >= reward.cost {
                "✓".green()
            } else {
                "✗".red()
            };

            println!("{}  [ID: {}] {} - {} Gold",
                affordable,
                reward.id.to_string().cyan(),
                reward.name.color(tier_color).bold(),
                reward.cost.to_string().yellow()
            );
            println!("     {}", reward.description);

            if reward.cooldown_hours > 0 {
                println!("     ⏰ Cooldown: {} hours", reward.cooldown_hours);
            }
            println!();
        }

        println!("Use 'taskquest buy <id or name>' to purchase a reward");
        println!();

        Ok(())
    }

    fn buy_reward(reward_identifier: String) -> Result<()> {
        use colored::Colorize;

        let data_dir = get_data_dir()?;
        let character_path = data_dir.join("character.json");
        let shop_path = data_dir.join("shop.json");
        let achievements_path = data_dir.join("achievements.json");

        let mut character: Character = load_with_backup(&character_path)?;
        let mut shop = RewardStore::load(&shop_path)?;
        let mut tracker = AchievementTracker::load(&achievements_path)
            .unwrap_or_else(|_| AchievementTracker::new());

        // Try to parse as ID first, otherwise search by name
        let reward_id = if let Ok(id) = reward_identifier.parse::<u32>() {
            id
        } else {
            shop.get_reward_by_name(&reward_identifier)
                .map(|r| r.id)
                .context(format!("Reward '{}' not found", reward_identifier))?
        };

        // Get reward info for display
        let reward_name = shop.get_reward(reward_id)
            .context("Reward not found")?
            .name.clone();

        // Attempt purchase
        let cost = shop.purchase_reward(reward_id, character.gold)?;

        // Deduct gold from character
        character.gold -= cost;

        // Track purchase in achievements
        let new_achievements = tracker.record_reward_purchase(&character);

        // Save all changes
        safe_write(&character_path, &character)?;
        safe_write(&shop_path, &shop)?;
        safe_write(&achievements_path, &tracker)?;

        // Print success message
        println!();
        println!("{}", "╔════════════════════════════════════════╗".green());
        println!("{}", "║      ✨ PURCHASE SUCCESSFUL! ✨        ║".green().bold());
        println!("{}", "╠════════════════════════════════════════╣".green());
        println!("║ Reward: {}                             ║", reward_name.bold());
        println!("║ Cost: {} Gold                          ║", cost.to_string().yellow());
        println!("║ Remaining Gold: {}                     ║", character.gold.to_string().yellow().bold());
        println!("{}", "╚════════════════════════════════════════╝".green());
        println!();
        println!("Enjoy your reward! 🎉");
        println!();

        // Display any new achievements
        if !new_achievements.is_empty() {
            for achievement in new_achievements {
                let color = achievement.tier.color_code();
                println!();
                println!("{}", "╔════════════════════════════════════════╗".yellow());
                println!("{}", "║      🏆 ACHIEVEMENT UNLOCKED! 🏆       ║".yellow().bold());
                println!("{}", "╠════════════════════════════════════════╣".yellow());
                println!("║ {} {} - [{}]              ║",
                    achievement.icon,
                    achievement.title.color(color).bold(),
                    achievement.tier.name().color(color)
                );
                println!("║ {}                         ║", achievement.description);
                println!("{}", "╚════════════════════════════════════════╝".yellow());
                println!();
            }
        }

        Ok(())
    }

    fn list_achievements() -> Result<()> {
        use colored::Colorize;

        let data_dir = get_data_dir()?;
        let character_path = data_dir.join("character.json");
        let achievements_path = data_dir.join("achievements.json");

        let character: Character = load_with_backup(&character_path)?;
        let tracker = AchievementTracker::load(&achievements_path)
            .unwrap_or_else(|_| AchievementTracker::new());

        println!();
        println!("{}", "╔════════════════════════════════════════════════════╗".yellow());
        println!("{}", "║            🏆  ACHIEVEMENTS  🏆                     ║".yellow().bold());
        println!("{}", "╚════════════════════════════════════════════════════╝".yellow());
        println!();

        let unlocked = tracker.get_unlocked_achievements();

        if unlocked.is_empty() {
            println!("No achievements unlocked yet. Complete quests to earn achievements!");
            println!();
            return Ok(());
        }

        // Group by tier
        use crate::achievements::AchievementTier;
        for tier in [
            AchievementTier::Legendary,
            AchievementTier::Epic,
            AchievementTier::Rare,
            AchievementTier::Uncommon,
            AchievementTier::Common,
        ] {
            let tier_achievements: Vec<_> = unlocked.iter()
                .filter(|a| a.tier == tier)
                .collect();

            if !tier_achievements.is_empty() {
                let color = tier.color_code();
                println!("{}", format!("=== {} ===", tier.name()).color(color).bold());
                for achievement in tier_achievements {
                    let progress = tracker.get_achievement_progress(achievement.id, &character);
                    println!("{} {} - {}",
                        achievement.icon,
                        achievement.title.color(color).bold(),
                        achievement.description
                    );
                    if progress < 1.0 {
                        println!("   Progress: {:.0}%", progress * 100.0);
                    }
                }
                println!();
            }
        }

        let total = crate::achievements::Achievement::all().len();
        println!("Unlocked: {}/{} ({:.1}%)",
            unlocked.len(),
            total,
            (unlocked.len() as f64 / total as f64) * 100.0
        );
        println!();

        Ok(())
    }

    fn add_reward(name: String, cost: u32, description: String, tier_str: String, cooldown: u32) -> Result<()> {
        use colored::Colorize;
        use crate::progression::RewardTier;

        let data_dir = get_data_dir()?;
        let shop_path = data_dir.join("shop.json");

        let mut shop = RewardStore::load(&shop_path)?;

        // Parse tier
        let tier = match tier_str.to_lowercase().as_str() {
            "normal" => RewardTier::Normal,
            "heroic" => RewardTier::Heroic,
            "epic" => RewardTier::Epic,
            "legendary" => RewardTier::Legendary,
            _ => return Err(anyhow::anyhow!("Invalid tier. Use: normal, heroic, epic, or legendary")),
        };

        // Add the reward
        let reward_id = shop.add_reward(name.clone(), cost, description.clone(), tier, cooldown);

        // Save changes
        safe_write(&shop_path, &shop)?;

        // Display success
        println!();
        println!("{}", "╔════════════════════════════════════════╗".green());
        println!("{}", "║      ✨ REWARD ADDED! ✨              ║".green().bold());
        println!("{}", "╠════════════════════════════════════════╣".green());
        println!("║ ID: {}                                  ║", reward_id.to_string().cyan().bold());
        println!("║ Name: {}                               ║", name.bold());
        println!("║ Cost: {} Gold                          ║", cost.to_string().yellow());
        println!("║ Tier: {}                               ║", tier_str);
        if cooldown > 0 {
            println!("║ Cooldown: {} hours                     ║", cooldown);
        }
        println!("{}", "╚════════════════════════════════════════╝".green());
        println!();
        println!("Reward added to shop! View with 'taskquest shop'");
        println!();

        Ok(())
    }

    fn remove_reward(reward_identifier: String) -> Result<()> {
        use colored::Colorize;

        let data_dir = get_data_dir()?;
        let shop_path = data_dir.join("shop.json");

        let mut shop = RewardStore::load(&shop_path)?;

        // Try to parse as ID first, otherwise search by name
        let reward_id = if let Ok(id) = reward_identifier.parse::<u32>() {
            id
        } else {
            shop.get_reward_by_name(&reward_identifier)
                .map(|r| r.id)
                .context(format!("Reward '{}' not found", reward_identifier))?
        };

        // Get reward name before removing
        let reward_name = shop.get_reward(reward_id)
            .context("Reward not found")?
            .name.clone();

        // Remove the reward
        shop.remove_reward(reward_id)?;

        // Save changes
        safe_write(&shop_path, &shop)?;

        // Display success
        println!();
        println!("{}", "╔════════════════════════════════════════╗".yellow());
        println!("{}", "║      🗑️  REWARD REMOVED  🗑️            ║".yellow().bold());
        println!("{}", "╠════════════════════════════════════════╣".yellow());
        println!("║ Removed: {}                            ║", reward_name.bold());
        println!("{}", "╚════════════════════════════════════════╝".yellow());
        println!();

        Ok(())
    }

    fn handle_sync(action: SyncAction) -> Result<()> {
        use crate::sync::GitSync;

        let data_dir = get_data_dir()?;

        match action {
            SyncAction::Init { remote } => {
                GitSync::init(&data_dir, remote)?;
            }
            SyncAction::Push => {
                GitSync::push(&data_dir)?;
            }
            SyncAction::Pull => {
                GitSync::pull(&data_dir)?;
            }
            SyncAction::Status => {
                GitSync::status(&data_dir)?;
            }
            SyncAction::History { limit } => {
                GitSync::history(&data_dir, limit)?;
            }
        }

        Ok(())
    }
}
