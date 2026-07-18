use sqlx::{PgPool, test};

use crate::{
    error::repo::{ConstraintViolation, Error},
    tag::repo::{CreateModel, PgTagRepository, TagRepository},
    tests::helpers::{create_test_user, generate_a_z},
    user::{repo::PgUserRepository, types::UserID},
};

// Existence Tests
#[test]
async fn user_not_exists(pool: PgPool) {
    let repo = PgTagRepository::init(pool.clone());

    let res = repo.create(UserID::new_v4(), CreateModel::default()).await;
    assert!(res.is_err());
    if let Err(err) = res {
        assert!(matches!(
            err,
            Error::Constraint(ConstraintViolation::MissingUser)
        ))
    }
}

// Input Tests
#[test]
async fn required_input(pool: PgPool) {
    let user_repo = PgUserRepository::init(pool.clone());
    let repo = PgTagRepository::init(pool.clone());

    let user = create_test_user(&user_repo).await;

    let res = repo.create(user.id, CreateModel::default()).await;
    assert!(res.is_ok());
}

#[test]
async fn full_input(pool: PgPool) {
    let user_repo = PgUserRepository::init(pool.clone());
    let repo = PgTagRepository::init(pool.clone());

    let user = create_test_user(&user_repo).await;

    let res = repo
        .create(
            user.id,
            CreateModel {
                label: "Test Tag".to_string(),
                category: Some("Testing".to_string()),
                position_key: generate_a_z(0).to_string(),
            },
        )
        .await;
    assert!(res.is_ok());
}

// Output Tests
#[test]
async fn verify_output(pool: PgPool) {
    let user_repo = PgUserRepository::init(pool.clone());
    let repo = PgTagRepository::init(pool.clone());

    let user = create_test_user(&user_repo).await;

    let create_tag = CreateModel {
        label: "Test Tag".to_string(),
        category: Some("Testing".to_string()),
        position_key: generate_a_z(0).to_string(),
    };
    let tag = repo.create(user.id, create_tag.clone()).await.unwrap();
    assert_eq!(tag.label, create_tag.label);
    assert_eq!(tag.category, create_tag.category);
    assert_eq!(tag.position_key, create_tag.position_key);
}
