use sqlx::{PgPool, test};

use crate::{
    error::{
        Resource,
        repo::{ConstraintViolation, Error},
    },
    task::{
        repo::{CreateModel, PgTaskRepository, TaskRepository},
        types::{TaskID, repo::TaskState},
    },
    tests::helpers::{create_test_user, soft_delete_task},
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

    let res = repo.delete(test_task.id, test_user.id).await;
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
    soft_delete_task(&repo, test_task.id, test_user.id).await;

    let res = repo.delete(test_task.id, test_user.id).await;
    assert!(res.is_ok());
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

    let res = repo.delete(other_task.id, test_user.id).await;
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

    let res = repo.delete(TaskID::new_v4(), test_user.id).await;
    assert!(res.is_err());
    if let Err(err) = res {
        assert!(matches!(
            err,
            Error::Constraint(ConstraintViolation::NotFound(Resource::Task))
        ));
    }
}

// Behavior Tests
#[test]
async fn works(pool: PgPool) {
    let user_repo = PgUserRepository::init(pool.clone());
    let repo = PgTaskRepository::init(pool.clone());

    let test_user = create_test_user(&user_repo).await;
    let test_task = repo
        .create(test_user.id, CreateModel::default())
        .await
        .unwrap();

    repo.delete(test_task.id, test_user.id).await.unwrap();
    let task_state = repo.get(test_task.id, test_user.id).await.unwrap();
    assert_eq!(task_state, TaskState::None);
}
