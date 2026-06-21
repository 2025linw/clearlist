//! Clear List API Library
//!
//! This module is the main library used to create Clear List API web server

mod com;
mod models;
mod response;

mod db;
mod routes;
mod service;

use std::env;

pub use db::{DatabaseConn, run_migration};

use axum::Router;
use reqwest::Url;

use crate::{db::user::PgUserRepository, routes::missing_404_handler, service::user::UserService};

#[derive(Clone)]
pub struct Config {
    cookie_key: String,
    auth_server_url: Url,
}

impl Config {
    pub fn from_env() -> Self {
        let cookie_key = env::var("COOKIE_KEY").unwrap_or("better-auth.session_token".to_string());
        let auth_server_url = Url::parse(
            &env::var("AUTH_SERVER_URL").expect("AUTH_SERVER_URL must be in environment variables"),
        )
        .expect("AUTH_SERVER_URL should be a valid URL format");

        Self {
            cookie_key,
            auth_server_url,
        }
    }
}

/// App State Type
///
/// `AppState` is used for reused resources throughout web server (such as database connections, etc)
#[derive(Clone)]
pub struct AppState {
    config: Config,
    db: DatabaseConn,
    user_service: UserService<PgUserRepository>,
}

impl AppState {
    /// Initialize an AppState with a database connection given by DatabaseConn
    pub fn init(conn: DatabaseConn, config: Config) -> Self {
        let user_service = UserService::new(PgUserRepository::new(conn.pool().clone()));

        Self {
            config,
            db: conn,
            user_service,
        }
    }
}

/// Creates a new Router to be used as the app for Clear List API webserver
pub fn create_app(app_state: AppState) -> Router {
    Router::new()
        .nest("/api", routes::create_api_router())
        .fallback(missing_404_handler)
        .with_state(app_state)
}
