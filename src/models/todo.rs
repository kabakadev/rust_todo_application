use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use super::priority::Priority;

/// Matches your `todos` table schema
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Todo {
    pub id: i64, // BIGINT in Postgres
    pub title: String,
    pub description: Option<String>,
    pub is_completed: bool,
    pub priority: Priority,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTodo {
    pub title: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)] // uses Priority::default() => Medium
    pub priority: Priority,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateTodo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<Priority>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_completed: Option<bool>,
}

impl CreateTodo {
    pub fn validate(&self) -> Result<(), String> {
        if self.title.trim().is_empty() {
            return Err("Title cannot be empty".into());
        }
        if self.title.len() > 200 {
            return Err("Title cannot exceed 200 characters".into());
        }
        if let Some(desc) = &self.description {
            if desc.len() > 1000 {
                return Err("Description cannot exceed 1000 characters".into());
            }
        }
        Ok(())
    }
}

impl UpdateTodo {
    pub fn validate(&self) -> Result<(), String> {
        if let Some(title) = &self.title {
            if title.trim().is_empty() {
                return Err("Title cannot be empty".into());
            }
            if title.len() > 200 {
                return Err("Title cannot exceed 200 characters".into());
            }
        }
        if let Some(desc) = &self.description {
            if desc.len() > 1000 {
                return Err("Description cannot exceed 1000 characters".into());
            }
        }
        Ok(())
    }

    /// True when no fields are provided (useful to short-circuit in handlers).
    pub fn is_all_none(&self) -> bool {
        self.title.is_none()
            && self.description.is_none()
            && self.priority.is_none()
            && self.is_completed.is_none()
    }

    /// Back-compat alias used elsewhere in your code.
    pub fn has_changes(&self) -> bool {
        !self.is_all_none()
    }
}
