use sqlx::PgPool;

use crate::{
    category::{
        repo::{CategoryRepository, PgCategoryRepository},
        types::repo::CreateModel,
    },
    tests::helpers::{
        category::{default_category, seed_categories},
        create_test_user, generate_a_z,
    },
    user::{repo::PgUserRepository, types::UserID},
};

async fn init(pool: PgPool) -> (UserID, PgCategoryRepository) {
    let user_repo = PgUserRepository::init(pool.clone());
    let tag_repo = PgCategoryRepository::init(pool.clone());

    let user = create_test_user(&user_repo).await;

    (user.id, tag_repo)
}

mod success {
    use super::*;
    use sqlx::test;

    #[test]
    async fn success(pool: PgPool) {
        let (user_id, category_repo) = init(pool).await;

        seed_categories(&category_repo, 25, user_id, default_category).await;

        let res = category_repo.list(user_id).await;
        assert!(res.is_ok());
    }

    #[test]
    async fn verify_output(pool: PgPool) {
        let (user_id, category_repo) = init(pool).await;

        category_repo
            .create(
                user_id,
                CreateModel {
                    name: "Test Category".to_string(),
                    position_key: generate_a_z(0).to_string(),
                },
            )
            .await
            .unwrap();

        let category = category_repo.list(user_id).await.unwrap().remove(0);
        assert!(category.name.starts_with("Test Category"));
        assert_eq!(category.position_key, generate_a_z(0).to_string());
    }
}

mod sort {
    use super::*;
    use sqlx::test;

    #[test]
    async fn sorts_by_position_then_id(pool: PgPool) {
        let (user_id, category_repo) = init(pool).await;

        for i in 0..5 {
            category_repo
                .create(
                    user_id,
                    CreateModel {
                        name: format!("Category Pos0 {}", i),
                        position_key: generate_a_z(0).to_string(),
                    },
                )
                .await
                .unwrap();
        }
        for i in 0..5 {
            category_repo
                .create(
                    user_id,
                    CreateModel {
                        name: format!("Category Pos1 {}", i),
                        position_key: generate_a_z(1).to_string(),
                    },
                )
                .await
                .unwrap();
        }
        for i in 0..5 {
            category_repo
                .create(
                    user_id,
                    CreateModel {
                        name: format!("Category Pos2 {}", i),
                        position_key: generate_a_z(2).to_string(),
                    },
                )
                .await
                .unwrap();
        }

        let categories = category_repo.list(user_id).await.unwrap();
        assert!(categories.is_sorted_by(|a, b| {
            (a.position_key.as_str(), a.id) <= (b.position_key.as_str(), b.id)
        }));
    }
}
