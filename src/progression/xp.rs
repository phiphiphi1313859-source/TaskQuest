use chrono::{DateTime, Utc, Duration};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskTiming {
    Early,       // >24hrs before due
    OnTime,      // Day of due date
    GracePeriod, // <24hrs late
    Late,        // >24hrs late
    NoDueDate,   // No penalty
}

pub struct XPCalculator;

impl XPCalculator {
    /// Calculate XP for a completed task
    ///
    /// # Arguments
    /// * `challenge` - Task challenge rating (1-10)
    /// * `urgency` - Taskwarrior's urgency score
    /// * `timing` - When the task was completed relative to due date
    pub fn calculate(challenge: u8, urgency: f64, timing: TaskTiming) -> u32 {
        // Base XP scales with challenge
        let base_xp = (challenge as u32) * 10;

        // Urgency modifier (1.0 to 1.5x)
        let urgency_multiplier = 1.0 + (urgency * 0.5).min(0.5);

        // Timing bonus/penalty
        let timing_multiplier = match timing {
            TaskTiming::Early => 1.3,       // >24hrs before due
            TaskTiming::OnTime => 1.0,      // Day of due date
            TaskTiming::GracePeriod => 0.8, // <24hrs late
            TaskTiming::Late => 0.5,        // >24hrs late
            TaskTiming::NoDueDate => 1.0,   // No penalty
        };

        ((base_xp as f64) * urgency_multiplier * timing_multiplier) as u32
    }

    /// Determine task timing based on due date and completion time
    pub fn determine_timing(
        due_date: Option<DateTime<Utc>>,
        completed_at: DateTime<Utc>,
    ) -> TaskTiming {
        let Some(due) = due_date else {
            return TaskTiming::NoDueDate;
        };

        let difference = completed_at.signed_duration_since(due);

        if difference < Duration::hours(-24) {
            TaskTiming::Early
        } else if difference < Duration::hours(0) {
            TaskTiming::OnTime
        } else if difference < Duration::hours(24) {
            TaskTiming::GracePeriod
        } else {
            TaskTiming::Late
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xp_calculation() {
        // Challenge 5, medium urgency, on time: ~75 XP
        let xp = XPCalculator::calculate(5, 0.5, TaskTiming::OnTime);
        assert!(xp >= 62 && xp <= 88); // 50 * 1.25 = 62.5, allowing for rounding

        // Challenge 10, high urgency, early: ~195 XP
        let xp = XPCalculator::calculate(10, 1.0, TaskTiming::Early);
        assert!(xp >= 180 && xp <= 210);

        // Challenge 3, low urgency, late: ~15 XP
        let xp = XPCalculator::calculate(3, 0.1, TaskTiming::Late);
        assert!(xp >= 10 && xp <= 20);
    }

    #[test]
    fn test_timing_determination() {
        let now = Utc::now();

        // Early: >24hrs before
        let due = now + Duration::hours(48);
        assert!(matches!(
            XPCalculator::determine_timing(Some(due), now),
            TaskTiming::Early
        ));

        // On time: within 24hrs before due
        let due = now + Duration::hours(12);
        assert!(matches!(
            XPCalculator::determine_timing(Some(due), now),
            TaskTiming::OnTime
        ));

        // Grace period: <24hrs late
        let due = now - Duration::hours(12);
        assert!(matches!(
            XPCalculator::determine_timing(Some(due), now),
            TaskTiming::GracePeriod
        ));

        // Late: >24hrs late
        let due = now - Duration::hours(48);
        assert!(matches!(
            XPCalculator::determine_timing(Some(due), now),
            TaskTiming::Late
        ));

        // No due date
        assert!(matches!(
            XPCalculator::determine_timing(None, now),
            TaskTiming::NoDueDate
        ));
    }
}
