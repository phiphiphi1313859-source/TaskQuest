use serde::{Deserialize, Serialize};
use super::StatType;

const STAT_CAP: f64 = 99.0;
const STAT_BASE: f64 = 10.0;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stats {
    // Stats stored as f64 internally for fractional progression
    // Displayed as u16 to users
    pub strength: f64,
    pub dexterity: f64,
    pub constitution: f64,
    pub intelligence: f64,
    pub wisdom: f64,
    pub charisma: f64,
}

impl Stats {
    pub fn new() -> Self {
        Self {
            strength: STAT_BASE,
            dexterity: STAT_BASE,
            constitution: STAT_BASE,
            intelligence: STAT_BASE,
            wisdom: STAT_BASE,
            charisma: STAT_BASE,
        }
    }

    /// Get stat value as display integer (what the user sees)
    pub fn get_stat(&self, stat_type: StatType) -> u16 {
        self.get_stat_raw(stat_type).floor() as u16
    }

    /// Get raw stat value (internal f64)
    pub fn get_stat_raw(&self, stat_type: StatType) -> f64 {
        match stat_type {
            StatType::STR => self.strength,
            StatType::DEX => self.dexterity,
            StatType::CON => self.constitution,
            StatType::INT => self.intelligence,
            StatType::WIS => self.wisdom,
            StatType::CHA => self.charisma,
        }
    }

    /// Calculate diminishing returns multiplier (quadratic curve)
    /// Returns a value from 1.0 (at base stat) to 2.0 (at cap)
    fn get_difficulty_multiplier(current_stat: f64) -> f64 {
        let progress = (current_stat - STAT_BASE) / (STAT_CAP - STAT_BASE);
        1.0 + progress.powi(2)
    }

    /// Increase stat with quadratic diminishing returns
    /// Base gain is reduced based on how close the stat is to the cap
    pub fn increase_stat(&mut self, stat_type: StatType, base_gain: f64) {
        let current = self.get_stat_raw(stat_type);
        let difficulty = Self::get_difficulty_multiplier(current);
        let actual_gain = base_gain / difficulty;

        let new_value = (current + actual_gain).min(STAT_CAP);

        match stat_type {
            StatType::STR => self.strength = new_value,
            StatType::DEX => self.dexterity = new_value,
            StatType::CON => self.constitution = new_value,
            StatType::INT => self.intelligence = new_value,
            StatType::WIS => self.wisdom = new_value,
            StatType::CHA => self.charisma = new_value,
        }
    }
}

impl Default for Stats {
    fn default() -> Self {
        Self::new()
    }
}
