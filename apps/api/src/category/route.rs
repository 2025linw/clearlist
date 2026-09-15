mod handler;

#[cfg(test)]
pub use handler::create_handler;

use axum::{Router, routing::get};

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
            "/{category_id}",
            get(handler::get_handler)
                .patch(handler::update_handler)
                .delete(handler::delete_handler),
        )
}
