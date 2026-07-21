use chrono::{DateTime, NaiveDate};
use sqlx::{PgPool, test};

use crate::{
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
                start_precision: Some(StartPrecision::DateTime),
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
                start_precision: StartPrecision::DateTime,
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
        start_precision: Some(StartPrecision::DateTime),
        deadline: Some(Some(get_today_date_pg().date_naive())),
        completed: Some(true),
        deleted: Some(false),
        position_key: Some(generate_a_z(1).to_string()),
    };
    let task = repo
        .update(test_task.id, test_user.id, update_model.clone())
        .await
        .unwrap()
        .unwrap();
    {
        let UpdateModel {
            title,
            notes,
            start,
            start_precision,
            deadline,
            completed,
            deleted,
            position_key,
        } = update_model;

        assert_eq!(task.title, title.unwrap());
        assert_eq!(task.notes, notes.unwrap());
        assert_eq!(task.start_dt, start.unwrap());
        assert!(task.has_time && start_precision.unwrap().has_time());
        assert_eq!(task.deadline, deadline.unwrap());
        assert!(task.completed_at.is_some() && completed.unwrap());
        assert!(task.deleted_at.is_none() && !deleted.unwrap());
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
