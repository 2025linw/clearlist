use chrono::{DateTime, NaiveDate};
use sqlx::{PgPool, test};

use crate::{
    error::repo::{ConstraintViolation, Error},
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

    let test_user = create_test_user(&user_repo).await;

    let res = repo.create(test_user.id, CreateModel::default()).await;
    assert!(res.is_ok());
}

#[test]
async fn full_input(pool: PgPool) {
    let user_repo = PgUserRepository::init(pool.clone());
    let repo = PgTaskRepository::init(pool.clone());

    let test_user = create_test_user(&user_repo).await;

    let res = repo
        .create(
            test_user.id,
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
                position_key: generate_a_z(0).to_string(),
            },
        )
        .await;
    assert!(res.is_ok());
}

// Output Tests
#[test]
async fn verify_output(pool: PgPool) {
    let user_repo = PgUserRepository::init(pool.clone());
    let repo = PgTaskRepository::init(pool.clone());

    let test_user = create_test_user(&user_repo).await;

    let create_task = CreateModel {
        title: "Test Task".to_string(),
        notes: Some("Notes for 'Test Task'".to_string()),
        start: Some(get_today_date_pg()),
        start_precision: StartPrecision::DateTime,
        deadline: Some(get_today_date_pg().date_naive()),
        position_key: generate_a_z(0).to_string(),
    };
    let task = repo
        .create(test_user.id, create_task.clone())
        .await
        .unwrap();
    {
        let CreateModel {
            title,
            notes,
            start,
            start_precision,
            deadline,
            position_key,
        } = create_task;

        assert_eq!(task.title, title);
        assert_eq!(task.notes, notes);
        assert_eq!(task.start_dt, start);
        assert!(task.has_time && start_precision.has_time());
        assert_eq!(task.deadline, deadline);
        assert_eq!(task.position_key, position_key);
    }
}
