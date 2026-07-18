use sqlx::{PgPool, test};

use crate::{
    tag::{
        repo::{CreateModel, PgTagRepository, TagRepository},
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

    let res = repo.get(test_tag.id, test_user.id).await;
    assert!(res.is_ok());
    if let Ok(tag_opt) = res {
        assert!(tag_opt.is_some());
    }
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

    let res = repo.get(other_tag.id, test_user.id).await;
    assert!(res.is_ok());
    if let Ok(tag_opt) = res {
        assert!(tag_opt.is_none());
    }
}

#[test]
async fn not_exists(pool: PgPool) {
    let user_repo = PgUserRepository::init(pool.clone());
    let repo = PgTagRepository::init(pool.clone());

    let test_user = create_test_user(&user_repo).await;

    let res = repo.get(TagID::new_v4(), test_user.id).await;
    assert!(res.is_ok());
    if let Ok(tag_opt) = res {
        assert!(tag_opt.is_none());
    }
}

// Output Tests
#[test]
async fn verify_output(pool: PgPool) {
    let user_repo = PgUserRepository::init(pool.clone());
    let repo = PgTagRepository::init(pool.clone());

    let test_user = create_test_user(&user_repo).await;

    let create_tag = CreateModel {
        label: "Test Tag".to_string(),
        category: Some("Test Category".to_string()),
        position_key: generate_a_z(0).to_string(),
    };
    let test_tag = repo.create(test_user.id, create_tag.clone()).await.unwrap();

    let tag = repo.get(test_tag.id, test_user.id).await.unwrap().unwrap();
    assert_eq!(tag.label, create_tag.label);
    assert_eq!(tag.category, create_tag.category);
    assert_eq!(tag.position_key, create_tag.position_key);
}
