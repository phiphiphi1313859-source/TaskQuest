use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum AchievementTier {
    Common,
    Uncommon,
    Rare,
    Epic,
    Legendary,
}

impl AchievementTier {
    pub fn color_code(&self) -> &'static str {
        match self {
            AchievementTier::Common => "\x1b[37m",      // White
            AchievementTier::Uncommon => "\x1b[92m",    // Bright Green
            AchievementTier::Rare => "\x1b[94m",        // Bright Blue
            AchievementTier::Epic => "\x1b[95m",        // Bright Magenta
            AchievementTier::Legendary => "\x1b[33m",   // Yellow/Gold
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            AchievementTier::Common => "Common",
            AchievementTier::Uncommon => "Uncommon",
            AchievementTier::Rare => "Rare",
            AchievementTier::Epic => "Epic",
            AchievementTier::Legendary => "Legendary",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Achievement {
    pub id: &'static str,
    pub title: &'static str,
    pub description: &'static str,
    pub tier: AchievementTier,
    pub icon: &'static str,
}

// All 25+ achievements as defined in the spec
pub const ACHIEVEMENTS: &[Achievement] = &[
    // ===== MILESTONE ACHIEVEMENTS =====
    Achievement {
        id: "first_steps",
        title: "First Steps",
        description: "Complete your first quest",
        tier: AchievementTier::Common,
        icon: "⚔️",
    },
    Achievement {
        id: "journey_begins",
        title: "The Journey Begins",
        description: "Reach level 5",
        tier: AchievementTier::Common,
        icon: "🎯",
    },
    Achievement {
        id: "seasoned_adventurer",
        title: "Seasoned Adventurer",
        description: "Complete 100 quests",
        tier: AchievementTier::Uncommon,
        icon: "🏆",
    },
    Achievement {
        id: "veteran_hero",
        title: "Veteran Hero",
        description: "Complete 500 quests",
        tier: AchievementTier::Rare,
        icon: "👑",
    },
    Achievement {
        id: "legendary_warrior",
        title: "Legendary Warrior",
        description: "Reach level 30",
        tier: AchievementTier::Epic,
        icon: "⚡",
    },
    Achievement {
        id: "living_legend",
        title: "Living Legend",
        description: "Reach level 50",
        tier: AchievementTier::Legendary,
        icon: "💫",
    },
    Achievement {
        id: "completionist",
        title: "Completionist",
        description: "Complete 1000 quests",
        tier: AchievementTier::Epic,
        icon: "📜",
    },

    // ===== VARIETY ACHIEVEMENTS =====
    Achievement {
        id: "master_of_balance",
        title: "Master of Balance",
        description: "Complete at least one task of each difficulty (1-10)",
        tier: AchievementTier::Uncommon,
        icon: "⚖️",
    },
    Achievement {
        id: "the_undaunted",
        title: "The Undaunted",
        description: "Complete 10 difficulty-10 quests",
        tier: AchievementTier::Rare,
        icon: "💪",
    },
    Achievement {
        id: "jack_of_all_trades",
        title: "Jack of All Trades",
        description: "Complete at least 20 tasks of each difficulty",
        tier: AchievementTier::Epic,
        icon: "🎭",
    },

    // ===== LONG-TERM PROGRESS ACHIEVEMENTS =====
    Achievement {
        id: "consistent_hero",
        title: "Consistent Hero",
        description: "Be active for 30 different days (non-consecutive)",
        tier: AchievementTier::Uncommon,
        icon: "📅",
    },
    Achievement {
        id: "marathon_hero",
        title: "Marathon Hero",
        description: "Be active for 100 different days",
        tier: AchievementTier::Epic,
        icon: "🏃",
    },
    Achievement {
        id: "renaissance_soul",
        title: "Renaissance Soul",
        description: "Complete tasks in 10 different projects",
        tier: AchievementTier::Uncommon,
        icon: "🎨",
    },

    // ===== COMEBACK ACHIEVEMENTS =====
    Achievement {
        id: "phoenix_rising",
        title: "Phoenix Rising",
        description: "Complete 5 quests after a 30+ day break",
        tier: AchievementTier::Rare,
        icon: "🔥",
    },

    // ===== LOOT & REWARDS ACHIEVEMENTS =====
    Achievement {
        id: "epic_collector",
        title: "Epic Collector",
        description: "Receive an Epic-tier loot drop",
        tier: AchievementTier::Rare,
        icon: "💎",
    },
    Achievement {
        id: "gold_hoarder",
        title: "Gold Hoarder",
        description: "Accumulate 5000 gold",
        tier: AchievementTier::Rare,
        icon: "💰",
    },
    Achievement {
        id: "wise_spender",
        title: "Wise Spender",
        description: "Purchase 10 rewards from the shop",
        tier: AchievementTier::Uncommon,
        icon: "🛒",
    },
    Achievement {
        id: "treasure_hunter",
        title: "Treasure Hunter",
        description: "Receive 50 loot drops",
        tier: AchievementTier::Uncommon,
        icon: "🗝️",
    },

    // ===== TIMING ACHIEVEMENTS =====
    Achievement {
        id: "early_riser",
        title: "Early Riser",
        description: "Complete 50 tasks early (>24hrs before due)",
        tier: AchievementTier::Uncommon,
        icon: "🌅",
    },
    Achievement {
        id: "pressure_handler",
        title: "Pressure Handler",
        description: "Complete 25 tasks within grace period",
        tier: AchievementTier::Uncommon,
        icon: "⏰",
    },
    Achievement {
        id: "time_master",
        title: "Time Master",
        description: "Complete 100 tasks with a due date set",
        tier: AchievementTier::Uncommon,
        icon: "⌚",
    },
    Achievement {
        id: "punctual_perfectionist",
        title: "Punctual Perfectionist",
        description: "Complete 100 tasks on time (no early/late)",
        tier: AchievementTier::Rare,
        icon: "🎯",
    },

    // ===== POWER ACHIEVEMENTS =====
    Achievement {
        id: "strength_incarnate",
        title: "Strength Incarnate",
        description: "Reach 100 STR",
        tier: AchievementTier::Uncommon,
        icon: "💪",
    },
    Achievement {
        id: "lightning_reflexes",
        title: "Lightning Reflexes",
        description: "Reach 100 DEX",
        tier: AchievementTier::Uncommon,
        icon: "⚡",
    },
    Achievement {
        id: "iron_constitution",
        title: "Iron Constitution",
        description: "Reach 100 CON",
        tier: AchievementTier::Uncommon,
        icon: "🛡️",
    },
    Achievement {
        id: "brilliant_mind",
        title: "Brilliant Mind",
        description: "Reach 100 INT",
        tier: AchievementTier::Uncommon,
        icon: "🧠",
    },
    Achievement {
        id: "sage_wisdom",
        title: "Sage Wisdom",
        description: "Reach 100 WIS",
        tier: AchievementTier::Uncommon,
        icon: "📚",
    },
    Achievement {
        id: "magnetic_personality",
        title: "Magnetic Personality",
        description: "Reach 100 CHA",
        tier: AchievementTier::Uncommon,
        icon: "✨",
    },

    // ===== LEGENDARY STAT ACHIEVEMENTS =====
    Achievement {
        id: "legendary_strength",
        title: "Legendary Strength",
        description: "Reach 500 in any stat",
        tier: AchievementTier::Epic,
        icon: "🌟",
    },
    Achievement {
        id: "transcendent_power",
        title: "Transcendent Power",
        description: "Reach 1000 in any stat",
        tier: AchievementTier::Legendary,
        icon: "👑",
    },
];

impl Achievement {
    pub fn get_by_id(id: &str) -> Option<&'static Achievement> {
        ACHIEVEMENTS.iter().find(|a| a.id == id)
    }

    pub fn all() -> &'static [Achievement] {
        ACHIEVEMENTS
    }
}
