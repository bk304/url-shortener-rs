use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct UrlModel {
    pub token: String,
    pub original_url: String,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
}
