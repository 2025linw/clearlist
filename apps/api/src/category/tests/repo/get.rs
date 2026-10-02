use sqlx::PgPool;

use crate::{
    category::{
        repo::{CategoryRepository, PgCategoryRepository},
        types::{CategoryID, repo::CreateModel},
    },
    tests::helpers::{create_test_user, generate_a_z},
    user::{repo::PgUserRepository, types::UserID},
};

async fn init(pool: PgPool) -> (UserID, CategoryID, PgCategoryRepository) {
    let user_repo = PgUserRepository::init(pool.clone());
    let category_repo = PgCategoryRepository::init(pool.clone());

    let user = create_test_user(&user_repo).await;
    let category = category_repo
        .create(user.id, CreateModel::default())
        .await
        .unwrap();

    (user.id, category.id, category_repo)
}

mod success {
    use super::*;
    use sqlx::test;

    #[test]
    async fn success(pool: PgPool) {
        let (user_id, category_id, category_repo) = init(pool).await;

        let res = category_repo.get(category_id, user_id).await;
        assert!(res.is_ok());
    }

    #[test]
    async fn verify_output(pool: PgPool) {
        let (user_id, _, category_repo) = init(pool).await;

        let create_model = CreateModel {
            name: "Test Category".to_string(),
            position_key: generate_a_z(0).to_string(),
        };
        let test_category = category_repo
            .create(user_id, create_model.clone())
            .await
            .unwrap();

        let tag = category_repo
            .get(test_category.id, user_id)
            .await
            .unwrap()
            .unwrap();

        let CreateModel { name, position_key } = create_model;
        assert_eq!(tag.name, name);
        assert_eq!(tag.position_key, position_key);
    }
}

mod existence {
    use super::*;
    use sqlx::test;

    #[test]
    async fn not_owned(pool: PgPool) {
        let (_, category_id, _) = init(pool.clone()).await; // other tag
        let (user_id, _, category_repo) = init(pool).await;

        let category_opt = category_repo.get(category_id, user_id).await.unwrap();
        assert!(category_opt.is_none());
    }

    #[test]
    async fn not_exists(pool: PgPool) {
        let (user_id, _, category_repo) = init(pool).await;

        let category_opt = category_repo
            .get(CategoryID::new_random(), user_id)
            .await
            .unwrap();
        assert!(category_opt.is_none());
    }
}
