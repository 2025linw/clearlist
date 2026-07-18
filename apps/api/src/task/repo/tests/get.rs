use std::collections::HashSet;

use sqlx::{PgPool, test};

use crate::{
    tag::{
        repo::{PgTagRepository, TagRepository},
        types::{TagID, repo::CreateModel as TagCreateModel},
    },
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
    let tag_repo = PgTagRepository::init(pool.clone());
    let repo = PgTaskRepository::init(pool.clone());

    let test_user = create_test_user(&user_repo).await;
    let test_tag = tag_repo
        .create(test_user.id, TagCreateModel::default())
        .await
        .unwrap();

    let create_task = CreateModel {
        title: "Test Task".to_string(),
        notes: Some("Note for 'Test Task'".to_string()),
        start: Some(get_today_date_pg()),
        start_precision: StartPrecision::DateTime,
        deadline: Some(get_today_date_pg().date_naive()),
        tags: vec![test_tag.id],
        position_key: generate_a_z(0).to_string(),
    };
    let test_task = repo
        .create(test_user.id, create_task.clone())
        .await
        .unwrap();

    let task = repo.get(test_task.id, test_user.id).await.unwrap().unwrap();
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
