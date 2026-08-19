mod handler;

#[cfg(test)]
pub use handler::create_handler;

use axum::{Router, routing::get};

use crate::{
    GenericAppState, tag::repo::TagRepository, task::repo::TaskRepository,
    user::repo::UserRepository,
};

pub fn create_router<U, T, Ta>() -> Router<GenericAppState<U, T, Ta>>
where
    U: UserRepository,
    T: TaskRepository,
    Ta: TagRepository,
{
    Router::new()
        .route(
            "/",
            get(handler::list_handler).post(handler::create_handler),
        )
        .route(
            "/{tag_id}",
            get(handler::get_handler)
                .patch(handler::update_handler)
                .delete(handler::delete_handler),
        )
}
