use serde::{Deserialize, Serialize};
use anyhow::{Result, Context};
use std::path::Path;
use std::collections::{HashMap, HashSet};
use chrono::{Utc, NaiveDate};
use crate::achievements::definitions::{Achievement, ACHIEVEMENTS};
use crate::character::Character;
use crate::progression::LootDrop;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AchievementProgress {
    // Tracking for various achievement conditions
    pub quests_completed: u32,
    pub quests_by_difficulty: HashMap<u8, u32>, // difficulty -> count
    pub difficulty_10_quests: u32,
    pub active_days: HashSet<String>, // Set of dates (YYYY-MM-DD)
    pub projects_completed: HashSet<String>, // Set of project names
    pub epic_loot_received: bool,
    pub loot_drops_received: u32,
    pub rewards_purchased: u32,
    pub early_tasks: u32, // >24hrs before due
    pub grace_period_tasks: u32,
    pub on_time_tasks: u32, // within due date, not early
    pub tasks_with_due_date: u32,
    pub last_activity_date: Option<String>, // ISO date
    pub comeback_quests_after_break: u32, // Track if on comeback
    pub had_30_day_break: bool,
    pub highest_stat_value: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AchievementTracker {
    pub unlocked: HashSet<String>, // Achievement IDs that have been unlocked
    pub progress: AchievementProgress,
}

impl Default for AchievementProgress {
    fn default() -> Self {
        Self {
            quests_completed: 0,
            quests_by_difficulty: HashMap::new(),
            difficulty_10_quests: 0,
            active_days: HashSet::new(),
            projects_completed: HashSet::new(),
            epic_loot_received: false,
            loot_drops_received: 0,
            rewards_purchased: 0,
            early_tasks: 0,
            grace_period_tasks: 0,
            on_time_tasks: 0,
            tasks_with_due_date: 0,
            last_activity_date: None,
            comeback_quests_after_break: 0,
            had_30_day_break: false,
            highest_stat_value: 0,
        }
    }
}

impl Default for AchievementTracker {
    fn default() -> Self {
        Self::new()
    }
}

impl AchievementTracker {
    pub fn new() -> Self {
        Self {
            unlocked: HashSet::new(),
            progress: AchievementProgress::default(),
        }
    }

    pub fn load(path: &Path) -> Result<Self> {
        if !path.exists() {
            return Ok(Self::new());
        }

        let file = std::fs::File::open(path)
            .context("Failed to open achievements file")?;
        let tracker: AchievementTracker = serde_json::from_reader(file)
            .context("Failed to parse achievements JSON")?;
        Ok(tracker)
    }

    pub fn save(&self, path: &Path) -> Result<()> {
        crate::storage::safe_write(path, self)
    }

    /// Record a completed quest and check for newly unlocked achievements
    pub fn record_quest_completion(
        &mut self,
        character: &Character,
        difficulty: u8,
        timing: TaskTiming,
        project: Option<&str>,
    ) -> Vec<&'static Achievement> {
        let today = Utc::now().format("%Y-%m-%d").to_string();

        // Update progress
        self.progress.quests_completed += 1;
        *self.progress.quests_by_difficulty.entry(difficulty).or_insert(0) += 1;

        if difficulty == 10 {
            self.progress.difficulty_10_quests += 1;
        }

        // Track active days
        self.progress.active_days.insert(today.clone());

        // Check for comeback scenario
        if let Some(ref last_date) = self.progress.last_activity_date {
            if let Ok(last) = NaiveDate::parse_from_str(last_date, "%Y-%m-%d") {
                if let Ok(current) = NaiveDate::parse_from_str(&today, "%Y-%m-%d") {
                    let days_diff = (current - last).num_days();
                    if days_diff >= 30 {
                        self.progress.had_30_day_break = true;
                        self.progress.comeback_quests_after_break = 0;
                    }
                    if self.progress.had_30_day_break {
                        self.progress.comeback_quests_after_break += 1;
                    }
                }
            }
        }
        self.progress.last_activity_date = Some(today);

        // Track projects
        if let Some(proj) = project {
            self.progress.projects_completed.insert(proj.to_string());
        }

        // Track timing
        match timing {
            TaskTiming::Early => self.progress.early_tasks += 1,
            TaskTiming::OnTime => self.progress.on_time_tasks += 1,
            TaskTiming::GracePeriod => self.progress.grace_period_tasks += 1,
            _ => {}
        }

        if timing != TaskTiming::NoDueDate {
            self.progress.tasks_with_due_date += 1;
        }

        // Update highest stat
        let max_stat = character.stats.strength
            .max(character.stats.dexterity)
            .max(character.stats.constitution)
            .max(character.stats.intelligence)
            .max(character.stats.wisdom)
            .max(character.stats.charisma);
        if max_stat > self.progress.highest_stat_value {
            self.progress.highest_stat_value = max_stat;
        }

        // Check for newly unlocked achievements
        self.check_achievements(character)
    }

    /// Record loot drop received
    pub fn record_loot_drop(&mut self, loot: &LootDrop, character: &Character) -> Vec<&'static Achievement> {
        self.progress.loot_drops_received += 1;

        if matches!(loot, LootDrop::Reward { tier: crate::progression::RewardTier::Epic, .. }) {
            self.progress.epic_loot_received = true;
        }

        self.check_achievements(character)
    }

    /// Record a reward purchase
    pub fn record_reward_purchase(&mut self, character: &Character) -> Vec<&'static Achievement> {
        self.progress.rewards_purchased += 1;
        self.check_achievements(character)
    }

    /// Check all achievement conditions and return newly unlocked ones
    fn check_achievements(&mut self, character: &Character) -> Vec<&'static Achievement> {
        let mut newly_unlocked = Vec::new();

        for achievement in ACHIEVEMENTS.iter() {
            // Skip if already unlocked
            if self.unlocked.contains(achievement.id) {
                continue;
            }

            // Check if this achievement is now unlocked
            if self.is_achievement_unlocked(achievement, character) {
                self.unlocked.insert(achievement.id.to_string());
                newly_unlocked.push(achievement);
            }
        }

        newly_unlocked
    }

    /// Check if a specific achievement's conditions are met
    fn is_achievement_unlocked(&self, achievement: &Achievement, character: &Character) -> bool {
        match achievement.id {
            // Milestone achievements
            "first_steps" => self.progress.quests_completed >= 1,
            "journey_begins" => character.level >= 5,
            "seasoned_adventurer" => self.progress.quests_completed >= 100,
            "veteran_hero" => self.progress.quests_completed >= 500,
            "legendary_warrior" => character.level >= 30,
            "living_legend" => character.level >= 50,
            "completionist" => self.progress.quests_completed >= 1000,

            // Variety achievements
            "master_of_balance" => {
                // Must have completed at least one task of each difficulty (1-10)
                (1..=10).all(|diff| {
                    self.progress.quests_by_difficulty.get(&diff).unwrap_or(&0) >= &1
                })
            }
            "the_undaunted" => self.progress.difficulty_10_quests >= 10,
            "jack_of_all_trades" => {
                // At least 20 tasks of each difficulty (1-10)
                (1..=10).all(|diff| {
                    self.progress.quests_by_difficulty.get(&diff).unwrap_or(&0) >= &20
                })
            }

            // Long-term progress
            "consistent_hero" => self.progress.active_days.len() >= 30,
            "marathon_hero" => self.progress.active_days.len() >= 100,
            "renaissance_soul" => self.progress.projects_completed.len() >= 10,

            // Comeback
            "phoenix_rising" => {
                self.progress.had_30_day_break && self.progress.comeback_quests_after_break >= 5
            }

            // Loot & Rewards
            "epic_collector" => self.progress.epic_loot_received,
            "gold_hoarder" => character.gold >= 5000,
            "wise_spender" => self.progress.rewards_purchased >= 10,
            "treasure_hunter" => self.progress.loot_drops_received >= 50,

            // Timing
            "early_riser" => self.progress.early_tasks >= 50,
            "pressure_handler" => self.progress.grace_period_tasks >= 25,
            "time_master" => self.progress.tasks_with_due_date >= 100,
            "punctual_perfectionist" => self.progress.on_time_tasks >= 100,

            // Power achievements (stat-based)
            "strength_incarnate" => character.stats.strength >= 100,
            "lightning_reflexes" => character.stats.dexterity >= 100,
            "iron_constitution" => character.stats.constitution >= 100,
            "brilliant_mind" => character.stats.intelligence >= 100,
            "sage_wisdom" => character.stats.wisdom >= 100,
            "magnetic_personality" => character.stats.charisma >= 100,

            // Legendary stat achievements
            "legendary_strength" => self.progress.highest_stat_value >= 500,
            "transcendent_power" => self.progress.highest_stat_value >= 1000,

            _ => false,
        }
    }

    /// Get all unlocked achievements
    pub fn get_unlocked_achievements(&self) -> Vec<&'static Achievement> {
        ACHIEVEMENTS
            .iter()
            .filter(|a| self.unlocked.contains(a.id))
            .collect()
    }

    /// Get progress toward a specific achievement (0.0 to 1.0)
    pub fn get_achievement_progress(&self, achievement_id: &str, character: &Character) -> f64 {
        if self.unlocked.contains(achievement_id) {
            return 1.0;
        }

        match achievement_id {
            "first_steps" => (self.progress.quests_completed.min(1) as f64) / 1.0,
            "journey_begins" => (character.level.min(5) as f64) / 5.0,
            "seasoned_adventurer" => (self.progress.quests_completed.min(100) as f64) / 100.0,
            "veteran_hero" => (self.progress.quests_completed.min(500) as f64) / 500.0,
            "legendary_warrior" => (character.level.min(30) as f64) / 30.0,
            "living_legend" => (character.level.min(50) as f64) / 50.0,
            "completionist" => (self.progress.quests_completed.min(1000) as f64) / 1000.0,

            "master_of_balance" => {
                let completed_difficulties = (1..=10).filter(|diff| {
                    self.progress.quests_by_difficulty.get(diff).unwrap_or(&0) >= &1
                }).count();
                (completed_difficulties as f64) / 10.0
            }
            "the_undaunted" => (self.progress.difficulty_10_quests.min(10) as f64) / 10.0,
            "jack_of_all_trades" => {
                let min_count = (1..=10).map(|diff| {
                    self.progress.quests_by_difficulty.get(&diff).unwrap_or(&0)
                }).min().unwrap_or(&0);
                (*min_count.min(&20) as f64) / 20.0
            }

            "consistent_hero" => (self.progress.active_days.len().min(30) as f64) / 30.0,
            "marathon_hero" => (self.progress.active_days.len().min(100) as f64) / 100.0,
            "renaissance_soul" => (self.progress.projects_completed.len().min(10) as f64) / 10.0,

            "phoenix_rising" => {
                if self.progress.had_30_day_break {
                    (self.progress.comeback_quests_after_break.min(5) as f64) / 5.0
                } else {
                    0.0
                }
            }

            "epic_collector" => if self.progress.epic_loot_received { 1.0 } else { 0.0 },
            "gold_hoarder" => (character.gold.min(5000) as f64) / 5000.0,
            "wise_spender" => (self.progress.rewards_purchased.min(10) as f64) / 10.0,
            "treasure_hunter" => (self.progress.loot_drops_received.min(50) as f64) / 50.0,

            "early_riser" => (self.progress.early_tasks.min(50) as f64) / 50.0,
            "pressure_handler" => (self.progress.grace_period_tasks.min(25) as f64) / 25.0,
            "time_master" => (self.progress.tasks_with_due_date.min(100) as f64) / 100.0,
            "punctual_perfectionist" => (self.progress.on_time_tasks.min(100) as f64) / 100.0,

            "strength_incarnate" => (character.stats.strength.min(100) as f64) / 100.0,
            "lightning_reflexes" => (character.stats.dexterity.min(100) as f64) / 100.0,
            "iron_constitution" => (character.stats.constitution.min(100) as f64) / 100.0,
            "brilliant_mind" => (character.stats.intelligence.min(100) as f64) / 100.0,
            "sage_wisdom" => (character.stats.wisdom.min(100) as f64) / 100.0,
            "magnetic_personality" => (character.stats.charisma.min(100) as f64) / 100.0,

            "legendary_strength" => (self.progress.highest_stat_value.min(500) as f64) / 500.0,
            "transcendent_power" => (self.progress.highest_stat_value.min(1000) as f64) / 1000.0,

            _ => 0.0,
        }
    }

    /// Get achievements by tier
    pub fn get_achievements_by_tier(&self, tier: crate::achievements::AchievementTier) -> Vec<&'static Achievement> {
        ACHIEVEMENTS
            .iter()
            .filter(|a| a.tier == tier)
            .collect()
    }

    /// Get count of unlocked achievements by tier
    pub fn get_unlocked_count_by_tier(&self, tier: crate::achievements::AchievementTier) -> usize {
        ACHIEVEMENTS
            .iter()
            .filter(|a| a.tier == tier && self.unlocked.contains(a.id))
            .count()
    }
}

// Re-export TaskTiming from progression module
use crate::progression::TaskTiming;
