use axum::{extract::State, http::StatusCode};
use serde_json::json;

use crate::{
    error::route::Result,
    tag::types::TagID,
    task::{
        repo::TaskRepository,
        service::TaskService,
        types::{
            TaskID,
            route::{CreateRequest, URLQueryOpts, UpdateRequest},
        },
    },
    types::{
        extract::{Json, Path, Query, UserContext},
        route::Response,
    },
};

pub async fn list_handler<T>(
    user_context: UserContext,
    State(task_service): State<TaskService<T>>,
    Query(query_opts): Query<URLQueryOpts>,
) -> Result<Response>
where
    T: TaskRepository,
{
    let tasks = task_service.list(user_context, query_opts).await?;

    Ok(Response::new(StatusCode::OK).data(json!({
        "count": tasks.len(),
        "tasks": tasks,
    })))
}

pub async fn create_handler<T>(
    user_context: UserContext,
    State(task_service): State<TaskService<T>>,
    Json(create_request): Json<CreateRequest>,
) -> Result<Response>
where
    T: TaskRepository,
{
    let task = task_service.create(user_context, create_request).await?;

    Ok(Response::new(StatusCode::CREATED).data(json!(task)))
}

pub async fn get_handler<T>(
    user_context: UserContext,
    State(task_service): State<TaskService<T>>,
    Path(task_id): Path<TaskID>,
) -> Result<Response>
where
    T: TaskRepository,
{
    match task_service.get(task_id, user_context).await {
        Ok(tag) => Ok(Response::new(StatusCode::OK).data(json!(tag))),
        Err(err) => Err(err.into()),
    }
}

pub async fn update_handler<T>(
    user_context: UserContext,
    State(task_service): State<TaskService<T>>,
    Path(task_id): Path<TaskID>,
    Json(update_request): Json<UpdateRequest>,
) -> Result<Response>
where
    T: TaskRepository,
{
    let tag = task_service
        .update(task_id, user_context, update_request)
        .await?;

    Ok(Response::new(StatusCode::OK).data(json!(tag)))
}

pub async fn delete_handler<T>(
    user_context: UserContext,
    State(task_service): State<TaskService<T>>,
    Path(task_id): Path<TaskID>,
) -> Result<Response>
where
    T: TaskRepository,
{
    task_service.delete(task_id, user_context).await?;

    Ok(Response::new(StatusCode::NO_CONTENT))
}

pub async fn restore_handler<T>(
    user_context: UserContext,
    State(task_service): State<TaskService<T>>,
    Path(task_id): Path<TaskID>,
) -> Result<Response>
where
    T: TaskRepository,
{
    task_service.restore(task_id, user_context).await?;

    Ok(Response::new(StatusCode::NO_CONTENT))
}

pub async fn complete_handler<T>(
    user_context: UserContext,
    State(task_service): State<TaskService<T>>,
    Path(task_id): Path<TaskID>,
) -> Result<Response>
where
    T: TaskRepository,
{
    task_service.complete(task_id, user_context).await?;

    Ok(Response::new(StatusCode::NO_CONTENT))
}

pub async fn reopen_handler<T>(
    user_context: UserContext,
    State(task_service): State<TaskService<T>>,
    Path(task_id): Path<TaskID>,
) -> Result<Response>
where
    T: TaskRepository,
{
    task_service.reopen(task_id, user_context).await?;

    Ok(Response::new(StatusCode::NO_CONTENT))
}

pub async fn list_tags_handler<T>(
    user_context: UserContext,
    State(task_service): State<TaskService<T>>,
    Path(task_id): Path<TaskID>,
) -> Result<Response>
where
    T: TaskRepository,
{
    task_service.list_tags(task_id, user_context).await?;

    Ok(Response::new(StatusCode::NO_CONTENT))
}

pub async fn add_tag_handler<T>(
    user_context: UserContext,
    State(task_service): State<TaskService<T>>,
    Path((task_id, tag_id)): Path<(TaskID, TagID)>,
) -> Result<Response>
where
    T: TaskRepository,
{
    task_service.add_tag(task_id, user_context, tag_id).await?;

    Ok(Response::new(StatusCode::NO_CONTENT))
}

pub async fn remove_tag_handler<T>(
    user_context: UserContext,
    State(task_service): State<TaskService<T>>,
    Path((task_id, tag_id)): Path<(TaskID, TagID)>,
) -> Result<Response>
where
    T: TaskRepository,
{
    task_service
        .remove_tag(task_id, user_context, tag_id)
        .await?;

    Ok(Response::new(StatusCode::NO_CONTENT))
}

pub async fn set_tags_handler<T>(
    user_context: UserContext,
    State(task_service): State<TaskService<T>>,
    Path(task_id): Path<TaskID>,
    Json(tag_ids): Json<Vec<TagID>>,
) -> Result<Response>
where
    T: TaskRepository,
{
    task_service
        .set_tags(task_id, user_context, tag_ids)
        .await?;

    Ok(Response::new(StatusCode::NO_CONTENT))
}
