use serde::{Deserialize, Serialize};
use crate::progression::RewardTier;
use anyhow::{Result, Context};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reward {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub cost: u32,
    pub tier: RewardTier,
    pub cooldown_hours: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_purchased: Option<String>, // ISO 8601 datetime
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RewardStore {
    pub rewards: Vec<Reward>,
    pub next_id: u32,
}

impl RewardStore {
    pub fn new() -> Self {
        Self {
            rewards: Self::default_rewards(),
            next_id: 11, // Start after the 10 default rewards
        }
    }

    fn default_rewards() -> Vec<Reward> {
        vec![
            Reward {
                id: 1,
                name: "Coffee Break".to_string(),
                description: "Enjoy a 15-minute coffee break".to_string(),
                cost: 50,
                tier: RewardTier::Normal,
                cooldown_hours: 0,
                last_purchased: None,
            },
            Reward {
                id: 2,
                name: "Gaming Session".to_string(),
                description: "30 minutes of guilt-free gaming".to_string(),
                cost: 100,
                tier: RewardTier::Normal,
                cooldown_hours: 0,
                last_purchased: None,
            },
            Reward {
                id: 3,
                name: "Movie Night".to_string(),
                description: "Watch a movie of your choice".to_string(),
                cost: 150,
                tier: RewardTier::Heroic,
                cooldown_hours: 0,
                last_purchased: None,
            },
            Reward {
                id: 4,
                name: "Treat Meal".to_string(),
                description: "Order your favorite takeout".to_string(),
                cost: 200,
                tier: RewardTier::Heroic,
                cooldown_hours: 0,
                last_purchased: None,
            },
            Reward {
                id: 5,
                name: "New Book".to_string(),
                description: "Buy that book you've been eyeing".to_string(),
                cost: 300,
                tier: RewardTier::Heroic,
                cooldown_hours: 0,
                last_purchased: None,
            },
            Reward {
                id: 6,
                name: "Day Off".to_string(),
                description: "Take a guilt-free rest day".to_string(),
                cost: 500,
                tier: RewardTier::Epic,
                cooldown_hours: 168, // 1 week
                last_purchased: None,
            },
            Reward {
                id: 7,
                name: "Hobby Supplies".to_string(),
                description: "Buy supplies for your hobby".to_string(),
                cost: 400,
                tier: RewardTier::Epic,
                cooldown_hours: 0,
                last_purchased: None,
            },
            Reward {
                id: 8,
                name: "Weekend Adventure".to_string(),
                description: "Plan a day trip or adventure".to_string(),
                cost: 750,
                tier: RewardTier::Epic,
                cooldown_hours: 336, // 2 weeks
                last_purchased: None,
            },
            Reward {
                id: 9,
                name: "Major Purchase".to_string(),
                description: "Buy that expensive item you want".to_string(),
                cost: 1500,
                tier: RewardTier::Legendary,
                cooldown_hours: 0,
                last_purchased: None,
            },
            Reward {
                id: 10,
                name: "Epic Reward".to_string(),
                description: "Your ultimate reward - define it yourself!".to_string(),
                cost: 3000,
                tier: RewardTier::Legendary,
                cooldown_hours: 720, // 30 days
                last_purchased: None,
            },
        ]
    }

    pub fn load(path: &Path) -> Result<Self> {
        if !path.exists() {
            return Ok(Self::new());
        }

        let file = std::fs::File::open(path)
            .context("Failed to open shop file")?;
        let store: RewardStore = serde_json::from_reader(file)
            .context("Failed to parse shop JSON")?;
        Ok(store)
    }

    pub fn save(&self, path: &Path) -> Result<()> {
        crate::storage::safe_write(path, self)
    }

    pub fn get_reward(&self, id: u32) -> Option<&Reward> {
        self.rewards.iter().find(|r| r.id == id)
    }

    pub fn get_reward_by_name(&self, name: &str) -> Option<&Reward> {
        self.rewards.iter()
            .find(|r| r.name.to_lowercase() == name.to_lowercase())
    }

    pub fn add_reward(&mut self, name: String, cost: u32, description: String, tier: RewardTier, cooldown_hours: u32) -> u32 {
        let id = self.next_id;
        self.next_id += 1;

        self.rewards.push(Reward {
            id,
            name,
            description,
            cost,
            tier,
            cooldown_hours,
            last_purchased: None,
        });

        id
    }

    pub fn remove_reward(&mut self, id: u32) -> Result<()> {
        let index = self.rewards.iter().position(|r| r.id == id)
            .context("Reward not found")?;

        // Don't allow removing default rewards (1-10)
        if id <= 10 {
            anyhow::bail!("Cannot remove default rewards");
        }

        self.rewards.remove(index);
        Ok(())
    }

    pub fn purchase_reward(&mut self, id: u32, gold: u32) -> Result<u32> {
        let reward = self.rewards.iter_mut()
            .find(|r| r.id == id)
            .context("Reward not found")?;

        // Check if enough gold
        if gold < reward.cost {
            anyhow::bail!("Not enough gold! Need {} but have {}", reward.cost, gold);
        }

        // Check cooldown
        if let Some(ref last) = reward.last_purchased {
            if reward.cooldown_hours > 0 {
                let last_time = chrono::DateTime::parse_from_rfc3339(last)
                    .context("Invalid last_purchased datetime")?;
                let now = chrono::Utc::now();
                let elapsed = now.signed_duration_since(last_time);
                let cooldown_duration = chrono::Duration::hours(reward.cooldown_hours as i64);

                if elapsed < cooldown_duration {
                    let remaining = cooldown_duration - elapsed;
                    let hours = remaining.num_hours();
                    let days = hours / 24;
                    if days > 0 {
                        anyhow::bail!("Reward on cooldown! {} days remaining", days);
                    } else {
                        anyhow::bail!("Reward on cooldown! {} hours remaining", hours);
                    }
                }
            }
        }

        // Update last purchased time
        reward.last_purchased = Some(chrono::Utc::now().to_rfc3339());

        Ok(reward.cost)
    }

    pub fn available_rewards(&self) -> Vec<&Reward> {
        self.rewards.iter()
            .filter(|r| {
                if let Some(ref last) = r.last_purchased {
                    if r.cooldown_hours > 0 {
                        if let Ok(last_time) = chrono::DateTime::parse_from_rfc3339(last) {
                            let now = chrono::Utc::now();
                            let elapsed = now.signed_duration_since(last_time);
                            let cooldown_duration = chrono::Duration::hours(r.cooldown_hours as i64);
                            return elapsed >= cooldown_duration;
                        }
                    }
                }
                true
            })
            .collect()
    }
}

impl Default for RewardStore {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_rewards() {
        let store = RewardStore::new();
        assert_eq!(store.rewards.len(), 10);
        assert_eq!(store.next_id, 11);
    }

    #[test]
    fn test_add_reward() {
        let mut store = RewardStore::new();
        let id = store.add_reward(
            "Test Reward".to_string(),
            250,
            "A test reward".to_string(),
            RewardTier::Normal,
        );
        assert_eq!(id, 11);
        assert_eq!(store.rewards.len(), 11);
    }

    #[test]
    fn test_purchase_reward() {
        let mut store = RewardStore::new();

        // Successful purchase
        let cost = store.purchase_reward(1, 100).unwrap();
        assert_eq!(cost, 50);

        // Not enough gold
        assert!(store.purchase_reward(10, 100).is_err());
    }
}
