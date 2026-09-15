use axum::{body::Bytes, extract::State, http::HeaderMap};
use reqwest::StatusCode;
use serde_json::json;

use crate::{
    GenericAppState,
    category::repo::CategoryRepository,
    error::route::Error,
    tag::repo::TagRepository,
    task::repo::TaskRepository,
    types::{
        extract::{Json, Session},
        response::Response,
    },
    user::{
        repo::UserRepository,
        service::UserService,
        types::route::{ProvisionRequest, UpdateRequest},
    },
    utils::route::verify_webhook_signature,
};

pub async fn provision_handler<U, T, Ta, C>(
    State(app_state): State<GenericAppState<U, T, Ta, C>>,
    headers: HeaderMap,
    body: Bytes,
) -> Result<Response, Error>
where
    U: UserRepository,
    T: TaskRepository,
    Ta: TagRepository,
    C: CategoryRepository,
{
    verify_webhook_signature(&app_state.config.webhook_secret, &headers, &body)?;

    let user_service = app_state.user_service;

    let create_request: ProvisionRequest = serde_json::from_slice(&body)
        .map_err(|_| Error::BadRequest("invalid request body".to_string()))?;
    let user = user_service.create(create_request).await?;
    Ok(Response::new(StatusCode::CREATED).data(json!(user)))
}

pub async fn get_handler<U>(
    session: Session,
    State(user_service): State<UserService<U>>,
) -> Result<Response, Error>
where
    U: UserRepository,
{
    match user_service.get(session.user_id).await {
        Ok(user) => Ok(Response::new(StatusCode::OK).data(json!(user))),
        Err(err) => Err(err.into()),
    }
}

pub async fn update_handler<U>(
    session: Session,
    State(user_service): State<UserService<U>>,
    Json(update_request): Json<UpdateRequest>,
) -> Result<Response, Error>
where
    U: UserRepository,
{
    let user = user_service.update(session.user_id, update_request).await?;

    Ok(Response::new(StatusCode::OK).data(json!(user)))
}
