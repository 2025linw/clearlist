use sqlx::{PgPool, test};

use crate::{
    error::{
        Resource,
        repo::{ConstraintViolation, Error},
    },
    tag::{
        repo::{CreateModel, PgTagRepository, TagRepository},
        types::TagID,
    },
    tests::helpers::create_test_user,
    user::repo::PgUserRepository,
};

// Existence Tests
#[test]
async fn success(pool: PgPool) {
    let user_repo = PgUserRepository::init(pool.clone());
    let repo = PgTagRepository::init(pool.clone());

    let test_user = create_test_user(&user_repo).await;
    let test_tag = repo
        .create(test_user.id, CreateModel::default())
        .await
        .unwrap();

    let res = repo.delete(test_tag.id, test_user.id).await;
    assert!(res.is_ok());
}

#[test]
async fn not_owned(pool: PgPool) {
    let user_repo = PgUserRepository::init(pool.clone());
    let repo = PgTagRepository::init(pool.clone());

    let test_user = create_test_user(&user_repo).await;
    let other_user = create_test_user(&user_repo).await;
    let other_tag = repo
        .create(other_user.id, CreateModel::default())
        .await
        .unwrap();

    let res = repo.delete(other_tag.id, test_user.id).await;
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

    let test_user = create_test_user(&user_repo).await;

    let res = repo.delete(TagID::new_v4(), test_user.id).await;
    assert!(res.is_err());
    if let Err(err) = res {
        assert!(matches!(
            err,
            Error::Constraint(ConstraintViolation::NotFound(Resource::Tag))
        ))
    }
}

// Behavior Tests
#[test]
async fn works(pool: PgPool) {
    let user_repo = PgUserRepository::init(pool.clone());
    let repo = PgTagRepository::init(pool.clone());

    let test_user = create_test_user(&user_repo).await;
    let test_tag = repo
        .create(test_user.id, CreateModel::default())
        .await
        .unwrap();

    repo.delete(test_tag.id, test_user.id).await.unwrap();
    let tag_opt = repo.get(test_tag.id, test_user.id).await.unwrap();
    assert_eq!(tag_opt, None);
}
