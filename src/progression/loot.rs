use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LootDrop {
    Gold(u32),
    Reward {
        tier: RewardTier,
        name: String,
    },
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum RewardTier {
    Normal,    // Color: white
    Heroic,    // Color: blue
    Epic,      // Color: purple
    Legendary, // Color: orange (not droppable)
}

impl RewardTier {
    pub fn color_code(&self) -> &'static str {
        match self {
            RewardTier::Normal => "\x1b[37m",    // White
            RewardTier::Heroic => "\x1b[94m",    // Bright Blue
            RewardTier::Epic => "\x1b[95m",      // Bright Magenta
            RewardTier::Legendary => "\x1b[33m", // Yellow/Gold
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            RewardTier::Normal => "Normal",
            RewardTier::Heroic => "Heroic",
            RewardTier::Epic => "Epic",
            RewardTier::Legendary => "Legendary",
        }
    }
}

pub struct LootSystem;

impl LootSystem {
    /// Roll for loot drop based on challenge level
    /// Base drop chance: 30% + (challenge * 2%)
    pub fn roll_for_loot(challenge: u8) -> Option<LootDrop> {
        let mut rng = rand::thread_rng();

        // Base drop chance: 30% + (challenge * 2%)
        let drop_chance = 0.30 + (challenge as f64 * 0.02);

        if rng.gen_bool(drop_chance.min(1.0)) {
            Some(Self::determine_loot_type())
        } else {
            None
        }
    }

    fn determine_loot_type() -> LootDrop {
        let mut rng = rand::thread_rng();
        let roll: f64 = rng.gen();

        match roll {
            r if r < 0.70 => {
                // 70% - Gold drop (10-50 gold)
                let amount = rng.gen_range(10..=50);
                LootDrop::Gold(amount)
            }
            r if r < 0.90 => {
                // 20% - Normal tier reward
                LootDrop::Reward {
                    tier: RewardTier::Normal,
                    name: Self::random_reward_name(RewardTier::Normal),
                }
            }
            r if r < 0.98 => {
                // 8% - Heroic tier reward
                LootDrop::Reward {
                    tier: RewardTier::Heroic,
                    name: Self::random_reward_name(RewardTier::Heroic),
                }
            }
            _ => {
                // 2% - Epic tier reward
                LootDrop::Reward {
                    tier: RewardTier::Epic,
                    name: Self::random_reward_name(RewardTier::Epic),
                }
            }
        }
    }

    fn random_reward_name(tier: RewardTier) -> String {
        let mut rng = rand::thread_rng();

        let rewards = match tier {
            RewardTier::Normal => vec![
                "Coffee Break",
                "Gaming Session",
                "Snack Time",
            ],
            RewardTier::Heroic => vec![
                "Movie Night",
                "Treat Meal",
                "New Book",
            ],
            RewardTier::Epic => vec![
                "Day Off",
                "Hobby Supplies",
                "Weekend Adventure",
            ],
            RewardTier::Legendary => vec![
                "Major Purchase",
                "Epic Reward",
            ],
        };

        rewards[rng.gen_range(0..rewards.len())].to_string()
    }

    /// Calculate drop chance for display
    pub fn drop_chance(challenge: u8) -> f64 {
        let base = 0.30 + (challenge as f64 * 0.02);
        (base * 100.0).min(100.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_drop_chances() {
        // Challenge 1: 32% drop chance
        assert!((LootSystem::drop_chance(1) - 32.0).abs() < 0.1);

        // Challenge 5: 40% drop chance
        assert!((LootSystem::drop_chance(5) - 40.0).abs() < 0.1);

        // Challenge 10: 50% drop chance
        assert!((LootSystem::drop_chance(10) - 50.0).abs() < 0.1);
    }

    #[test]
    fn test_loot_distribution() {
        let mut gold_count = 0;
        let mut normal_count = 0;
        let mut heroic_count = 0;
        let mut epic_count = 0;

        // Simulate 1000 drops
        for _ in 0..1000 {
            if let Some(loot) = LootSystem::roll_for_loot(10) {
                match loot {
                    LootDrop::Gold(_) => gold_count += 1,
                    LootDrop::Reward { tier, .. } => match tier {
                        RewardTier::Normal => normal_count += 1,
                        RewardTier::Heroic => heroic_count += 1,
                        RewardTier::Epic => epic_count += 1,
                        RewardTier::Legendary => {}
                    },
                }
            }
        }

        // Verify approximate distribution
        let total_drops = gold_count + normal_count + heroic_count + epic_count;
        assert!(gold_count > 0);
        assert!(total_drops > 400); // ~50% drop rate for challenge 10
    }
}
