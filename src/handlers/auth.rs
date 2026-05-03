use axum::{
    extract::State,
    Json,
};
use sqlx::PgPool;
use crate::dto::auth::{RegisterRequest, LoginRequest, AuthResponse};
use crate::error::{AppError, AppResult};
use crate::models::user::User;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use jsonwebtoken::{encode, Header, EncodingKey};
use serde::{Serialize, Deserialize};
use chrono::{Utc, Duration};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid,
    pub exp: usize,
    pub iat: usize,
}

pub async fn register(
    State(pool): State<PgPool>,
    Json(payload): Json<RegisterRequest>,
) -> AppResult<Json<AuthResponse>> {
    // 1. Check if user already exists
    let existing = sqlx::query!("SELECT id FROM users WHERE email = $1 OR username = $2", payload.email, payload.username)
        .fetch_optional(&pool)
        .await?;

    if existing.is_some() {
        return Err(AppError::Conflict("User with this email or username already exists".to_string()));
    }

    // 2. Hash password
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(payload.password.as_bytes(), &salt)
        .map_err(|_| AppError::Internal)?
        .to_string();

    // 3. Create user
    let referral_code = payload.username.to_lowercase();
    
    let user = sqlx::query_as!(
        User,
        r#"
        INSERT INTO users (full_name, username, email, password_hash, country_iso, referral_code)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING *
        "#,
        payload.full_name,
        payload.username,
        payload.email,
        Some(password_hash),
        "NG", // Default for now, should come from frontend
        referral_code
    )
    .fetch_one(&pool)
    .await?;

    // 4. Generate token
    let token = generate_token(user.id)?;

    Ok(Json(AuthResponse { token, user }))
}

pub async fn login(
    State(pool): State<PgPool>,
    Json(payload): Json<LoginRequest>,
) -> AppResult<Json<AuthResponse>> {
    // 1. Find user
    let user = sqlx::query_as!(
        User,
        "SELECT * FROM users WHERE email = $1",
        payload.email
    )
    .fetch_optional(&pool)
    .await?
    .ok_or(AppError::Unauthorized)?;

    // 2. Verify password
    let hash = user.password_hash.as_ref().ok_or(AppError::Unauthorized)?;
    let parsed_hash = PasswordHash::new(hash).map_err(|_| AppError::Internal)?;
    
    Argon2::default()
        .verify_password(payload.password.as_bytes(), &parsed_hash)
        .map_err(|_| AppError::Unauthorized)?;

    // 3. Generate token
    let token = generate_token(user.id)?;

    Ok(Json(AuthResponse { token, user }))
}

fn generate_token(user_id: Uuid) -> AppResult<String> {
    let jwt_secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "secret".to_string());
    let expiration = Utc::now()
        .checked_add_signed(Duration::days(7))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        sub: user_id,
        iat: Utc::now().timestamp() as usize,
        exp: expiration as usize,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret.as_bytes()),
    )
    .map_err(|_| AppError::Internal)
}
