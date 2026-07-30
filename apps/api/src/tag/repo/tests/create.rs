use sqlx::{PgPool, test};

use crate::{
    error::{
        Resource,
        repo::{ConstraintViolation, Error},
    },
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

    let test_user = create_test_user(&user_repo).await;

    let res = repo.create(test_user.id, CreateModel::default()).await;
    assert!(res.is_ok());
}

#[test]
async fn full_input(pool: PgPool) {
    let user_repo = PgUserRepository::init(pool.clone());
    let repo = PgTagRepository::init(pool.clone());

    let test_user = create_test_user(&user_repo).await;
    let test_category_id = repo
        .add_category(
            test_user.id,
            "Testing".to_string(),
            generate_a_z(0).to_string(),
        )
        .await
        .unwrap();

    let res = repo
        .create(
            test_user.id,
            CreateModel {
                label: "Test Tag".to_string(),
                category_id: Some(test_category_id),
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

    let test_user = create_test_user(&user_repo).await;
    let test_category_id = repo
        .add_category(
            test_user.id,
            "Testing".to_string(),
            generate_a_z(0).to_string(),
        )
        .await
        .unwrap();

    let create_model = CreateModel {
        label: "Test Tag".to_string(),
        category_id: Some(test_category_id),
        position_key: generate_a_z(0).to_string(),
    };
    let tag = repo
        .create(test_user.id, create_model.clone())
        .await
        .unwrap();
    {
        let CreateModel {
            label,
            category_id,
            position_key,
        } = create_model;

        assert_eq!(tag.label, label);
        assert_eq!(tag.category_id, category_id);
        assert_eq!(tag.category_name.as_deref(), Some("Testing"));
        assert_eq!(tag.position_key, position_key);
    }
}

// Behavior Tests
#[test]
async fn errors_on_duplicate_uncategorized_tag(pool: PgPool) {
    let user_repo = PgUserRepository::init(pool.clone());
    let repo = PgTagRepository::init(pool.clone());

    let test_user = create_test_user(&user_repo).await;
    repo.create(test_user.id, CreateModel::default())
        .await
        .unwrap();

    let res = repo.create(test_user.id, CreateModel::default()).await;
    assert!(res.is_err());
    if let Err(err) = res {
        assert!(matches!(
            err,
            Error::Constraint(ConstraintViolation::Unique(Resource::Tag))
        ))
    }
}

#[test]
async fn errors_on_duplicate_categorized_tag(pool: PgPool) {
    let user_repo = PgUserRepository::init(pool.clone());
    let repo = PgTagRepository::init(pool.clone());

    let test_user = create_test_user(&user_repo).await;
    let test_category_id = repo
        .add_category(
            test_user.id,
            "Testing".to_string(),
            generate_a_z(0).to_string(),
        )
        .await
        .unwrap();
    repo.create(
        test_user.id,
        CreateModel {
            category_id: Some(test_category_id),
            ..Default::default()
        },
    )
    .await
    .unwrap();

    let res = repo
        .create(
            test_user.id,
            CreateModel {
                category_id: Some(test_category_id),
                ..Default::default()
            },
        )
        .await;
    assert!(res.is_err());
    if let Err(err) = res {
        assert!(matches!(
            err,
            Error::Constraint(ConstraintViolation::Unique(Resource::Tag))
        ))
    }
}
