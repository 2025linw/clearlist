use sqlx::{PgPool, test};

use crate::{
    error::repo::{ConstraintViolation, Error, Resource},
    tag::repo::{CreateModel, PgTagRepository, TagRepository},
    tests::helpers::{create_test_user, generate_a_z},
    user::{repo::PgUserRepository, types::UserID},
};

#[test]
async fn create_required(pool: PgPool) {
    let user_repo = PgUserRepository::init(pool.clone());
    let repo = PgTagRepository::init(pool.clone());

    let user = create_test_user(&user_repo).await;

    let res = repo.create(user.id, CreateModel::default()).await;
    assert!(res.is_ok());
    if let Ok(tag) = res {
        assert_eq!(tag.label, "");
        assert!(tag.category.is_none());
    }
}

#[test]
async fn create_full(pool: PgPool) {
    let user_repo = PgUserRepository::init(pool.clone());
    let repo = PgTagRepository::init(pool.clone());

    let user = create_test_user(&user_repo).await;

    let res = repo
        .create(
            user.id,
            CreateModel {
                label: String::from("Test Tag"),
                category: Some(String::from("Testing")),
                position_key: format!("full{}", generate_a_z(0)),
            },
        )
        .await;
    assert!(res.is_ok());
    if let Ok(tag) = res {
        assert_eq!(tag.label, "Test Tag");
        assert_eq!(tag.category.unwrap(), "Testing");
    }
}

#[test]
async fn create_nonexistent_user(pool: PgPool) {
    let repo = PgTagRepository::init(pool.clone());

    let res = repo.create(UserID::new_v4(), CreateModel::default()).await;
    assert!(res.is_err());
    if let Err(err) = res {
        assert!(
            matches!(
                err,
                Error::Constraint(ConstraintViolation::NotFound(Resource::User))
            ),
            "{}",
            err
        )
    }
}
