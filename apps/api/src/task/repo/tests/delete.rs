use sqlx::{PgPool, test};

use crate::{
    error::repo::{ConstraintViolation, Error, Resource},
    task::{
        repo::{CreateModel, PgTaskRepository, TaskRepository},
        types::TaskID,
    },
    tests::helpers::{create_test_user, soft_delete_task},
    user::repo::PgUserRepository,
};

#[test]
async fn delete_existing(pool: PgPool) {
    let user_repo = PgUserRepository::init(pool.clone());
    let repo = PgTaskRepository::init(pool.clone());

    let user = create_test_user(&user_repo).await;
    let test_task = repo.create(user.id, CreateModel::default()).await.unwrap();

    let res = repo.delete(test_task.id, user.id).await;
    assert!(matches!(res, Ok(())));
}

#[test]
async fn delete_soft_deleted(pool: PgPool) {
    let user_repo = PgUserRepository::init(pool.clone());
    let repo = PgTaskRepository::init(pool.clone());

    let user = create_test_user(&user_repo).await;
    let test_task = repo.create(user.id, CreateModel::default()).await.unwrap();
    soft_delete_task(&repo, test_task.id, user.id).await;

    let res = repo.delete(test_task.id, user.id).await;
    if let Err(err) = res {
        assert!(matches!(
            err,
            Error::Constraint(ConstraintViolation::NotFound(Resource::Task))
        ))
    }
}

#[test]
async fn delete_deleted(pool: PgPool) {
    let user_repo = PgUserRepository::init(pool.clone());
    let repo = PgTaskRepository::init(pool.clone());

    let user = create_test_user(&user_repo).await;
    let test_task = repo.create(user.id, CreateModel::default()).await.unwrap();
    repo.delete(test_task.id, user.id).await.unwrap();

    let res = repo.delete(test_task.id, user.id).await;
    if let Err(err) = res {
        assert!(matches!(
            err,
            Error::Constraint(ConstraintViolation::NotFound(Resource::Task))
        ))
    }
}

#[test]
async fn delete_not_owned(pool: PgPool) {
    let user_repo = PgUserRepository::init(pool.clone());
    let repo = PgTaskRepository::init(pool.clone());

    let user = create_test_user(&user_repo).await;
    let other_user = create_test_user(&user_repo).await;
    let other_task = repo
        .create(other_user.id, CreateModel::default())
        .await
        .unwrap();

    let res = repo.delete(other_task.id, user.id).await;
    assert!(res.is_err());
    if let Err(err) = res {
        assert!(matches!(
            err,
            Error::Constraint(ConstraintViolation::NotFound(Resource::Task))
        ))
    }
}

#[test]
async fn delete_nonexistent(pool: PgPool) {
    let user_repo = PgUserRepository::init(pool.clone());
    let repo = PgTaskRepository::init(pool.clone());

    let user = create_test_user(&user_repo).await;

    let res = repo.delete(TaskID::new_v4(), user.id).await;
    assert!(res.is_err());
    if let Err(err) = res {
        assert!(matches!(
            err,
            Error::Constraint(ConstraintViolation::NotFound(Resource::Task))
        ))
    }
}
