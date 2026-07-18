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
    tests::helpers::{create_test_user, soft_delete_task},
    user::repo::PgUserRepository,
};

// Existence Tests
#[test]
async fn task_exists(pool: PgPool) {
    let user_repo = PgUserRepository::init(pool.clone());
    let tag_repo = PgTagRepository::init(pool.clone());
    let repo = PgTaskRepository::init(pool.clone());

    let test_user = create_test_user(&user_repo).await;
    let test_tag = tag_repo
        .create(test_user.id, TagCreateModel::default())
        .await
        .unwrap();
    let test_task = repo
        .create(
            test_user.id,
            CreateModel {
                tags: vec![test_tag.id],
                ..Default::default()
            },
        )
        .await
        .unwrap();

    let res = repo
        .remove_tag(test_task.id, test_user.id, test_tag.id)
        .await;
    assert!(res.is_ok());
    if let Ok(task_state) = res {
        assert!(task_state.exists());
    }

    let task = repo
        .get(test_task.id, test_user.id)
        .await
        .unwrap()
        .expect("task was just created for the test");
    assert_eq!(task.tags.len(), 0);
}

#[test]
async fn task_soft_deleted(pool: PgPool) {
    let user_repo = PgUserRepository::init(pool.clone());
    let tag_repo = PgTagRepository::init(pool.clone());
    let repo = PgTaskRepository::init(pool.clone());

    let test_user = create_test_user(&user_repo).await;
    let test_tag = tag_repo
        .create(test_user.id, TagCreateModel::default())
        .await
        .unwrap();
    let test_task = repo
        .create(
            test_user.id,
            CreateModel {
                tags: vec![test_tag.id],
                ..Default::default()
            },
        )
        .await
        .unwrap();
    soft_delete_task(&repo, test_task.id, test_user.id).await;

    let res = repo
        .remove_tag(test_task.id, test_user.id, test_tag.id)
        .await;
    assert!(res.is_ok());
    if let Ok(task_state) = res {
        assert!(task_state.deleted());
    }
}

#[test]
async fn task_not_owned(pool: PgPool) {
    let user_repo = PgUserRepository::init(pool.clone());
    let tag_repo = PgTagRepository::init(pool.clone());
    let repo = PgTaskRepository::init(pool.clone());

    let test_user = create_test_user(&user_repo).await;
    let other_user = create_test_user(&user_repo).await;
    let other_tag = tag_repo
        .create(other_user.id, TagCreateModel::default())
        .await
        .unwrap();
    let other_task = repo
        .create(
            other_user.id,
            CreateModel {
                tags: vec![other_tag.id],
                ..Default::default()
            },
        )
        .await
        .unwrap();

    let res = repo
        .remove_tag(other_task.id, test_user.id, other_tag.id)
        .await;
    assert!(res.is_ok());
    if let Ok(task_state) = res {
        assert!(task_state.missing());
    }
}

#[test]
async fn task_not_exist(pool: PgPool) {
    let user_repo = PgUserRepository::init(pool.clone());
    let tag_repo = PgTagRepository::init(pool.clone());
    let repo = PgTaskRepository::init(pool.clone());

    let test_user = create_test_user(&user_repo).await;
    let test_tag = tag_repo
        .create(test_user.id, TagCreateModel::default())
        .await
        .unwrap();

    let res = repo
        .remove_tag(TaskID::new_v4(), test_user.id, test_tag.id)
        .await;
    assert!(res.is_ok());
    if let Ok(task_state) = res {
        assert!(task_state.missing());
    }
}

#[test]
async fn tag_not_exist(pool: PgPool) {
    let user_repo = PgUserRepository::init(pool.clone());
    let repo = PgTaskRepository::init(pool.clone());

    let test_user = create_test_user(&user_repo).await;
    let test_task = repo
        .create(test_user.id, CreateModel::default())
        .await
        .unwrap();

    let res = repo
        .remove_tag(test_task.id, test_user.id, TagID::new_v4())
        .await;
    assert!(res.is_ok());
}

// Behavior Tests
#[test]
async fn updates_updated_at(pool: PgPool) {
    let user_repo = PgUserRepository::init(pool.clone());
    let tag_repo = PgTagRepository::init(pool.clone());
    let repo = PgTaskRepository::init(pool.clone());

    let test_user = create_test_user(&user_repo).await;
    let test_tag = tag_repo
        .create(test_user.id, TagCreateModel::default())
        .await
        .unwrap();
    let test_task = repo
        .create(
            test_user.id,
            CreateModel {
                tags: vec![test_tag.id],
                ..Default::default()
            },
        )
        .await
        .unwrap();

    repo.remove_tag(test_task.id, test_user.id, test_tag.id)
        .await
        .unwrap()
        .unwrap();
    let task = repo
        .get(test_task.id, test_user.id)
        .await
        .unwrap()
        .expect("task was just created for the test");
    assert!(task.updated_at > test_task.updated_at);
}

#[test]
async fn is_idempotent(pool: PgPool) {
    let user_repo = PgUserRepository::init(pool.clone());
    let tag_repo = PgTagRepository::init(pool.clone());
    let repo = PgTaskRepository::init(pool.clone());

    let test_user = create_test_user(&user_repo).await;
    let test_tag = tag_repo
        .create(test_user.id, TagCreateModel::default())
        .await
        .unwrap();
    let test_task = repo
        .create(test_user.id, CreateModel::default())
        .await
        .unwrap();

    repo.remove_tag(test_task.id, test_user.id, test_tag.id)
        .await
        .unwrap()
        .unwrap();
    let task = repo
        .get(test_task.id, test_user.id)
        .await
        .unwrap()
        .expect("task was just created for the test");
    assert_eq!(task.updated_at, test_task.updated_at);
}
