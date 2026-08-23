pub mod error;
pub mod types;
pub mod utils;

// Resources
pub mod category;
pub mod tag;
pub mod task;
pub mod user;

#[cfg(test)]
mod tests;

use std::env;

use axum::{
    Router,
    http::StatusCode,
    routing::{get, post},
};
use reqwest::Url;
use serde_json::json;
use sqlx::PgPool;

use crate::{
    category::{
        repo::{CategoryRepository, PgCategoryRepository},
        route::create_router as create_category_router,
        service::CategoryService,
    },
    tag::{
        repo::{PgTagRepository, TagRepository},
        route::create_router as create_tag_router,
        service::TagService,
    },
    task::{
        repo::{PgTaskRepository, TaskRepository},
        route::create_router as create_task_router,
        service::TaskService,
    },
    types::route::Response,
    user::{
        repo::{PgUserRepository, UserRepository},
        route::create_router as create_user_router,
        service::UserService,
    },
};

#[derive(Debug, Clone)]
pub struct Config {
    cookie_key: String,
    auth_server_url: Url,
    webhook_secret: String,
}

impl Config {
    pub fn init(cookie_key: String, auth_server_url: Url, webhook_secret: String) -> Self {
        Self {
            cookie_key,
            auth_server_url,
            webhook_secret,
        }
    }

    pub fn from_env() -> Self {
        let cookie_key = env::var("COOKIE_KEY").unwrap_or("better-auth.session_token".to_string());
        let auth_server_url = Url::parse(
            &env::var("AUTH_SERVER_URL")
                .expect("AUTH_SERVER_URL not found in environment variables"),
        )
        .expect("AUTH_SERVER_URL should be a valid URL format");
        let webhook_secret =
            env::var("WEBHOOK_SECRET").expect("WEBHOOK_SECRET not found in environment variables");

        Self {
            cookie_key,
            auth_server_url,
            webhook_secret,
        }
    }
}

#[derive(Clone)]
pub struct GenericAppState<U, T, Ta, C>
where
    U: UserRepository,
    C: CategoryRepository,
    Ta: TagRepository,
    T: TaskRepository,
{
    config: Config,
    user_service: UserService<U>,
    category_service: CategoryService<C>,
    tag_service: TagService<Ta, C>,
    task_service: TaskService<T>,
}

pub type PgAppState =
    GenericAppState<PgUserRepository, PgTaskRepository, PgTagRepository, PgCategoryRepository>;

impl PgAppState {
    pub fn init(pool: PgPool, config: Config) -> Self {
        let user_service = UserService::init(PgUserRepository::init(pool.clone()));
        let category_service = CategoryService::init(PgCategoryRepository::init(pool.clone()));
        let tag_service = TagService::init(
            PgTagRepository::init(pool.clone()),
            PgCategoryRepository::init(pool.clone()),
        );
        let task_service = TaskService::init(PgTaskRepository::init(pool.clone()));

        Self {
            config,
            user_service,
            task_service,
            tag_service,
            category_service,
        }
    }
}

pub fn create_app(app_state: PgAppState) -> Router {
    // internal-use routes
    let internal_routes =
        Router::new().route("/users/provision", post(user::route::provision_handler));

    // user-facing routes
    let api_routes = Router::new()
        .route("/health", get(health_check_handler))
        .nest("/me", create_user_router())
        .nest("/tasks", create_task_router())
        .nest("/tags", create_tag_router())
        .nest("/categories", create_category_router());

    Router::new()
        .nest("/internal", internal_routes)
        .nest("/api", api_routes)
        .fallback(async || {
            Response::new(StatusCode::NOT_FOUND).message("Endpoint not found".to_string())
        })
        .with_state(app_state)
}

pub async fn health_check_handler() -> Response {
    const MESSAGE: &str = "Todo List API Services";

    Response::new(StatusCode::OK)
        .message(MESSAGE.to_string())
        .add_kv("version", json!(env!("CARGO_PKG_VERSION")))
}
