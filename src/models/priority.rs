use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "priority_level")]
#[serde(rename_all = "lowercase")]
pub enum Priority {
    #[sqlx(rename = "low")]
    Low,
    #[sqlx(rename = "medium")]
    Medium,
    #[sqlx(rename = "high")]
    High,
    // Keep only if your DB enum includes it; otherwise remove or add a migration first.
    #[sqlx(rename = "urgent")]
    Urgent,
}

impl Default for Priority {
    fn default() -> Self { Priority::Medium }
}
