use sqlx::{PgPool, test};

use crate::{
    error::{
        Resource,
        repo::{ConstraintViolation, Error},
    },
    tag::{
        repo::{PgTagRepository, TagRepository},
        types::{TagCategoryID, repo::CreateModel},
    },
    tests::helpers::{create_test_user, generate_a_z},
    user::repo::PgUserRepository,
};

// Existence Tests
#[test]
async fn success(pool: PgPool) {
    let user_repo = PgUserRepository::init(pool.clone());
    let repo = PgTagRepository::init(pool.clone());

    let test_user = create_test_user(&user_repo).await;
    let test_category_id = repo
        .add_category(
            test_user.id,
            "Test Category".to_string(),
            generate_a_z(0).to_string(),
        )
        .await
        .unwrap();

    let res = repo
        .reposition_category(test_category_id, test_user.id, generate_a_z(1).to_string())
        .await;
    assert!(res.is_ok());
}

#[test]
async fn not_owned(pool: PgPool) {
    let user_repo = PgUserRepository::init(pool.clone());
    let repo = PgTagRepository::init(pool.clone());

    let test_user = create_test_user(&user_repo).await;
    let other_user = create_test_user(&user_repo).await;
    let test_category_id = repo
        .add_category(
            other_user.id,
            "Test Category".to_string(),
            generate_a_z(0).to_string(),
        )
        .await
        .unwrap();

    let res = repo
        .reposition_category(test_category_id, test_user.id, generate_a_z(1).to_string())
        .await;
    assert!(res.is_err());
    if let Err(err) = res {
        assert!(matches!(
            err,
            Error::Constraint(ConstraintViolation::NotFound(Resource::Category))
        ))
    }
}

#[test]
async fn not_exists(pool: PgPool) {
    let user_repo = PgUserRepository::init(pool.clone());
    let repo = PgTagRepository::init(pool.clone());

    let test_user = create_test_user(&user_repo).await;

    let res = repo
        .reposition_category(
            TagCategoryID::new_v4(),
            test_user.id,
            generate_a_z(1).to_string(),
        )
        .await;
    assert!(res.is_err());
    if let Err(err) = res {
        assert!(matches!(
            err,
            Error::Constraint(ConstraintViolation::NotFound(Resource::Category))
        ))
    }
}

// Behavior Tests
#[test]
async fn works(pool: PgPool) {
    let user_repo = PgUserRepository::init(pool.clone());
    let repo = PgTagRepository::init(pool.clone());

    let test_user = create_test_user(&user_repo).await;
    let test_category_id = repo
        .add_category(
            test_user.id,
            "Test Category".to_string(),
            generate_a_z(0).to_string(),
        )
        .await
        .unwrap();
    let test_tag = repo
        .create(
            test_user.id,
            CreateModel {
                category_id: Some(test_category_id),
                ..Default::default()
            },
        )
        .await
        .unwrap();

    let res = repo
        .reposition_category(test_category_id, test_user.id, generate_a_z(1).to_string())
        .await;
    assert!(res.is_ok());

    let tag = repo.get(test_tag.id, test_user.id).await.unwrap().unwrap();
    assert_eq!(tag.cat_position_key, Some(generate_a_z(1).to_string()));
}
