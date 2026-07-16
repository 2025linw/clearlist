use chrono::{DateTime, NaiveDate, Utc};

use super::generate_a_z;
use crate::{
    tag::types::TagID,
    task::repo::{CreateModel, PgTaskRepository, TaskRepository, UpdateModel},
    types::date::StartPrecision,
    user::types::UserID,
};

pub async fn seed_tasks(
    repo: &PgTaskRepository,
    n: usize,
    user_id: UserID,
    tag_ids: Vec<TagID>,
    state: Option<(bool, bool)>,
    builder: impl Fn(usize, Vec<TagID>) -> CreateModel,
) {
    for i in 0..n {
        let tag_ids = tag_ids.to_vec();
        let task = repo.create(user_id, builder(i, tag_ids)).await.unwrap();

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

pub fn default_task(i: usize, tags: Vec<TagID>) -> CreateModel {
    CreateModel {
        tags,
        position_key: format!("def{}", generate_a_z(i)),
        ..Default::default()
    }
}

pub fn task_with_start(i: usize, tags: Vec<TagID>) -> CreateModel {
    let dt =
        DateTime::parse_from_rfc3339(&format!("2026-01-{:02}T{:02}:00:00Z", (i % 31) + 1, i % 24))
            .unwrap()
            .to_utc();

    CreateModel {
        start: Some(dt),
        start_precision: StartPrecision::Date,
        tags,
        position_key: format!("s{}", generate_a_z(i)),
        ..Default::default()
    }
}

pub fn task_with_deadline(i: usize, tags: Vec<TagID>) -> CreateModel {
    let date = NaiveDate::from_ymd_opt(2026, 01, ((i % 31) + 1) as u32).unwrap();

    CreateModel {
        deadline: Some(date),
        tags,
        position_key: format!("due{}", generate_a_z(i)),
        ..Default::default()
    }
}

pub fn full_task(i: usize, tags: Vec<TagID>) -> CreateModel {
    let dt =
        DateTime::parse_from_rfc3339(&format!("2026-01-{:02}T{:02}:00:00Z", (i % 31) + 1, i % 24))
            .unwrap()
            .to_utc();

    CreateModel {
        title: format!("Task {i}"),
        notes: Some(format!("Note for 'Task {i}'")),
        start: Some(dt),
        start_precision: StartPrecision::DateTime,
        deadline: Some(dt.date_naive()),
        tags,
        position_key: format!("full{}", generate_a_z(i)),
    }
}

impl Default for UpdateModel {
    fn default() -> Self {
        Self {
            title: Some("Updated task".to_string()),
            notes: Some(Some("Updated notes".to_string())),
            start: Some(Some(Utc::now())),
            start_precision: Some(StartPrecision::DateTime),
            deadline: Some(Some(Utc::now().date_naive())),
            tags: None,
            completed: Some(false),
            deleted: Some(false),
            position_key: Some(generate_a_z(1).to_string()),
        }
    }
}
