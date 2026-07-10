pub mod tag;
pub mod task;
pub mod user;

use chrono::Utc;
use uuid::Uuid;

use crate::{
    tag::types::Model as TagModel,
    task::{
        repo::{PgTaskRepository, TaskRepository, UpdateModel as TaskUpdateModel},
        types::{Model as TaskModel, TaskID},
    },
    types::order::SortOrder,
    user::{
        repo::{CreateModel as UserCreateModel, PgUserRepository, UserRepository},
        types::{Model as UserModel, UserID},
    },
};

pub const TEST_USER_ID: UserID = UserID(Uuid::nil());

// Assertions
pub fn is_task_ordered<F, K>(tasks: &[TaskModel], mut f: F, sort: SortOrder) -> bool
where
    F: FnMut(&TaskModel) -> K,
    K: Ord,
{
    tasks.is_sorted_by(|a, b| match sort {
        SortOrder::Ascending => f(a) <= f(b),
        SortOrder::Descending => f(a) >= f(b),
    })
}

pub fn is_tag_ordered<F, K>(tags: &[TagModel], mut f: F, sort: SortOrder) -> bool
where
    F: FnMut(&TagModel) -> K,
    K: PartialOrd,
{
    tags.is_sorted_by(|a, b| match sort {
        SortOrder::Ascending => f(a) <= f(b),
        SortOrder::Descending => f(a) >= f(b),
    })
}

// Helper Functions
pub fn generate_a_z(i: usize) -> char {
    let offset = (i % 26) as u8;

    (b'a' + offset) as char
}

pub fn nulls_last_key<T>(key: Option<T>, sort: SortOrder) -> (bool, Option<T>)
where
    T: Ord,
{
    match sort {
        SortOrder::Ascending => (key.is_none(), key),
        SortOrder::Descending => (key.is_some(), key),
    }
}

pub fn create_user_model() -> UserCreateModel {
    UserCreateModel {
        id: UserID::new_v4(),
        display_name: String::from("Test User"),
        created_at: Utc::now(),
    }
}

pub async fn create_test_user(repo: &PgUserRepository) -> UserModel {
    repo.create(create_user_model()).await.unwrap()
}

pub async fn soft_delete_task(repo: &PgTaskRepository, id: TaskID, user_id: UserID) {
    repo.update(
        id,
        user_id,
        TaskUpdateModel {
            deleted: Some(true),
            ..Default::default()
        },
    )
    .await
    .unwrap();
}
