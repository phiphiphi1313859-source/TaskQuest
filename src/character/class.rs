use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum Class {
    Rogue,   // Cunning, agile
    Ranger,  // Skilled, versatile
    Warrior, // Strong, durable
    Paladin, // Righteous, balanced
    Monk,    // Disciplined, wise
}

impl Class {
    pub fn description(&self) -> &'static str {
        match self {
            Class::Rogue => "Cunning and agile",
            Class::Ranger => "Skilled and versatile",
            Class::Warrior => "Strong and durable",
            Class::Paladin => "Righteous and balanced",
            Class::Monk => "Disciplined and wise",
        }
    }

    pub fn all() -> &'static [Class] {
        &[
            Class::Rogue,
            Class::Ranger,
            Class::Warrior,
            Class::Paladin,
            Class::Monk,
        ]
    }
}

impl std::fmt::Display for Class {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Class::Rogue => write!(f, "Rogue"),
            Class::Ranger => write!(f, "Ranger"),
            Class::Warrior => write!(f, "Warrior"),
            Class::Paladin => write!(f, "Paladin"),
            Class::Monk => write!(f, "Monk"),
        }
    }
}

impl std::str::FromStr for Class {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "rogue" => Ok(Class::Rogue),
            "ranger" => Ok(Class::Ranger),
            "warrior" => Ok(Class::Warrior),
            "paladin" => Ok(Class::Paladin),
            "monk" => Ok(Class::Monk),
            _ => Err(anyhow::anyhow!("Invalid class: {}", s)),
        }
    }
}
