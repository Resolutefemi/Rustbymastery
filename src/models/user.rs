use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: Uuid,
    pub full_name: String,
    pub username: String,
    pub email: String,
    pub password_hash: Option<String>,
    pub avatar_url: Option<String>,
    pub bio: Option<String>,
    pub country_iso: String,
    pub github_id: Option<String>,
    pub google_id: Option<String>,
    pub gitlab_id: Option<String>,
    pub github_username: Option<String>,
    pub current_rank: String,
    pub xp_total: i32,
    pub ren_coin_balance: i32,
    pub ren_coin_total_earned: i32,
    pub streak_current: i32,
    pub streak_longest: i32,
    pub last_active_at: Option<DateTime<Utc>>,
    pub email_verified: bool,
    pub onboarding_completed: bool,
    pub referral_code: String,
    pub referred_by_user_id: Option<Uuid>,
    pub how_heard: Option<String>,
    pub push_token: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
