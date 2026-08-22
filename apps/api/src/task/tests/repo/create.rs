use chrono::{DateTime, NaiveDate};
use sqlx::PgPool;

use crate::{
    error::repo::{ConstraintViolation, Error},
    task::{
        repo::{PgTaskRepository, TaskRepository},
        types::repo::CreateModel,
    },
    tests::helpers::{create_test_user, generate_a_z, get_today_date_pg},
    user::{repo::PgUserRepository, types::UserID},
};

async fn init(pool: PgPool) -> (UserID, PgTaskRepository) {
    let user_repo = PgUserRepository::init(pool.clone());
    let task_repo = PgTaskRepository::init(pool.clone());

    let user = create_test_user(&user_repo).await;

    (user.id, task_repo)
}

mod success {
    use super::*;
    use sqlx::test;

    #[test]
    async fn success(pool: PgPool) {
        let (user_id, task_repo) = init(pool).await;

        let res = task_repo.create(user_id, CreateModel::default()).await;
        assert!(res.is_ok());
    }

    #[test]
    async fn verify_output(pool: PgPool) {
        let (user_id, task_repo) = init(pool).await;

        let create_model = CreateModel {
            title: "Test Task".to_string(),
            notes: Some("Notes for 'Test Task'".to_string()),
            start: Some(get_today_date_pg()),
            has_time: true,
            deadline: Some(get_today_date_pg().date_naive()),
            position_key: generate_a_z(0).to_string(),
        };
        let task = task_repo
            .create(user_id, create_model.clone())
            .await
            .unwrap();

        let CreateModel {
            title,
            notes,
            start,
            has_time,
            deadline,
            position_key,
        } = create_model;
        assert_eq!(task.title, title);
        assert_eq!(task.notes, notes);
        assert_eq!(task.start, start);
        assert_eq!(task.has_time, has_time);
        assert_eq!(task.deadline, deadline);
        assert_eq!(task.position_key, position_key);
    }
}

mod input {
    use super::*;
    use sqlx::test;

    #[test]
    async fn required_input(pool: PgPool) {
        let (user_id, task_repo) = init(pool).await;

        let create_model = CreateModel {
            title: "Test Task".to_string(),
            ..Default::default()
        };
        let res = task_repo.create(user_id, create_model).await;
        assert!(res.is_ok());
    }

    #[test]
    async fn full_input(pool: PgPool) {
        let (user_id, task_repo) = init(pool).await;

        let create_model = CreateModel {
            title: "Test Task".to_string(),
            notes: Some("Note for 'Test Task'".to_string()),
            start: Some(
                DateTime::parse_from_rfc3339("2026-01-01T08:45:00-06:00")
                    .unwrap()
                    .to_utc(),
            ),
            has_time: true,
            deadline: Some(NaiveDate::from_ymd_opt(2026, 1, 7).unwrap()),
            position_key: generate_a_z(0).to_string(),
        };
        let res = task_repo.create(user_id, create_model).await;
        assert!(res.is_ok());
    }
}

mod constraint {
    use sqlx::test;

    use crate::error::Resource;

    use super::*;

    #[test]
    async fn user_not_exists(pool: PgPool) {
        let (_, task_repo) = init(pool).await;

        let res = task_repo
            .create(UserID::new_random(), CreateModel::default())
            .await;
        assert!(res.is_err());
        if let Err(err) = res {
            assert!(matches!(
                err,
                Error::Constraint(ConstraintViolation::ForeignKey {
                    resource: Resource::Task,
                    message: _
                })
            ))
        }
    }
}
