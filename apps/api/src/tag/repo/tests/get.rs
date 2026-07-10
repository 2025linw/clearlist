use sqlx::{PgPool, test};

use crate::{
    tag::{
        repo::{CreateModel, PgTagRepository, TagRepository},
        types::TagID,
    },
    tests::helpers::create_test_user,
    user::repo::PgUserRepository,
};

#[test]
async fn get_existing(pool: PgPool) {
    let user_repo = PgUserRepository::init(pool.clone());
    let repo = PgTagRepository::init(pool.clone());

    let user = create_test_user(&user_repo).await;
    let test_tag = repo.create(user.id, CreateModel::default()).await.unwrap();

    let res = repo.get(test_tag.id, user.id).await;
    assert!(res.is_ok());
    if let Ok(tag_opt) = res {
        assert!(tag_opt.is_some());
        if let Some(tag) = tag_opt {
            assert_eq!(tag, test_tag);
        }
    }
}

#[test]
async fn get_not_owned(pool: PgPool) {
    let user_repo = PgUserRepository::init(pool.clone());
    let repo = PgTagRepository::init(pool.clone());

    let user = create_test_user(&user_repo).await;
    let other_user = create_test_user(&user_repo).await;
    let other_tag = repo
        .create(other_user.id, CreateModel::default())
        .await
        .unwrap();

    let res = repo.get(other_tag.id, user.id).await;
    assert!(res.is_ok());
    if let Ok(tag_opt) = res {
        assert!(tag_opt.is_none());
    }
}

#[test]
async fn get_nonexistent(pool: PgPool) {
    let user_repo = PgUserRepository::init(pool.clone());
    let repo = PgTagRepository::init(pool.clone());

    let user = create_test_user(&user_repo).await;

    let res = repo.get(TagID::new_v4(), user.id).await;
    assert!(res.is_ok());
    if let Ok(tag_opt) = res {
        assert!(tag_opt.is_none());
    }
}
