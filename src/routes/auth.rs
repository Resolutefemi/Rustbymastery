use ax_auth::register;
use axum::{
    routing::post,
    Router,
};
use sqlx::PgPool;
use crate::handlers::auth;

pub fn router() -> Router<PgPool> {
    Router::new()
        .route("/register", post(auth::register))
        .route("/login", post(auth::login))
}
