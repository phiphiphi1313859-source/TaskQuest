use rand::Rng;

pub struct GoldCalculator;

impl GoldCalculator {
    /// Calculate gold reward for a completed task
    ///
    /// # Arguments
    /// * `challenge` - Task challenge rating (1-10)
    ///
    /// # Returns
    /// Gold amount with ±20% variance
    pub fn calculate(challenge: u8) -> u32 {
        // Base: challenge * 5
        let base = (challenge as u32) * 5;

        // Random variance: ±20%
        let variance = (base as f64 * 0.2) as u32;
        let min = base.saturating_sub(variance);
        let max = base + variance;

        rand::thread_rng().gen_range(min..=max)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gold_calculation() {
        // Challenge 1: 4-6 gold
        for _ in 0..100 {
            let gold = GoldCalculator::calculate(1);
            assert!(gold >= 4 && gold <= 6);
        }

        // Challenge 5: 20-30 gold
        for _ in 0..100 {
            let gold = GoldCalculator::calculate(5);
            assert!(gold >= 20 && gold <= 30);
        }

        // Challenge 10: 40-60 gold
        for _ in 0..100 {
            let gold = GoldCalculator::calculate(10);
            assert!(gold >= 40 && gold <= 60);
        }
    }
}
