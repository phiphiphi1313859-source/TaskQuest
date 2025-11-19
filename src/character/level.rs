/// Level system based on WoW-inspired curve
/// XP required for level N: 100 * (N ^ 2.1)
pub struct LevelSystem;

impl LevelSystem {
    /// Calculate XP required to reach a specific level
    pub fn xp_for_level(level: u32) -> u32 {
        if level == 1 {
            return 0;
        }
        (100.0 * (level as f64).powf(2.1)) as u32
    }

    /// Calculate current level from total XP
    pub fn level_from_xp(total_xp: u32) -> u32 {
        let mut level = 1;
        while total_xp >= Self::xp_for_level(level + 1) {
            level += 1;
        }
        level
    }

    /// Get XP needed for next level from current XP
    pub fn xp_to_next_level(total_xp: u32) -> u32 {
        let current_level = Self::level_from_xp(total_xp);
        let next_level_xp = Self::xp_for_level(current_level + 1);
        next_level_xp - total_xp
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_level_progression() {
        assert_eq!(LevelSystem::xp_for_level(1), 0);
        assert_eq!(LevelSystem::xp_for_level(2), 121);
        assert_eq!(LevelSystem::xp_for_level(3), 483);
        assert_eq!(LevelSystem::xp_for_level(5), 2023);
    }

    #[test]
    fn test_level_from_xp() {
        assert_eq!(LevelSystem::level_from_xp(0), 1);
        assert_eq!(LevelSystem::level_from_xp(120), 1);
        assert_eq!(LevelSystem::level_from_xp(121), 2);
        assert_eq!(LevelSystem::level_from_xp(500), 3);
    }
}
