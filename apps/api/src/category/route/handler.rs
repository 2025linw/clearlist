use axum::{extract::State, http::StatusCode};
use serde_json::json;

use crate::{
    category::{
        repo::CategoryRepository,
        service::CategoryService,
        types::{
            CategoryID,
            route::{CreateRequest, UpdateRequest},
        },
    },
    error::route::Result,
    types::{
        extract::{Json, Path, UserContext},
        route::Response,
    },
};

pub async fn list_handler<C>(
    user_context: UserContext,
    State(category_service): State<CategoryService<C>>,
) -> Result<Response>
where
    C: CategoryRepository,
{
    let categories = category_service.list(user_context).await?;

    Ok(Response::new(StatusCode::OK).data(json!({
        "count": categories.len(),
        "categories": categories,
    })))
}

pub async fn create_handler<C>(
    user_context: UserContext,
    State(category_service): State<CategoryService<C>>,
    Json(create_request): Json<CreateRequest>,
) -> Result<Response>
where
    C: CategoryRepository,
{
    let category = category_service
        .create(user_context, create_request)
        .await?;

    Ok(Response::new(StatusCode::CREATED).data(json!(category)))
}

pub async fn get_handler<C>(
    user_context: UserContext,
    State(category_service): State<CategoryService<C>>,
    Path(category_id): Path<CategoryID>,
) -> Result<Response>
where
    C: CategoryRepository,
{
    match category_service.get(category_id, user_context).await {
        Ok(category) => Ok(Response::new(StatusCode::OK).data(json!(category))),
        Err(err) => Err(err.into()),
    }
}

pub async fn update_handler<C>(
    user_context: UserContext,
    State(category_service): State<CategoryService<C>>,
    Path(category_id): Path<CategoryID>,
    Json(update_request): Json<UpdateRequest>,
) -> Result<Response>
where
    C: CategoryRepository,
{
    let category = category_service
        .update(category_id, user_context, update_request)
        .await?;

    Ok(Response::new(StatusCode::OK).data(json!(category)))
}

pub async fn delete_handler<C>(
    user_context: UserContext,
    State(category_service): State<CategoryService<C>>,
    Path(category_id): Path<CategoryID>,
) -> Result<Response>
where
    C: CategoryRepository,
{
    category_service.delete(category_id, user_context).await?;

    Ok(Response::new(StatusCode::NO_CONTENT))
}
