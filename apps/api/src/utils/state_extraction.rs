use axum::extract::FromRef;

use crate::{
    GenericAppState,
    category::{repo::CategoryRepository, service::CategoryService},
    tag::{repo::TagRepository, service::TagService},
    task::{repo::TaskRepository, service::TaskService},
    user::{repo::UserRepository, service::UserService},
};

impl<U, T, Ta, C> FromRef<GenericAppState<U, T, Ta, C>> for UserService<U>
where
    U: UserRepository,
    T: TaskRepository,
    Ta: TagRepository,
    C: CategoryRepository,
{
    fn from_ref(app_state: &GenericAppState<U, T, Ta, C>) -> Self {
        app_state.user_service.clone()
    }
}

impl<U, T, Ta, C> FromRef<GenericAppState<U, T, Ta, C>> for TaskService<T>
where
    U: UserRepository,
    T: TaskRepository,
    Ta: TagRepository,
    C: CategoryRepository,
{
    fn from_ref(app_state: &GenericAppState<U, T, Ta, C>) -> Self {
        app_state.task_service.clone()
    }
}

impl<U, T, Ta, C> FromRef<GenericAppState<U, T, Ta, C>> for TagService<Ta, C>
where
    U: UserRepository,
    T: TaskRepository,
    Ta: TagRepository,
    C: CategoryRepository,
{
    fn from_ref(app_state: &GenericAppState<U, T, Ta, C>) -> Self {
        app_state.tag_service.clone()
    }
}

impl<U, T, Ta, C> FromRef<GenericAppState<U, T, Ta, C>> for CategoryService<C>
where
    U: UserRepository,
    T: TaskRepository,
    Ta: TagRepository,
    C: CategoryRepository,
{
    fn from_ref(app_state: &GenericAppState<U, T, Ta, C>) -> Self {
        app_state.category_service.clone()
    }
}
