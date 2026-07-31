use chrono::{DateTime, NaiveDate};
use sqlx::{PgPool, test};

use crate::{
    error::{
        Resource,
        repo::{ConstraintViolation, Error},
    },
    task::{
        repo::{CreateModel, PgTaskRepository, TaskRepository, UpdateModel},
        types::TaskID,
    },
    tests::helpers::{create_test_user, generate_a_z, get_today_date_pg, soft_delete_task},
    user::repo::PgUserRepository,
};

// Existence Tests
#[test]
async fn success(pool: PgPool) {
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
    assert!(res.is_err());
    if let Err(err) = res {
        assert!(matches!(
            err,
            Error::Constraint(ConstraintViolation::Deleted(Resource::Task))
        ));
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
    let user_repo = PgUserRepository::init(pool.clone());
    let repo = PgTaskRepository::init(pool.clone());

    let test_user = create_test_user(&user_repo).await;

    let res = repo
        .update(TaskID::new_v4(), test_user.id, UpdateModel::default())
        .await;
    if let Err(err) = res {
        assert!(matches!(
            err,
            Error::Constraint(ConstraintViolation::NotFound(Resource::Task))
        ));
    }
}

// Input Tests
#[test]
async fn full_input(pool: PgPool) {
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
                title: Some("Updated Task".to_string()),
                notes: Some(Some("Notes for 'Updated Task'".to_string())),
                start: Some(Some(get_today_date_pg())),
                has_time: Some(true),
                deadline: Some(Some(get_today_date_pg().date_naive())),
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
    let repo = PgTaskRepository::init(pool.clone());

    let test_user = create_test_user(&user_repo).await;
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
                has_time: true,
                deadline: Some(NaiveDate::from_ymd_opt(2026, 1, 7).unwrap()),
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
                has_time: Some(false),
                deadline: Some(None),
                ..Default::default()
            },
        )
        .await;
    assert!(res.is_ok());
    if let Ok(task) = res {
        assert!(task.notes.is_none());
        assert!(task.start_dt.is_none());
        assert!(!task.has_time);
        assert!(task.deadline.is_none());
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
    if let Ok(task) = res {
        assert!(task.deleted_at.is_some());
    }
}

// Output Tests
#[test]
async fn verify_output(pool: PgPool) {
    let user_repo = PgUserRepository::init(pool.clone());
    let repo = PgTaskRepository::init(pool.clone());

    let test_user = create_test_user(&user_repo).await;
    let test_task = repo
        .create(test_user.id, CreateModel::default())
        .await
        .unwrap();

    let update_model = UpdateModel {
        title: Some("Updated Task".to_string()),
        notes: Some(Some("Updated notes for 'Updated Task'".to_string())),
        start: Some(Some(get_today_date_pg())),
        has_time: Some(true),
        deadline: Some(Some(get_today_date_pg().date_naive())),
        completed: Some(true),
        deleted: Some(false),
        position_key: Some(generate_a_z(1).to_string()),
    };
    let task = repo
        .update(test_task.id, test_user.id, update_model.clone())
        .await
        .unwrap();
    {
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
        assert_eq!(task.deleted_at.is_none(), !deleted.unwrap());
        assert_eq!(task.position_key, position_key.unwrap());
    }
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

    let update_model = UpdateModel {
        title: Some("Updated Task".to_string()),
        ..Default::default()
    };
    let update_1 = repo
        .update(test_task.id, test_user.id, update_model.clone())
        .await
        .unwrap();
    let update_2 = repo
        .update(test_task.id, test_user.id, update_model.clone())
        .await
        .unwrap();

    assert_eq!(update_1, update_2);
}
