use sqlx::{PgPool, test};

use crate::{
    error::{
        Resource,
        repo::{ConstraintViolation, Error},
    },
    tag::repo::{PgTagRepository, TagRepository},
    tests::helpers::{create_test_user, generate_a_z},
    user::{repo::PgUserRepository, types::UserID},
};

// Existence Tests
#[test]
async fn user_not_exists(pool: PgPool) {
    let repo = PgTagRepository::init(pool.clone());

    let res = repo
        .add_category(
            UserID::new_v4(),
            "Test Category".to_string(),
            generate_a_z(0).to_string(),
        )
        .await;
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
async fn input(pool: PgPool) {
    let user_repo = PgUserRepository::init(pool.clone());
    let repo = PgTagRepository::init(pool.clone());

    let test_user = create_test_user(&user_repo).await;

    let res = repo
        .add_category(
            test_user.id,
            "Test Category".to_string(),
            generate_a_z(0).to_string(),
        )
        .await;
    assert!(res.is_ok())
}

// Behavior Tests
#[test]
async fn errors_on_duplicate_category(pool: PgPool) {
    let user_repo = PgUserRepository::init(pool.clone());
    let repo = PgTagRepository::init(pool.clone());

    let test_user = create_test_user(&user_repo).await;
    repo.add_category(
        test_user.id,
        "Test Category".to_string(),
        generate_a_z(0).to_string(),
    )
    .await
    .unwrap();

    let res = repo
        .add_category(
            test_user.id,
            "Test Category".to_string(),
            generate_a_z(0).to_string(),
        )
        .await;
    assert!(res.is_err());
    if let Err(err) = res {
        assert!(matches!(
            err,
            Error::Constraint(ConstraintViolation::Unique(Resource::Category))
        ))
    }
}
