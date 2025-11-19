use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Deserializer, Serialize};

fn deserialize_challenge<'de, D>(deserializer: D) -> Result<Option<u8>, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de::Error;
    let value: Option<serde_json::Value> = Option::deserialize(deserializer)?;

    match value {
        None => Ok(None),
        Some(serde_json::Value::Number(n)) => {
            if let Some(i) = n.as_u64() {
                Ok(Some(i as u8))
            } else if let Some(f) = n.as_f64() {
                Ok(Some(f as u8))
            } else {
                Ok(None)
            }
        },
        Some(_) => Ok(None),
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskData {
    pub id: Option<u32>,
    pub uuid: String,
    pub status: String,
    pub description: String,
    pub urgency: Option<f64>,
    pub due: Option<String>,
    pub end: Option<String>,
    #[serde(deserialize_with = "deserialize_challenge")]
    pub challenge: Option<u8>,
    pub project: Option<String>,
    pub stat1: Option<String>,
    pub stat2: Option<String>,
}

impl TaskData {
    /// Parse task from JSON string
    pub fn from_json(json: &str) -> Result<Self> {
        serde_json::from_str(json).context("Failed to parse task JSON")
    }

    /// Get challenge level (defaults to 5 if not set)
    pub fn get_challenge(&self) -> u8 {
        self.challenge.unwrap_or(5).clamp(1, 10)
    }

    /// Get urgency score (defaults to 0.0 if not set)
    pub fn get_urgency(&self) -> f64 {
        self.urgency.unwrap_or(0.0)
    }

    /// Parse due date
    pub fn get_due_date(&self) -> Option<DateTime<Utc>> {
        self.due.as_ref()
            .and_then(|d| DateTime::parse_from_rfc3339(d).ok())
            .map(|dt| dt.with_timezone(&Utc))
    }

    /// Parse completion date
    pub fn get_completion_date(&self) -> Option<DateTime<Utc>> {
        self.end.as_ref()
            .and_then(|d| DateTime::parse_from_rfc3339(d).ok())
            .map(|dt| dt.with_timezone(&Utc))
    }

    /// Check if task was completed
    pub fn is_completed(&self) -> bool {
        self.status == "completed"
    }

    /// Get project name if set
    pub fn get_project(&self) -> Option<String> {
        self.project.clone()
    }

    /// Get stat1 as StatType
    pub fn get_stat1(&self) -> Option<crate::character::StatType> {
        self.stat1.as_ref()
            .and_then(|s| s.parse().ok())
    }

    /// Get stat2 as StatType
    pub fn get_stat2(&self) -> Option<crate::character::StatType> {
        self.stat2.as_ref()
            .and_then(|s| s.parse().ok())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_task() {
        let json = r#"{
            "id": 1,
            "uuid": "12345678-1234-1234-1234-123456789012",
            "status": "completed",
            "description": "Test task",
            "urgency": 5.5,
            "challenge": 7
        }"#;

        let task = TaskData::from_json(json).unwrap();
        assert_eq!(task.description, "Test task");
        assert_eq!(task.get_challenge(), 7);
        assert_eq!(task.get_urgency(), 5.5);
    }

    #[test]
    fn test_default_challenge() {
        let json = r#"{
            "uuid": "12345678-1234-1234-1234-123456789012",
            "status": "pending",
            "description": "Test task"
        }"#;

        let task = TaskData::from_json(json).unwrap();
        assert_eq!(task.get_challenge(), 5);
    }
}
