use chrono::{DateTime, NaiveDate};
use sqlx::{PgPool, test};

use crate::{
    error::repo::{ConstraintViolation, Error, Resource},
    tag::{
        repo::{CreateModel as TagCreateModel, PgTagRepository, TagRepository},
        types::TagID,
    },
    task::repo::{CreateModel, PgTaskRepository, TaskRepository},
    tests::helpers::{create_test_user, generate_a_z},
    types::date::StartPrecision,
    user::{repo::PgUserRepository, types::UserID},
};

#[test]
async fn create_required(pool: PgPool) {
    let user_repo = PgUserRepository::init(pool.clone());
    let repo = PgTaskRepository::init(pool.clone());

    let user = create_test_user(&user_repo).await;

    let res = repo.create(user.id, CreateModel::default()).await;
    assert!(res.is_ok());
    if let Ok(task) = res {
        assert_eq!(task.title, "");
        assert!(task.notes.is_none());
        assert!(task.start_dt.is_none());
        assert!(!task.has_time);
        assert!(task.deadline.is_none());
        assert_eq!(task.tags.len(), 0);
    }
}

#[test]
async fn create_full(pool: PgPool) {
    let user_repo = PgUserRepository::init(pool.clone());
    let repo = PgTaskRepository::init(pool.clone());

    let user = create_test_user(&user_repo).await;

    let res = repo
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
        .await;
    assert!(res.is_ok());
    if let Ok(task) = res {
        assert_eq!(task.title, "Test Task");
        assert_eq!(task.notes.unwrap(), "This is the note for Test Task");
        assert_eq!(
            task.start_dt
                .unwrap()
                .to_rfc3339_opts(chrono::SecondsFormat::AutoSi, true),
            "2026-01-01T14:45:00Z"
        );
        assert!(task.has_time);
        assert_eq!(
            task.deadline.map(|date| date.to_string()).unwrap(),
            "2026-01-07"
        );
        assert_eq!(task.tags.len(), 0);
    }
}

#[test]
async fn create_with_tags(pool: PgPool) {
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

    let res = repo
        .create(
            user.id,
            CreateModel {
                tags: test_tags.iter().map(|tag| tag.id).collect(),
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
async fn create_tags_different_owner(pool: PgPool) {
    let user_repo = PgUserRepository::init(pool.clone());
    let repo = PgTaskRepository::init(pool.clone());
    let tag_repo = PgTagRepository::init(pool.clone());

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
async fn create_tags_nonexistent(pool: PgPool) {
    let user_repo = PgUserRepository::init(pool.clone());
    let repo = PgTaskRepository::init(pool.clone());

    let user = create_test_user(&user_repo).await;

    let res = repo
        .create(
            user.id,
            CreateModel {
                title: String::from("Test Task with Nonexistent Tag"),
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

#[test]
async fn create_nonexistent_user(pool: PgPool) {
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
