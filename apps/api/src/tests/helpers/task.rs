use chrono::{DateTime, NaiveDate};

use crate::{
    tag::types::TagID,
    task::{
        repo::{PgTaskRepository, TaskRepository},
        types::{
            repo::{CreateModel, UpdateModel},
            route::CreateRequest,
        },
    },
    user::types::UserID,
};

use super::generate_a_z;

pub async fn seed_tasks(
    repo: &PgTaskRepository,
    n: usize,
    user_id: UserID,
    state: Option<(bool, bool)>,
    builder: impl Fn(usize) -> CreateModel,
) {
    for i in 0..n {
        let task = repo.create(user_id, builder(i)).await.unwrap();
        if let Some((completed, deleted)) = state {
            repo.update(
                task.id,
                user_id,
                UpdateModel {
                    completed: Some(completed),
                    deleted: Some(deleted),
                    ..Default::default()
                },
            )
            .await
            .unwrap();
        }
    }
}

pub async fn seed_tasks_with_tags(
    repo: &PgTaskRepository,
    n: usize,
    user_id: UserID,
    state: Option<(bool, bool)>,
    tag_ids: Vec<TagID>,
    builder: impl Fn(usize) -> CreateModel,
) {
    for i in 0..n {
        let task = repo.create(user_id, builder(i)).await.unwrap();

        if let Some((completed, deleted)) = state {
            repo.update(
                task.id,
                user_id,
                UpdateModel {
                    completed: Some(completed),
                    deleted: Some(deleted),
                    ..Default::default()
                },
            )
            .await
            .unwrap();
        }
        if !tag_ids.is_empty() {
            repo.set_tags(task.id, user_id, tag_ids.clone())
                .await
                .unwrap();
        }
    }
}

pub fn default_task(i: usize) -> CreateModel {
    CreateModel {
        position_key: format!("def{}", generate_a_z(i)),
        ..Default::default()
    }
}

pub fn task_with_start(i: usize) -> CreateModel {
    let dt =
        DateTime::parse_from_rfc3339(&format!("2026-01-{:02}T{:02}:00:00Z", (i % 31) + 1, i % 24))
            .unwrap()
            .to_utc();

    CreateModel {
        start: Some(dt),
        has_time: true,
        position_key: format!("start{}", generate_a_z(i)),
        ..Default::default()
    }
}

pub fn task_with_deadline(i: usize) -> CreateModel {
    let date = NaiveDate::from_ymd_opt(2026, 1, ((i % 31) + 1) as u32).unwrap();

    CreateModel {
        deadline: Some(date),
        position_key: format!("due{}", generate_a_z(i)),
        ..Default::default()
    }
}

impl Default for CreateRequest {
    fn default() -> Self {
        Self {
            title: String::new(),
            notes: None,
            start: None,
            deadline: None,
            tags: None,
            position_key: generate_a_z(0).to_string(),
        }
    }
}

impl Default for CreateModel {
    fn default() -> Self {
        Self {
            title: String::new(),
            notes: None,
            start: None,
            has_time: true,
            deadline: None,
            position_key: generate_a_z(0).to_string(),
        }
    }
}
