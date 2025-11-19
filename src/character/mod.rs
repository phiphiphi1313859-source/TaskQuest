pub mod stats;
pub mod class;
pub mod level;
pub mod avatars;

pub use stats::Stats;
pub use class::Class;
pub use level::LevelSystem;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Character {
    pub name: String,
    pub class: Class,
    pub level: u32,
    pub total_xp: u32,
    pub stats: Stats,
    pub gold: u32,
    pub tasks_completed: u32,
    pub active_title: Option<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum StatType {
    STR,
    DEX,
    CON,
    INT,
    WIS,
    CHA,
}

impl std::str::FromStr for StatType {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "STR" => Ok(StatType::STR),
            "DEX" => Ok(StatType::DEX),
            "CON" => Ok(StatType::CON),
            "INT" => Ok(StatType::INT),
            "WIS" => Ok(StatType::WIS),
            "CHA" => Ok(StatType::CHA),
            _ => Err(anyhow::anyhow!("Invalid stat type: {}", s)),
        }
    }
}

impl Character {
    pub fn new(name: String, class: Class) -> Self {
        Self {
            name,
            class,
            level: 1,
            total_xp: 0,
            stats: Stats::new(),
            gold: 0,
            tasks_completed: 0,
            active_title: None,
        }
    }

    pub fn add_xp(&mut self, xp: u32) {
        self.total_xp += xp;
        self.level = LevelSystem::level_from_xp(self.total_xp);
    }

    pub fn add_gold(&mut self, gold: u32) {
        self.gold += gold;
    }

    pub fn complete_task(&mut self, challenge: u8, stat1: Option<StatType>, stat2: Option<StatType>) {
        self.tasks_completed += 1;

        // Base formula: challenge / 20.0
        // Challenge 1: 20 tasks per point, Challenge 5: 4 tasks per point, Challenge 10: 2 tasks per point
        // Diminishing returns are applied inside increase_stat()
        let base_gain = challenge as f64 / 20.0;

        // stat1 gets full gain, stat2 gets half
        let stat1_gain = base_gain;
        let stat2_gain = base_gain / 2.0;

        // Increase stats if specified (diminishing returns applied automatically)
        if let Some(s1) = stat1 {
            self.stats.increase_stat(s1, stat1_gain);
        }
        if let Some(s2) = stat2 {
            self.stats.increase_stat(s2, stat2_gain);
        }
    }

    pub fn xp_to_next_level(&self) -> u32 {
        let next_level_xp = LevelSystem::xp_for_level(self.level + 1);
        next_level_xp.saturating_sub(self.total_xp)
    }

    pub fn xp_progress_percent(&self) -> f64 {
        let current_level_xp = LevelSystem::xp_for_level(self.level);
        let next_level_xp = LevelSystem::xp_for_level(self.level + 1);
        let xp_in_level = self.total_xp - current_level_xp;
        let xp_needed = next_level_xp - current_level_xp;

        (xp_in_level as f64 / xp_needed as f64) * 100.0
    }
}
