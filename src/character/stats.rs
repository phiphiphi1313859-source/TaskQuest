use serde::{Deserialize, Serialize};
use super::StatType;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stats {
    pub strength: u16,     // Physical power
    pub dexterity: u16,    // Agility, reflexes
    pub constitution: u16, // Endurance, health
    pub intelligence: u16, // Logic, memory
    pub wisdom: u16,       // Perception, insight
    pub charisma: u16,     // Force of personality
}

impl Stats {
    pub fn new() -> Self {
        Self {
            strength: 10,
            dexterity: 10,
            constitution: 10,
            intelligence: 10,
            wisdom: 10,
            charisma: 10,
        }
    }

    pub fn get_stat(&self, stat_type: StatType) -> u16 {
        match stat_type {
            StatType::STR => self.strength,
            StatType::DEX => self.dexterity,
            StatType::CON => self.constitution,
            StatType::INT => self.intelligence,
            StatType::WIS => self.wisdom,
            StatType::CHA => self.charisma,
        }
    }

    pub fn increase_stat(&mut self, stat_type: StatType, amount: u16) {
        match stat_type {
            StatType::STR => self.strength += amount,
            StatType::DEX => self.dexterity += amount,
            StatType::CON => self.constitution += amount,
            StatType::INT => self.intelligence += amount,
            StatType::WIS => self.wisdom += amount,
            StatType::CHA => self.charisma += amount,
        }
    }
}

impl Default for Stats {
    fn default() -> Self {
        Self::new()
    }
}
