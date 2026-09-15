use chrono::{DateTime, NaiveDate};
use sqlx::PgPool;

use crate::{
    error::{
        Resource,
        repo::{ConstraintViolation, Error},
    },
    task::{
        repo::{PgTaskRepository, TaskRepository},
        types::{
            TaskID,
            repo::{CreateModel, UpdateModel},
        },
    },
    tests::helpers::{create_test_user, generate_a_z, get_today_date_pg, soft_delete_task},
    user::{repo::PgUserRepository, types::UserID},
};

async fn init(pool: PgPool) -> (UserID, TaskID, PgTaskRepository) {
    let user_repo = PgUserRepository::init(pool.clone());
    let task_repo = PgTaskRepository::init(pool.clone());

    let user = create_test_user(&user_repo).await;
    let task = task_repo
        .create(user.id, CreateModel::default())
        .await
        .unwrap();

    (user.id, task.id, task_repo)
}

mod success {
    use super::*;
    use sqlx::test;

    #[test]
    async fn success(pool: PgPool) {
        let (user_id, task_id, task_repo) = init(pool).await;

        let res = task_repo
            .update(
                task_id,
                user_id,
                UpdateModel {
                    title: Some("Updated Task".to_string()),
                    ..Default::default()
                },
            )
            .await;
        assert!(res.is_ok());
    }

    #[test]
    async fn verify_output(pool: PgPool) {
        let (user_id, task_id, task_repo) = init(pool).await;

        let update_model = UpdateModel {
            title: Some("Updated Task".to_string()),
            notes: Some(Some("Updated notes for 'Updated Task'".to_string())),
            start: Some(Some(get_today_date_pg())),
            has_time: Some(true),
            deadline: Some(Some(get_today_date_pg().date_naive())),
            completed: Some(true),
            deleted: Some(true),
            position_key: Some(generate_a_z(1).to_string()),
        };
        let task = task_repo
            .update(task_id, user_id, update_model.clone())
            .await
            .unwrap();

        let UpdateModel {
            title,
            notes,
            start,
            has_time,
            deadline,
            completed,
            deleted,
            position_key,
        } = update_model;
        assert_eq!(task.title, title.unwrap());
        assert_eq!(task.notes, notes.unwrap());
        assert_eq!(task.start_dt, start.unwrap());
        assert_eq!(task.has_time, has_time.unwrap());
        assert_eq!(task.deadline, deadline.unwrap());
        assert_eq!(task.completed_at.is_some(), completed.unwrap());
        assert_eq!(task.deleted_at.is_some(), deleted.unwrap());
        assert_eq!(task.position_key, position_key.unwrap());
    }

    #[test]
    async fn updates_updated_at(pool: PgPool) {
        let (user_id, task_id, task_repo) = init(pool).await;
        let task_init = task_repo.get(task_id, user_id).await.unwrap().unwrap();

        let task = task_repo
            .update(
                task_id,
                user_id,
                UpdateModel {
                    title: Some("Update Task".to_string()),
                    ..Default::default()
                },
            )
            .await
            .unwrap();
        assert!(task.updated_at > task_init.updated_at);
    }

    #[test]
    async fn is_idempotent(pool: PgPool) {
        let (user_id, task_id, task_repo) = init(pool).await;

        let update_model = UpdateModel {
            title: Some("Updated Task".to_string()),
            ..Default::default()
        };
        let first_update = task_repo
            .update(task_id, user_id, update_model.clone())
            .await
            .unwrap();
        let second_update = task_repo
            .update(task_id, user_id, update_model.clone())
            .await
            .unwrap();

        assert_eq!(first_update, second_update);
    }
}

mod existence {
    use super::*;
    use sqlx::test;

    #[test]
    async fn soft_deleted(pool: PgPool) {
        let (user_id, task_id, task_repo) = init(pool).await;
        soft_delete_task(&task_repo, task_id, user_id).await;

        let res = task_repo
            .update(
                task_id,
                user_id,
                UpdateModel {
                    title: Some("Update Task".to_string()),
                    ..Default::default()
                },
            )
            .await;
        assert!(res.is_ok());
    }

    #[test]
    async fn not_owned(pool: PgPool) {
        let (_, task_id, _) = init(pool.clone()).await; // other task
        let (user_id, _, task_repo) = init(pool).await;

        let res = task_repo
            .update(
                task_id,
                user_id,
                UpdateModel {
                    title: Some("Update Task".to_string()),
                    ..Default::default()
                },
            )
            .await;
        assert!(res.is_err());
        if let Err(err) = res {
            assert!(matches!(
                err,
                Error::Constraint(ConstraintViolation::NotFound(Resource::Task))
            ));
        }
    }

    #[test]
    async fn not_exists(pool: PgPool) {
        let (user_id, _, task_repo) = init(pool).await;

        let res = task_repo
            .update(
                TaskID::new_random(),
                user_id,
                UpdateModel {
                    title: Some("Updated Task".to_string()),
                    ..Default::default()
                },
            )
            .await;
        assert!(res.is_err());
        if let Err(err) = res {
            assert!(matches!(
                err,
                Error::Constraint(ConstraintViolation::NotFound(Resource::Task))
            ));
        }
    }
}

mod input {
    use super::*;
    use sqlx::test;

    #[test]
    async fn full_input(pool: PgPool) {
        let (user_id, task_id, task_repo) = init(pool).await;

        let update_model = UpdateModel {
            title: Some("Updated Task".to_string()),
            notes: Some(Some("Notes for 'Updated Task'".to_string())),
            start: Some(Some(get_today_date_pg())),
            has_time: Some(true),
            deadline: Some(Some(get_today_date_pg().date_naive())),
            completed: Some(true),
            deleted: Some(true),
            position_key: Some(format!("full{}", generate_a_z(1))),
        };
        let res = task_repo.update(task_id, user_id, update_model).await;
        assert!(res.is_ok());
    }

    #[test]
    async fn null_input(pool: PgPool) {
        let (user_id, _, task_repo) = init(pool).await;
        let task = task_repo
            .create(
                user_id,
                CreateModel {
                    notes: Some("This is the note for Test Task".to_string()),
                    start: Some(
                        DateTime::parse_from_rfc3339("2026-01-01T08:45:00-06:00")
                            .unwrap()
                            .to_utc(),
                    ),
                    has_time: true,
                    deadline: Some(NaiveDate::from_ymd_opt(2026, 1, 7).unwrap()),
                    ..Default::default()
                },
            )
            .await
            .unwrap();

        let update_model = UpdateModel {
            notes: Some(None),
            start: Some(None),
            deadline: Some(None),
            ..Default::default()
        };
        let res = task_repo.update(task.id, user_id, update_model).await;
        if let Ok(task) = res {
            assert!(task.notes.is_none());
            assert!(task.start_dt.is_none());
            assert!(task.deadline.is_none());
        }
    }
}
