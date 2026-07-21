use sqlx::{PgPool, test};

use crate::{
    task::{
        repo::{CreateModel, PgTaskRepository, TaskRepository},
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

    let res = repo.get(test_task.id, test_user.id).await;
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
    soft_delete_task(&repo, test_task.id, test_user.id).await;

    let res = repo.get(test_task.id, test_user.id).await;
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

    let res = repo.get(other_task.id, test_user.id).await;
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

    let res = repo.get(TaskID::new_v4(), test_user.id).await;
    assert!(res.is_ok());
    if let Ok(task_state) = res {
        assert!(task_state.missing());
    }
}

// Output Tests
#[test]
async fn verify_output(pool: PgPool) {
    let user_repo = PgUserRepository::init(pool.clone());
    let repo = PgTaskRepository::init(pool.clone());

    let test_user = create_test_user(&user_repo).await;

    let create_model = CreateModel {
        title: "Test Task".to_string(),
        notes: Some("Note for 'Test Task'".to_string()),
        start: Some(get_today_date_pg()),
        start_precision: StartPrecision::DateTime,
        deadline: Some(get_today_date_pg().date_naive()),
        position_key: generate_a_z(0).to_string(),
    };
    let test_task = repo
        .create(test_user.id, create_model.clone())
        .await
        .unwrap();

    let task = repo.get(test_task.id, test_user.id).await.unwrap().unwrap();
    {
        let CreateModel {
            title,
            notes,
            start,
            start_precision,
            deadline,
            position_key,
        } = create_model;

        assert_eq!(task.title, title);
        assert_eq!(task.notes, notes);
        assert_eq!(task.start_dt, start);
        assert!(task.has_time && start_precision.has_time());
        assert_eq!(task.deadline, deadline);
        assert_eq!(task.position_key, position_key);
    }
}
