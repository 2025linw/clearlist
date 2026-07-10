use sqlx::{PgPool, test};

use crate::{
    error::repo::{ConstraintViolation, Error, Resource},
    tag::{
        repo::{CreateModel, PgTagRepository, TagRepository, UpdateModel},
        types::TagID,
    },
    tests::helpers::{create_test_user, generate_a_z},
    user::repo::PgUserRepository,
};

#[test]
async fn update_existing(pool: PgPool) {
    let user_repo = PgUserRepository::init(pool.clone());
    let repo = PgTagRepository::init(pool.clone());

    let user = create_test_user(&user_repo).await;
    let test_tag = repo.create(user.id, CreateModel::default()).await.unwrap();

    let res = repo
        .update(
            test_tag.id,
            user.id,
            UpdateModel {
                label: Some(String::from("Updated Tag")),
                ..Default::default()
            },
        )
        .await;
    assert!(res.is_ok());
}

#[test]
async fn update_not_owned(pool: PgPool) {
    let user_repo = PgUserRepository::init(pool.clone());
    let repo = PgTagRepository::init(pool.clone());

    let user = create_test_user(&user_repo).await;
    let other_user = create_test_user(&user_repo).await;
    let other_tag = repo
        .create(other_user.id, CreateModel::default())
        .await
        .unwrap();

    let res = repo
        .update(other_tag.id, user.id, UpdateModel::default())
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
async fn update_nonexistent(pool: PgPool) {
    let user_repo = PgUserRepository::init(pool.clone());
    let repo = PgTagRepository::init(pool.clone());

    let user = create_test_user(&user_repo).await;

    let res = repo
        .update(TagID::new_v4(), user.id, UpdateModel::default())
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
async fn update_is_idempotent(pool: PgPool) {
    let user_repo = PgUserRepository::init(pool.clone());
    let repo = PgTagRepository::init(pool.clone());

    let user = create_test_user(&user_repo).await;
    let test_tag = repo.create(user.id, CreateModel::default()).await.unwrap();

    let update_tag = UpdateModel {
        label: Some(String::from("Updated Tag")),
        ..Default::default()
    };
    let update_1 = repo
        .update(test_tag.id, user.id, update_tag.clone())
        .await
        .unwrap();
    let update_2 = repo
        .update(test_tag.id, user.id, update_tag.clone())
        .await
        .unwrap();

    assert_eq!(update_1, update_2);
}

#[test]
async fn update_full(pool: PgPool) {
    let user_repo = PgUserRepository::init(pool.clone());
    let repo = PgTagRepository::init(pool.clone());

    let user = create_test_user(&user_repo).await;
    let test_tag = repo.create(user.id, CreateModel::default()).await.unwrap();

    let res = repo
        .update(
            test_tag.id,
            user.id,
            UpdateModel {
                label: Some(String::from("Updated Tag")),
                category: Some(Some("Category".to_string())),
                position_key: Some(format!("full{}", generate_a_z(0))),
            },
        )
        .await;
    assert!(res.is_ok());
    if let Ok(tag) = res {
        assert_eq!(tag.label, "Updated Tag");
        assert_eq!(tag.category.unwrap(), "Category");

        assert_eq!(tag.position_key, format!("full{}", generate_a_z(0)));
    }
}

#[test]
async fn update_full_clear(pool: PgPool) {
    let user_repo = PgUserRepository::init(pool.clone());
    let repo = PgTagRepository::init(pool.clone());

    let user = create_test_user(&user_repo).await;
    let test_tag = repo
        .create(
            user.id,
            CreateModel {
                label: String::from("Test Tag"),
                category: Some(String::from("Testing")),
                position_key: format!("full{}", generate_a_z(0)),
            },
        )
        .await
        .unwrap();

    let res = repo
        .update(
            test_tag.id,
            user.id,
            UpdateModel {
                label: Some(String::new()),
                category: Some(None),
                position_key: None,
            },
        )
        .await;
    assert!(res.is_ok());
    if let Ok(tag) = res {
        assert_eq!(tag.label, "");
        assert!(tag.category.is_none());
    }
}
