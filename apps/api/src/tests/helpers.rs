pub mod tag;
pub mod task;
pub mod user;

use chrono::{DateTime, SubsecRound, Utc};

use crate::{
    task::{
        repo::{PgTaskRepository, TaskRepository},
        types::{TaskID, TaskModel, repo::UpdateModel as TaskUpdateModel},
    },
    types::order::SortOrder,
    user::{
        repo::{PgUserRepository, UserRepository},
        types::{Model as UserModel, UserID, repo::CreateModel as UserCreateModel},
    },
};

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

// Helper Functions
#[inline]
pub fn get_today_date_pg() -> DateTime<Utc> {
    Utc::now().trunc_subsecs(6)
}

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

pub async fn create_test_user(repo: &PgUserRepository) -> UserModel {
    repo.create(UserCreateModel::default()).await.unwrap()
}

pub async fn soft_delete_task(repo: &PgTaskRepository, id: TaskID, user_id: UserID) -> TaskModel {
    let state = repo
        .update(
            id,
            user_id,
            TaskUpdateModel {
                deleted: Some(true),
                ..Default::default()
            },
        )
        .await
        .unwrap();

    if let Some(task) = state.into_inner() {
        task
    } else {
        panic!()
    }
}
