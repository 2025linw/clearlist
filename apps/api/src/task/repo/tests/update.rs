use std::collections::HashSet;

use chrono::{DateTime, NaiveDate};
use sqlx::{PgPool, test};

use crate::{
    error::repo::{ConstraintViolation, Error, Resource},
    tag::{
        repo::{PgTagRepository, TagRepository},
        types::{TagID, repo::CreateModel as TagCreateModel},
    },
    task::{
        repo::{CreateModel, PgTaskRepository, TaskRepository, UpdateModel},
        types::TaskID,
    },
    tests::helpers::{create_test_user, generate_a_z, get_today_date_pg, soft_delete_task},
    types::date::StartPrecision,
    user::repo::PgUserRepository,
};

// Existence Tests
#[test]
async fn exists(pool: PgPool) {
    let user_repo = PgUserRepository::init(pool.clone());
    let repo = PgTaskRepository::init(pool.clone());

    let test_user = create_test_user(&user_repo).await;
    let test_task = repo
        .create(test_user.id, CreateModel::default())
        .await
        .unwrap();

    let res = repo
        .update(test_task.id, test_user.id, UpdateModel::default())
        .await;
    assert!(res.is_ok());
    if let Ok(task_state) = res {
        assert!(task_state.exists());
    }
}

#[test]
async fn soft_deleted(pool: PgPool) {
    let user_repo = PgUserRepository::init(pool.clone());
    let repo = PgTaskRepository::init(pool.clone());

    let test_user = create_test_user(&user_repo).await;
    let test_task = repo
        .create(test_user.id, CreateModel::default())
        .await
        .unwrap();
    let test_task = soft_delete_task(&repo, test_task.id, test_user.id).await;

    let res = repo
        .update(test_task.id, test_user.id, UpdateModel::default())
        .await;
    assert!(res.is_ok());
    if let Ok(task_state) = res {
        assert!(task_state.deleted());
    }
}

#[test]
async fn not_owned(pool: PgPool) {
    let user_repo = PgUserRepository::init(pool.clone());
    let repo = PgTaskRepository::init(pool.clone());

    let test_user = create_test_user(&user_repo).await;
    let other_user = create_test_user(&user_repo).await;
    let other_task = repo
        .create(other_user.id, CreateModel::default())
        .await
        .unwrap();

    let res = repo
        .update(other_task.id, test_user.id, UpdateModel::default())
        .await;
    assert!(res.is_ok());
    if let Ok(task_state) = res {
        assert!(task_state.missing());
    }
}

#[test]
async fn not_exists(pool: PgPool) {
    let user_repo = PgUserRepository::init(pool.clone());
    let repo = PgTaskRepository::init(pool.clone());

    let test_user = create_test_user(&user_repo).await;

    let res = repo
        .update(TaskID::new_v4(), test_user.id, UpdateModel::default())
        .await;
    assert!(res.is_ok());
    if let Ok(task_state) = res {
        assert!(task_state.missing());
    }
}

// Input Tests
#[test]
async fn full_input(pool: PgPool) {
    let user_repo = PgUserRepository::init(pool.clone());
    let tag_repo = PgTagRepository::init(pool.clone());
    let repo = PgTaskRepository::init(pool.clone());

    let test_user = create_test_user(&user_repo).await;
    let test_tag = tag_repo
        .create(test_user.id, TagCreateModel::default())
        .await
        .unwrap();

    let test_task = repo
        .create(test_user.id, CreateModel::default())
        .await
        .unwrap();
    let res = repo
        .update(
            test_task.id,
            test_user.id,
            UpdateModel {
                title: Some("Updated Task".to_string()),
                notes: Some(Some("Notes for 'Updated Task'".to_string())),
                start: Some(Some(get_today_date_pg())),
                start_precision: Some(StartPrecision::DateTime),
                deadline: Some(Some(get_today_date_pg().date_naive())),
                tags: Some(vec![test_tag.id]),
                completed: Some(true),
                deleted: Some(false),
                position_key: Some(format!("full{}", generate_a_z(1))),
            },
        )
        .await;
    assert!(res.is_ok());
}

#[test]
async fn null_input(pool: PgPool) {
    let user_repo = PgUserRepository::init(pool.clone());
    let tag_repo = PgTagRepository::init(pool.clone());
    let repo = PgTaskRepository::init(pool.clone());

    let test_user = create_test_user(&user_repo).await;
    let test_tag = tag_repo
        .create(test_user.id, TagCreateModel::default())
        .await
        .unwrap();
    let test_task = repo
        .create(
            test_user.id,
            CreateModel {
                notes: Some("This is the note for Test Task".to_string()),
                start: Some(
                    DateTime::parse_from_rfc3339("2026-01-01T08:45:00-06:00")
                        .unwrap()
                        .to_utc(),
                ),
                start_precision: StartPrecision::DateTime,
                deadline: Some(NaiveDate::from_ymd_opt(2026, 1, 7).unwrap()),
                tags: vec![test_tag.id],
                ..Default::default()
            },
        )
        .await
        .unwrap();

    let res = repo
        .update(
            test_task.id,
            test_user.id,
            UpdateModel {
                notes: Some(None),
                start: Some(None),
                start_precision: Some(StartPrecision::Date),
                deadline: Some(None),
                ..Default::default()
            },
        )
        .await;
    assert!(res.is_ok());
    if let Ok(task_state) = res {
        let task = task_state.unwrap();
        assert!(task.notes.is_none());
        assert!(task.start_dt.is_none());
        assert!(!task.has_time);
        assert!(task.deadline.is_none());
        assert_eq!(task.tags.len(), 0);
    }
}

