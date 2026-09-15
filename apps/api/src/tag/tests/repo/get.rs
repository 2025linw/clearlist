use sqlx::PgPool;

use crate::{
    category::{repo::PgCategoryRepository, types::CategoryID},
    tag::{
        repo::{PgTagRepository, TagRepository},
        types::{TagID, repo::CreateModel},
    },
    tests::helpers::{create_test_category, create_test_user, generate_a_z},
    user::{repo::PgUserRepository, types::UserID},
};

async fn init(pool: PgPool) -> (UserID, CategoryID, TagID, PgTagRepository) {
    let user_repo = PgUserRepository::init(pool.clone());
    let category_repo = PgCategoryRepository::init(pool.clone());
    let tag_repo = PgTagRepository::init(pool.clone());

    let user = create_test_user(&user_repo).await;
    let category = create_test_category(&category_repo, user.id).await;
    let tag = tag_repo
        .create(user.id, CreateModel::default())
        .await
        .unwrap();

    (user.id, category.id, tag.id, tag_repo)
}

mod success {
    use super::*;
    use sqlx::test;

    #[test]
    async fn success(pool: PgPool) {
        let (user_id, _, tag_id, tag_repo) = init(pool).await;

        let res = tag_repo.get(tag_id, user_id).await;
        assert!(res.is_ok());
    }

    #[test]
    async fn verify_output(pool: PgPool) {
        let (user_id, category_id, _, tag_repo) = init(pool).await;

        let create_model = CreateModel {
            label: "Test Tag".to_string(),
            category_id: Some(category_id),
            position_key: generate_a_z(0).to_string(),
        };
        let test_tag = tag_repo
            .create(user_id, create_model.clone())
            .await
            .unwrap();

        let tag = tag_repo.get(test_tag.id, user_id).await.unwrap().unwrap();

        let CreateModel {
            label,
            category_id,
            position_key,
        } = create_model;
        assert_eq!(tag.label, label);
        assert_eq!(tag.category_id, category_id);
        assert_eq!(tag.category_name.unwrap(), "Test Category");
        assert_eq!(tag.position_key, position_key);
    }
}

mod existence {
    use super::*;
    use sqlx::test;

    #[test]
    async fn not_owned(pool: PgPool) {
        let (_, _, tag_id, _) = init(pool.clone()).await; // other tag
        let (user_id, _, _, tag_repo) = init(pool).await;

        let tag_opt = tag_repo.get(tag_id, user_id).await.unwrap();
        assert!(tag_opt.is_none());
    }

    #[test]
    async fn not_exists(pool: PgPool) {
        let (user_id, _, _, tag_repo) = init(pool).await;

        let tag_opt = tag_repo.get(TagID::new_random(), user_id).await.unwrap();
        assert!(tag_opt.is_none());
    }
}
