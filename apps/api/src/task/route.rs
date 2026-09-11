mod handler;

use axum::{
    Router,
    routing::{get, patch, post},
};

use crate::{
    GenericAppState, category::repo::CategoryRepository, tag::repo::TagRepository,
    task::repo::TaskRepository, user::repo::UserRepository,
};

pub fn create_router<U, T, Ta, C>() -> Router<GenericAppState<U, T, Ta, C>>
where
    U: UserRepository,
    T: TaskRepository,
    Ta: TagRepository,
    C: CategoryRepository,
{
    Router::new()
        .route(
            "/",
            get(handler::list_handler).post(handler::create_handler),
        )
        .route(
            "/{task_id}",
            get(handler::get_handler)
                .patch(handler::update_handler)
                .delete(handler::delete_handler),
        )
        .route("/{task_id}/restore", post(handler::restore_handler))
        .route("/{task_id}/complete", post(handler::complete_handler))
        .route("/{task_id}/reopen", post(handler::reopen_handler))
        .nest(
            "/{task_id}/tags",
            Router::new()
                .route(
                    "/",
                    get(handler::list_tags_handler).put(handler::set_tags_handler),
                )
                .route(
                    "/{tag_id}",
                    patch(handler::add_tag_handler).delete(handler::remove_tag_handler),
                ),
        )
}
