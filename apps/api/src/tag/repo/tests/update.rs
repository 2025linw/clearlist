use sqlx::{PgPool, test};

use crate::{
    error::{
        Resource,
        repo::{ConstraintViolation, Error},
    },
    tag::{
        repo::{CreateModel, PgTagRepository, TagRepository, UpdateModel},
        types::TagID,
    },
    tests::helpers::{create_test_user, generate_a_z},
    user::repo::PgUserRepository,
};

// Existence Tests
#[test]
async fn exists(pool: PgPool) {
    let user_repo = PgUserRepository::init(pool.clone());
    let repo = PgTagRepository::init(pool.clone());

    let test_user = create_test_user(&user_repo).await;
    let test_tag = repo
        .create(test_user.id, CreateModel::default())
        .await
        .unwrap();

    let res = repo
        .update(test_tag.id, test_user.id, UpdateModel::default())
        .await;
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

    let res = repo
        .update(other_tag.id, test_user.id, UpdateModel::default())
        .await;
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

    let res = repo
        .update(TagID::new_v4(), test_user.id, UpdateModel::default())
        .await;
    assert!(res.is_err());
    if let Err(err) = res {
        assert!(matches!(
            err,
            Error::Constraint(ConstraintViolation::NotFound(Resource::Tag))
        ))
    }
}

// Input Tests
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
    let test_tag = repo
        .create(test_user.id, CreateModel::default())
        .await
        .unwrap();

    let res = repo
        .update(
            test_tag.id,
            test_user.id,
            UpdateModel {
                label: Some("Updated Tag".to_string()),
                category_id: Some(Some(test_category_id)),
                position_key: Some(generate_a_z(1).to_string()),
            },
        )
        .await;
    assert!(res.is_ok());
}

#[test]
async fn null_input(pool: PgPool) {
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
        .update(
            test_tag.id,
            test_user.id,
            UpdateModel {
                category_id: Some(None),
                ..Default::default()
            },
        )
        .await;
    assert!(res.is_ok());
    if let Ok(tag) = res {
        assert!(tag.category_id.is_none());
    }
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
    let test_tag = repo
        .create(test_user.id, CreateModel::default())
        .await
        .unwrap();

    let update_model = UpdateModel {
        label: Some("Updated Tag".to_string()),
        category_id: Some(Some(test_category_id)),
        position_key: Some(generate_a_z(1).to_string()),
    };
    let tag = repo
        .update(test_tag.id, test_user.id, update_model.clone())
        .await
        .unwrap();
    assert_eq!(tag.label, update_model.label.unwrap());
    assert_eq!(tag.category_name.as_deref(), Some("Testing"));
    assert_eq!(tag.position_key, update_model.position_key.unwrap());
}

// Behavior Tests
#[test]
async fn updates_updated_at(pool: PgPool) {
    let user_repo = PgUserRepository::init(pool.clone());
    let repo = PgTagRepository::init(pool.clone());

    let test_user = create_test_user(&user_repo).await;
    let test_tag = repo
        .create(test_user.id, CreateModel::default())
        .await
        .unwrap();

    let tag = repo
        .update(test_tag.id, test_user.id, UpdateModel::default())
        .await
        .unwrap();
    assert!(tag.updated_at > test_tag.updated_at);
}

#[test]
async fn is_idempotent(pool: PgPool) {
    let user_repo = PgUserRepository::init(pool.clone());
    let repo = PgTagRepository::init(pool.clone());

    let test_user = create_test_user(&user_repo).await;
    let test_tag = repo
        .create(test_user.id, CreateModel::default())
        .await
        .unwrap();

    let update_model = UpdateModel {
        label: Some("Updated Tag".to_string()),
        ..Default::default()
    };
    let update_1 = repo
        .update(test_tag.id, test_user.id, update_model.clone())
        .await
        .unwrap();
    let update_2 = repo
        .update(test_tag.id, test_user.id, update_model.clone())
        .await
        .unwrap();

    assert_eq!(update_1, update_2);
}
