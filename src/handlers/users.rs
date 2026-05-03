use axum::{
    extract::{Path, State},
    Json,
};
use sqlx::PgPool;
use uuid::Uuid;
use crate::error::{AppError, AppResult};
use crate::models::user::User;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct UpdateUserRequest {
    pub full_name: Option<String>,
    pub bio: Option<String>,
    pub github_username: Option<String>,
}

pub async fn get_profile(
    State(pool): State<PgPool>,
    Path(username): Path<String>,
) -> AppResult<Json<User>> {
    let user = sqlx::query_as!(
        User,
        "SELECT * FROM users WHERE username = $1",
        username
    )
    .fetch_optional(&pool)
    .await?
    .ok_or(AppError::NotFound("User not found".to_string()))?;

    Ok(Json(user))
}

pub async fn update_profile(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateUserRequest>,
) -> AppResult<Json<User>> {
    let user = sqlx::query_as!(
        User,
        r#"
        UPDATE users
        SET full_name = COALESCE($1, full_name),
            bio = COALESCE($2, bio),
            github_username = COALESCE($3, github_username)
        WHERE id = $4
        RETURNING *
        "#,
        payload.full_name,
        payload.bio,
        payload.github_username,
        id
    )
    .fetch_one(&pool)
    .await?;

    Ok(Json(user))
}
