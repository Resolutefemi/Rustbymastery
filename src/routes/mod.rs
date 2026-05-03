use axum::Router;
use sqlx::PgPool;

pub mod auth;
pub mod users;

pub fn create_router() -> Router<PgPool> {
    Router::new()
        .nest("/auth", auth::router())
        .nest("/users", users::router())
}
