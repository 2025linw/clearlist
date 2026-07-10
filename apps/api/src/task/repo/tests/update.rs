use std::time::Duration;

use chrono::{DateTime, NaiveDate, SubsecRound, Utc};
use sqlx::{PgPool, test};

use crate::{
    error::repo::{ConstraintViolation, Error, Resource},
    tag::{
        repo::{CreateModel as TagCreateModel, PgTagRepository, TagRepository},
        types::TagID,
    },
    task::{
        repo::{CreateModel, PgTaskRepository, TaskRepository, UpdateModel},
        types::TaskID,
    },
    tests::helpers::{create_test_user, generate_a_z, soft_delete_task},
    types::date::StartPrecision,
    user::repo::PgUserRepository,
};

#[test]
async fn update_existing(pool: PgPool) {
    let user_repo = PgUserRepository::init(pool.clone());
    let repo = PgTaskRepository::init(pool.clone());

    let user = create_test_user(&user_repo).await;
    let test_task = repo.create(user.id, CreateModel::default()).await.unwrap();

    let res = repo
        .update(
            test_task.id,
            user.id,
            UpdateModel {
                title: Some(String::from("Updated task")),
                ..Default::default()
            },
        )
        .await;
    assert!(res.is_ok());
}

#[test]
async fn update_soft_deleted(pool: PgPool) {
    let user_repo = PgUserRepository::init(pool.clone());
    let repo = PgTaskRepository::init(pool.clone());

    let user = create_test_user(&user_repo).await;
    let test_task = repo.create(user.id, CreateModel::default()).await.unwrap();
    soft_delete_task(&repo, test_task.id, user.id).await;

    let res = repo
        .update(test_task.id, user.id, UpdateModel::default())
        .await;
    assert!(res.is_err());
    if let Err(err) = res {
        assert!(matches!(
            err,
            Error::Constraint(ConstraintViolation::NotFound(Resource::Task))
        ))
    }
}

#[test]
async fn update_deleted(pool: PgPool) {
    let user_repo = PgUserRepository::init(pool.clone());
    let repo = PgTaskRepository::init(pool.clone());

    let user = create_test_user(&user_repo).await;
    let test_task = repo.create(user.id, CreateModel::default()).await.unwrap();
    repo.delete(test_task.id, user.id).await.unwrap();

    let res = repo
        .update(test_task.id, user.id, UpdateModel::default())
        .await;
    assert!(res.is_err());
    if let Err(err) = res {
        assert!(matches!(
            err,
            Error::Constraint(ConstraintViolation::NotFound(Resource::Task))
        ))
    }
}

#[test]
async fn update_not_owned(pool: PgPool) {
    let user_repo = PgUserRepository::init(pool.clone());
    let repo = PgTaskRepository::init(pool.clone());

    let user = create_test_user(&user_repo).await;
    let other_user = create_test_user(&user_repo).await;
    let other_task = repo
        .create(other_user.id, CreateModel::default())
        .await
        .unwrap();

    let res = repo
        .update(other_task.id, user.id, UpdateModel::default())
        .await;
    assert!(res.is_err());
    if let Err(err) = res {
        assert!(matches!(
            err,
            Error::Constraint(ConstraintViolation::NotFound(Resource::Task))
        ))
    }
}

#[test]
async fn update_nonexistent(pool: PgPool) {
    let user_repo = PgUserRepository::init(pool.clone());
    let repo = PgTaskRepository::init(pool.clone());

    let user = create_test_user(&user_repo).await;

    let res = repo
        .update(TaskID::new_v4(), user.id, UpdateModel::default())
        .await;
    assert!(res.is_err());
    if let Err(err) = res {
        assert!(matches!(
            err,
            Error::Constraint(ConstraintViolation::NotFound(Resource::Task))
        ))
    }
}

#[test]
async fn update_is_idempotent(pool: PgPool) {
    let user_repo = PgUserRepository::init(pool.clone());
    let repo = PgTaskRepository::init(pool.clone());

    let user = create_test_user(&user_repo).await;
    let test_task = repo.create(user.id, CreateModel::default()).await.unwrap();

    let update_task = UpdateModel {
        title: Some(String::from("Updated task")),
        ..Default::default()
    };
    let update_1 = repo
        .update(test_task.id, user.id, update_task.clone())
        .await
        .unwrap();
    let update_2 = repo
        .update(test_task.id, user.id, update_task.clone())
        .await
        .unwrap();

    assert_eq!(update_1, update_2);
}

#[test]
async fn update_full(pool: PgPool) {
    let user_repo = PgUserRepository::init(pool.clone());
    let repo = PgTaskRepository::init(pool.clone());

    let user = create_test_user(&user_repo).await;
    let test_task = repo.create(user.id, CreateModel::default()).await.unwrap();

    let dt = Utc::now().trunc_subsecs(6);
    let res = repo
        .update(
            test_task.id,
            user.id,
            UpdateModel {
                title: Some(String::from("Updated task")),
                notes: Some(Some("Notes for 'Updated task'".to_string())),
                start: Some(Some(dt + Duration::from_hours(24 * 7))),
                start_precision: Some(StartPrecision::DateTime),
                deadline: Some(Some(dt.date_naive())),
                tags: None,
                completed: Some(true),
                deleted: Some(true),
                position_key: Some(format!("full{}", generate_a_z(0))),
            },
        )
        .await;
    assert!(res.is_ok());
    if let Ok(task) = res {
        assert_eq!(task.title, "Updated task");
        assert_eq!(task.notes.unwrap(), "Notes for 'Updated task'");
        assert_eq!(task.start_dt.unwrap(), dt + Duration::from_hours(24 * 7));
        assert!(task.has_time);
        assert_eq!(task.deadline.unwrap(), dt.date_naive());

        assert!(task.completed_at.is_some());
        assert!(task.deleted_at.is_some());

        assert_eq!(task.position_key, format!("full{}", generate_a_z(0)));
    }
}

