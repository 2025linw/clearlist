//! # Routes Module
//!
//! This module contains all routing functions and handlers

pub mod models;

mod error;
mod util;

mod tag;
mod task;

pub use error::Error;

use std::sync::Arc;

use axum::{
    Router, extract::State, http::StatusCode, response::IntoResponse, routing::{get, patch}
};
use serde_json::json;
use tower_governor::GovernorLayer;

use crate::{AppState, models::user::Model as UserModel, response::{Response, UserResponse}, routes::util::UserSession, service::user::UserServiceTrait};

/// Create API router for all resources
///
/// Router is rate limited to 8 requests refreshing 1 every 1 second
pub fn create_api_router() -> Router<AppState> {
    // TOOD: replace create_resource_router with the actual ends
    let task_routes = Router::new()
        .route("/", get(task::query_handler).post(task::create_handler))
        .route(
            "/{task_id}",
            get(task::retrieve_handler)
                .put(task::update_handler)
                .delete(task::delete_handler),
        )
        .route("/{task_id}/restore", patch(task::restore_handler))
        .route("/{task_id}/complete", patch(task::complete_handler))
        .nest(
            "/{task_id}/tags",
            Router::new()
                .route(
                    "/",
                    get(task::tag::query_handler).put(task::tag::update_handler),
                )
                .route(
                    "/{tag_id}",
                    patch(task::tag::append_handler).delete(task::tag::delete_handler),
                ),
        );

    let tag_routes = Router::new()
        .route("/", get(tag::query_handler).post(tag::create_handler))
        .route(
            "/{tag_id}",
            get(tag::retrieve_handler)
                .put(tag::update_handler)
                .delete(tag::delete_handler),
        );

    Router::new()
        .route("/health", get(health_check_handler))
        .route("/me", get(me))
        .nest("/tasks", task_routes)
        .nest("/tags", tag_routes)
        .layer(GovernorLayer {
            config: Arc::new(util::create_rate_limiter(8, 1)),
        })
}

/// Handler for API health check
///
/// Responds with OK (200) and a message
pub async fn health_check_handler() -> impl IntoResponse {
    const MESSAGE: &str = "Todo List API Services";

    Response::new(StatusCode::OK)
        .message(MESSAGE)
        .add_kv("version", json!(env!("CARGO_PKG_VERSION")))
}

/// Handler for invalid routes
///
/// Responds with Not Found (404)
pub async fn missing_404_handler() -> impl IntoResponse {
    Response::new(StatusCode::NOT_FOUND).message("Endpoint not found")
}

pub async fn me(
    user_session: UserSession,
    State(data): State<AppState>,
) -> Result<Response, Error>  {
    let auth_user = user_session.user;

    let user = data.user_service.clone().get(auth_user.id).await?;

    let user = if let Some(user) = user { // existing user without app.users row
        user
    } else { // new user
        let user = UserModel {
            id: auth_user.id,
            display_name: auth_user.name,
            created_at: auth_user.created_at,
        };

        data.user_service.create(user).await?
    };

    Ok(Response::new(StatusCode::OK).data(json!(UserResponse::from(user))))
}
