use std::collections::HashSet;

use chrono::{DateTime, NaiveDate};
use sqlx::{PgPool, test};

use crate::{
    error::repo::{ConstraintViolation, Error, Resource},
    tag::{
        repo::{CreateModel as TagCreateModel, PgTagRepository, TagRepository},
        types::TagID,
    },
    task::repo::{CreateModel, PgTaskRepository, TaskRepository},
    tests::helpers::{create_test_user, generate_a_z, get_today_date_pg},
    types::date::StartPrecision,
    user::{repo::PgUserRepository, types::UserID},
};

// Existence Tests
#[test]
async fn user_not_exists(pool: PgPool) {
    let repo = PgTaskRepository::init(pool.clone());

    let res = repo.create(UserID::new_v4(), CreateModel::default()).await;
    assert!(res.is_err());
    if let Err(err) = res {
        assert!(matches!(
            err,
            Error::Constraint(ConstraintViolation::MissingUser)
        ))
    }
}

// Input Tests
#[test]
async fn required_input(pool: PgPool) {
    let user_repo = PgUserRepository::init(pool.clone());
    let repo = PgTaskRepository::init(pool.clone());

    let user = create_test_user(&user_repo).await;

    let res = repo.create(user.id, CreateModel::default()).await;
    assert!(res.is_ok());
}

#[test]
async fn full_input(pool: PgPool) {
    let user_repo = PgUserRepository::init(pool.clone());
    let tag_repo = PgTagRepository::init(pool.clone());
    let repo = PgTaskRepository::init(pool.clone());

    let user = create_test_user(&user_repo).await;
    let tag = tag_repo
        .create(user.id, TagCreateModel::default())
        .await
        .unwrap();

    let res = repo
        .create(
            user.id,
            CreateModel {
                title: "Test Task".to_string(),
                notes: Some("Note for 'Test Task'".to_string()),
                start: Some(
                    DateTime::parse_from_rfc3339("2026-01-01T08:45:00-06:00")
                        .unwrap()
                        .to_utc(),
                ),
                start_precision: StartPrecision::DateTime,
                deadline: Some(NaiveDate::from_ymd_opt(2026, 1, 7).unwrap()),
                tags: vec![tag.id],
                position_key: generate_a_z(0).to_string(),
            },
        )
        .await;
    assert!(res.is_ok());
}

#[test]
async fn with_other_user_tag_input(pool: PgPool) {
    let user_repo = PgUserRepository::init(pool.clone());
    let tag_repo = PgTagRepository::init(pool.clone());
    let repo = PgTaskRepository::init(pool.clone());

    let user = create_test_user(&user_repo).await;
    let other_user = create_test_user(&user_repo).await;
    let other_tag = tag_repo
        .create(other_user.id, TagCreateModel::default())
        .await
        .unwrap();

    let res = repo
        .create(
            user.id,
            CreateModel {
                tags: vec![other_tag.id],
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

    let user = create_test_user(&user_repo).await;

    let res = repo
        .create(
            user.id,
            CreateModel {
                title: "Test Task with Nonexistent Tag".to_string(),
                tags: vec![TagID::new_v4()],
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

    let user = create_test_user(&user_repo).await;
    let test_tag = tag_repo
        .create(user.id, TagCreateModel::default())
        .await
        .unwrap();

    let create_task = CreateModel {
        title: "Test Task".to_string(),
        notes: Some("Notes for 'Test Task'".to_string()),
        start: Some(get_today_date_pg()),
        start_precision: StartPrecision::DateTime,
        deadline: Some(get_today_date_pg().date_naive()),
        tags: vec![test_tag.id],
        position_key: generate_a_z(0).to_string(),
    };
    let task = repo.create(user.id, create_task.clone()).await.unwrap();
    assert_eq!(task.title, create_task.title);
    assert_eq!(task.notes, create_task.notes);
    assert_eq!(task.start_dt, create_task.start);
    assert!(task.has_time == create_task.start_precision.has_time());
    assert_eq!(task.deadline, create_task.deadline);
    assert_eq!(
        task.tags
            .iter()
            .map(|tag| tag.id)
            .collect::<HashSet<TagID>>(),
        create_task.tags.into_iter().collect::<HashSet<TagID>>(),
    );
    assert_eq!(task.position_key, create_task.position_key);
}
