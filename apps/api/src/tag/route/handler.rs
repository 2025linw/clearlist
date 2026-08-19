use axum::{extract::State, http::StatusCode};
use serde_json::json;

use crate::{
    GenericAppState,
    error::route::Error,
    tag::{
        repo::TagRepository,
        types::{
            TagID,
            route::{CreateRequest, URLQueryOpts, UpdateRequest},
        },
    },
    task::repo::TaskRepository,
    types::{
        extract::{Json, Path, Query, UserContext},
        response::Response,
    },
    user::repo::UserRepository,
};

pub async fn list_handler<U, T, Ta>(
    user_context: UserContext,
    State(app_state): State<GenericAppState<U, T, Ta>>,
    Query(query_opts): Query<URLQueryOpts>,
) -> Result<Response, Error>
where
    U: UserRepository,
    T: TaskRepository,
    Ta: TagRepository,
{
    let tag_service = app_state.tag_service;

    let tags = tag_service.list(user_context, Some(query_opts)).await?;

    Ok(Response::new(StatusCode::OK).data(json!({
        "count": tags.len(),
        "tags": tags,
    })))
}

pub async fn create_handler<U, T, Ta>(
    user_context: UserContext,
    State(app_state): State<GenericAppState<U, T, Ta>>,
    Json(create_request): Json<CreateRequest>,
) -> Result<Response, Error>
where
    U: UserRepository,
    T: TaskRepository,
    Ta: TagRepository,
{
    let tag_service = app_state.tag_service;

    let tag = tag_service.create(user_context, create_request).await?;

    Ok(Response::new(StatusCode::CREATED).data(json!(tag)))
}

pub async fn get_handler<U, T, Ta>(
    user_context: UserContext,
    State(app_state): State<GenericAppState<U, T, Ta>>,
    Path(tag_id): Path<TagID>,
) -> Result<Response, Error>
where
    U: UserRepository,
    T: TaskRepository,
    Ta: TagRepository,
{
    let tag_service = app_state.tag_service;

    match tag_service.get(tag_id, user_context).await {
        Ok(tag) => Ok(Response::new(StatusCode::OK).data(json!(tag))),
        Err(err) => Err(err.into()),
    }
}

pub async fn update_handler<U, T, Ta>(
    user_context: UserContext,
    State(app_state): State<GenericAppState<U, T, Ta>>,
    Path(tag_id): Path<TagID>,
    Json(update_request): Json<UpdateRequest>,
) -> Result<Response, Error>
where
    U: UserRepository,
    T: TaskRepository,
    Ta: TagRepository,
{
    let tag_service = app_state.tag_service;

    let tag = tag_service
        .update(tag_id, user_context, update_request)
        .await?;

    Ok(Response::new(StatusCode::OK).data(json!(tag)))
}

pub async fn delete_handler<U, T, Ta>(
    user_context: UserContext,
    State(app_state): State<GenericAppState<U, T, Ta>>,
    Path(tag_id): Path<TagID>,
) -> Result<Response, Error>
where
    U: UserRepository,
    T: TaskRepository,
    Ta: TagRepository,
{
    let tag_service = app_state.tag_service;

    tag_service.delete(tag_id, user_context).await?;

    Ok(Response::new(StatusCode::NO_CONTENT))
}
