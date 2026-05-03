use axum::{
    routing::{get, put},
    Router,
};
use sqlx::PgPool;
use crate::handlers::users;

pub fn router() -> Router<PgPool> {
    Router::new()
        .route("/:username", get(users::get_profile))
        .route("/:id", put(users::update_profile))
}