#[test]
async fn delete_input(pool: PgPool) {
    let user_repo = PgUserRepository::init(pool.clone());
    let repo = PgTaskRepository::init(pool.clone());

    let test_user = create_test_user(&user_repo).await;
    let test_task = repo
        .create(test_user.id, CreateModel::default())
        .await
        .unwrap();

    let res = repo
        .update(
            test_task.id,
            test_user.id,
            UpdateModel {
                deleted: Some(true),
                ..Default::default()
            },
        )
        .await;
    assert!(res.is_ok());
    if let Ok(task_state) = res {
        assert!(task_state.deleted());
    }
}

#[test]
async fn with_other_user_tag_input(pool: PgPool) {
    let user_repo = PgUserRepository::init(pool.clone());
    let tag_repo = PgTagRepository::init(pool.clone());
    let repo = PgTaskRepository::init(pool.clone());

    let test_user = create_test_user(&user_repo).await;
    let other_user = create_test_user(&user_repo).await;
    let other_tag = tag_repo
        .create(other_user.id, TagCreateModel::default())
        .await
        .unwrap();
    let test_task = repo
        .create(test_user.id, CreateModel::default())
        .await
        .unwrap();

    let res = repo
        .update(
            test_task.id,
            test_user.id,
            UpdateModel {
                tags: Some(vec![other_tag.id]),
                ..Default::default()
            },
        )
        .await;
    assert!(res.is_err());
    if let Err(err) = res {
        assert!(matches!(
            err,
            Error::Constraint(ConstraintViolation::NotFound(Resource::Tag))
        ))
    }
}

#[test]
async fn with_tag_not_exist_input(pool: PgPool) {
    let user_repo = PgUserRepository::init(pool.clone());
    let repo = PgTaskRepository::init(pool.clone());

    let test_user = create_test_user(&user_repo).await;
    let test_task = repo
        .create(test_user.id, CreateModel::default())
        .await
        .unwrap();

    let res = repo
        .update(
            test_task.id,
            test_user.id,
            UpdateModel {
                tags: Some(vec![TagID::new_v4()]),
                ..Default::default()
            },
        )
        .await;
    assert!(res.is_err());
    if let Err(err) = res {
        assert!(matches!(
            err,
            Error::Constraint(ConstraintViolation::NotFound(Resource::Tag))
        ))
    }
}

// Output Tests
#[test]
async fn verify_output(pool: PgPool) {
    let user_repo = PgUserRepository::init(pool.clone());
    let tag_repo = PgTagRepository::init(pool.clone());
    let repo = PgTaskRepository::init(pool.clone());

    let test_user = create_test_user(&user_repo).await;
    let test_tag = tag_repo
        .create(test_user.id, TagCreateModel::default())
        .await
        .unwrap();
    let test_task = repo
        .create(test_user.id, CreateModel::default())
        .await
        .unwrap();

    let update_task = UpdateModel {
        title: Some("Updated Task".to_string()),
        notes: Some(Some("Updated notes for 'Updated Task'".to_string())),
        start: Some(Some(get_today_date_pg())),
        start_precision: Some(StartPrecision::DateTime),
        deadline: Some(Some(get_today_date_pg().date_naive())),
        tags: Some(vec![test_tag.id]),
        completed: Some(true),
        deleted: Some(false),
        position_key: Some(generate_a_z(1).to_string()),
    };
    let task = repo
        .update(test_task.id, test_user.id, update_task.clone())
        .await
        .unwrap()
        .unwrap();
    assert_eq!(task.title, update_task.title.unwrap());
    assert_eq!(task.notes, update_task.notes.unwrap());
    assert_eq!(task.start_dt, update_task.start.unwrap());
    assert!(task.has_time == update_task.start_precision.unwrap().has_time());
    assert_eq!(task.deadline, update_task.deadline.unwrap());
    assert_eq!(
        task.tags
            .iter()
            .map(|tag| tag.id)
            .collect::<HashSet<TagID>>(),
        update_task
            .tags
            .unwrap()
            .into_iter()
            .collect::<HashSet<TagID>>(),
    );
    assert!(task.completed_at.is_some());
    assert_eq!(task.position_key, update_task.position_key.unwrap());
}

// Behavior Tests
#[test]
async fn updates_updated_at(pool: PgPool) {
    let user_repo = PgUserRepository::init(pool.clone());
    let repo = PgTaskRepository::init(pool.clone());

    let test_user = create_test_user(&user_repo).await;
    let test_task = repo
        .create(test_user.id, CreateModel::default())
        .await
        .unwrap();

    let task = repo
        .update(test_task.id, test_user.id, UpdateModel::default())
        .await
        .unwrap()
        .unwrap();
    assert!(task.updated_at > test_task.updated_at);
}

#[test]
async fn is_idempotent(pool: PgPool) {
    let user_repo = PgUserRepository::init(pool.clone());
    let repo = PgTaskRepository::init(pool.clone());

    let test_user = create_test_user(&user_repo).await;
    let test_task = repo
        .create(test_user.id, CreateModel::default())
        .await
        .unwrap();

    let update_task = UpdateModel {
        title: Some("Updated Task".to_string()),
        ..Default::default()
    };
    let update_1 = repo
        .update(test_task.id, test_user.id, update_task.clone())
        .await
        .unwrap();
    let update_2 = repo
        .update(test_task.id, test_user.id, update_task.clone())
        .await
        .unwrap();

    assert_eq!(update_1, update_2);
}