#[test]
async fn update_full_clear(pool: PgPool) {
    let user_repo = PgUserRepository::init(pool.clone());
    let repo = PgTaskRepository::init(pool.clone());

    let user = create_test_user(&user_repo).await;
    let test_task = repo
        .create(
            user.id,
            CreateModel {
                title: String::from("Test Task"),
                notes: Some(String::from("This is the note for Test Task")),
                start: Some(
                    DateTime::parse_from_rfc3339("2026-01-01T08:45:00-06:00")
                        .unwrap()
                        .to_utc(),
                ),
                start_precision: StartPrecision::DateTime,
                deadline: Some(NaiveDate::from_ymd_opt(2026, 1, 7).unwrap()),
                tags: Vec::new(),
                position_key: format!("full{}", generate_a_z(0)),
            },
        )
        .await
        .unwrap();

    let res = repo
        .update(
            test_task.id,
            user.id,
            UpdateModel {
                title: Some(String::new()),
                notes: Some(None),
                start: Some(None),
                start_precision: Some(StartPrecision::Date),
                deadline: Some(None),
                tags: None,
                completed: Some(false),
                deleted: Some(false),
                position_key: None,
            },
        )
        .await;
    assert!(res.is_ok());
    if let Ok(task) = res {
        assert_eq!(task.title, "");
        assert!(task.notes.is_none());
        assert!(task.start_dt.is_none());
        assert!(!task.has_time);
        assert!(task.deadline.is_none());

        assert!(task.completed_at.is_none());
        assert!(task.deleted_at.is_none());
    }
}

#[test]
async fn update_with_tags(pool: PgPool) {
    let user_repo = PgUserRepository::init(pool.clone());
    let repo = PgTaskRepository::init(pool.clone());
    let tag_repo = PgTagRepository::init(pool.clone());

    let user = create_test_user(&user_repo).await;
    let test_task = repo.create(user.id, CreateModel::default()).await.unwrap();
    let mut test_tags = Vec::with_capacity(5);
    for _ in 0..5 {
        let tag = tag_repo
            .create(user.id, TagCreateModel::default())
            .await
            .unwrap();

        test_tags.push(tag);
    }

    let res = repo
        .update(
            test_task.id,
            user.id,
            UpdateModel {
                tags: Some(test_tags.iter().map(|tag| tag.id).collect()),
                ..Default::default()
            },
        )
        .await;
    assert!(res.is_ok());
    if let Ok(task) = res {
        assert_eq!(task.tags.len(), 5);
        for tag in task.tags {
            assert!(test_tags.contains(&tag));
        }
    }
}

#[test]
async fn update_with_tags_full_clear(pool: PgPool) {
    let user_repo = PgUserRepository::init(pool.clone());
    let repo = PgTaskRepository::init(pool.clone());
    let tag_repo = PgTagRepository::init(pool.clone());

    let user = create_test_user(&user_repo).await;
    let mut test_tags = Vec::with_capacity(5);
    for _ in 0..5 {
        let tag = tag_repo
            .create(user.id, TagCreateModel::default())
            .await
            .unwrap();

        test_tags.push(tag);
    }
    let test_task = repo
        .create(
            user.id,
            CreateModel {
                tags: test_tags.iter().map(|tag| tag.id).collect(),
                ..Default::default()
            },
        )
        .await
        .unwrap();
    let res = repo
        .update(test_task.id, user.id, UpdateModel::default())
        .await;
    assert!(res.is_ok());
    if let Ok(task) = res {
        assert_eq!(task.tags.len(), 0);
    }
}

#[test]
async fn update_tags_different_owner(pool: PgPool) {
    let user_repo = PgUserRepository::init(pool.clone());
    let repo = PgTaskRepository::init(pool.clone());
    let tag_repo = PgTagRepository::init(pool.clone());

    let user = create_test_user(&user_repo).await;
    let other_user = create_test_user(&user_repo).await;
    let other_tag = tag_repo
        .create(other_user.id, TagCreateModel::default())
        .await
        .unwrap();
    let test_task = repo.create(user.id, CreateModel::default()).await.unwrap();

    let res = repo
        .update(
            test_task.id,
            user.id,
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
async fn update_tags_nonexistent(pool: PgPool) {
    let user_repo = PgUserRepository::init(pool.clone());
    let repo = PgTaskRepository::init(pool.clone());

    let user = create_test_user(&user_repo).await;
    let test_task = repo.create(user.id, CreateModel::default()).await.unwrap();

    let res = repo
        .update(
            test_task.id,
            user.id,
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
