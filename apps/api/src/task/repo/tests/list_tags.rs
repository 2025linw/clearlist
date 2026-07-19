use sqlx::{PgPool, test};

use crate::{
    error::{
        Resource,
        repo::{ConstraintViolation, Error},
    },
    tag::{
        repo::{PgTagRepository, TagRepository},
        types::repo::CreateModel as TagCreateModel,
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

    let res = repo.list_tags(test_task.id, test_user.id).await;
    assert!(res.is_ok());
    if let Ok(state) = res {
        assert!(state.exists());
    }
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

    let res = repo.list_tags(test_task.id, test_user.id).await;
    assert!(res.is_ok());
    if let Ok(state) = res {
        assert!(state.deleted());
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

    let res = repo.list_tags(other_task.id, test_user.id).await;
    assert!(res.is_err());
    if let Err(err) = res {
        assert!(matches!(
            err,
            Error::Constraint(ConstraintViolation::NotFound(Resource::Task))
        ));
    }
}

#[test]
async fn task_not_exist(pool: PgPool) {
    let user_repo = PgUserRepository::init(pool.clone());
    let repo = PgTaskRepository::init(pool.clone());

    let test_user = create_test_user(&user_repo).await;

    let res = repo.list_tags(TaskID::new_v4(), test_user.id).await;
    assert!(res.is_err());
    if let Err(err) = res {
        assert!(matches!(
            err,
            Error::Constraint(ConstraintViolation::NotFound(Resource::Task))
        ));
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

    let tags = repo
        .list_tags(test_task.id, test_user.id)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(tags.len(), 1);
    assert!(tags.contains(&test_tag));
}
