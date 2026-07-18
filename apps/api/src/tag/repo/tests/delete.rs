use sqlx::{PgPool, test};

use crate::{
    error::repo::{ConstraintViolation, Error, Resource},
    tag::{
        repo::{CreateModel, PgTagRepository, TagRepository},
        types::TagID,
    },
    tests::helpers::create_test_user,
    user::repo::PgUserRepository,
};

// Existence Tests
#[test]
async fn exists(pool: PgPool) {
    let user_repo = PgUserRepository::init(pool.clone());
    let repo = PgTagRepository::init(pool.clone());

    let user = create_test_user(&user_repo).await;
    let test_tag = repo.create(user.id, CreateModel::default()).await.unwrap();

    let res = repo.delete(test_tag.id, user.id).await;
    assert!(res.is_ok());
}

#[test]
async fn not_owned(pool: PgPool) {
    let user_repo = PgUserRepository::init(pool.clone());
    let repo = PgTagRepository::init(pool.clone());

    let user = create_test_user(&user_repo).await;
    let other_user = create_test_user(&user_repo).await;
    let other_tag = repo
        .create(other_user.id, CreateModel::default())
        .await
        .unwrap();

    let res = repo.delete(other_tag.id, user.id).await;
    assert!(res.is_err());
    if let Err(err) = res {
        assert!(matches!(
            err,
            Error::Constraint(ConstraintViolation::NotFound(Resource::Tag))
        ))
    }
}

#[test]
async fn not_exists(pool: PgPool) {
    let user_repo = PgUserRepository::init(pool.clone());
    let repo = PgTagRepository::init(pool.clone());

    let user = create_test_user(&user_repo).await;

    let res = repo.delete(TagID::new_v4(), user.id).await;
    assert!(res.is_err());
    if let Err(err) = res {
        assert!(matches!(
            err,
            Error::Constraint(ConstraintViolation::NotFound(Resource::Tag))
        ))
    }
}
