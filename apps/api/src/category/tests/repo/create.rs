use sqlx::PgPool;

use crate::{
    category::{
        repo::{CategoryRepository, PgCategoryRepository},
        types::repo::CreateModel,
    },
    error::{
        Resource,
        repo::{ConstraintViolation, Error},
    },
    tests::helpers::{create_test_user, generate_a_z},
    user::{repo::PgUserRepository, types::UserID},
};

async fn init(pool: PgPool) -> (UserID, PgCategoryRepository) {
    let user_repo = PgUserRepository::init(pool.clone());
    let category_repo = PgCategoryRepository::init(pool.clone());

    let user = create_test_user(&user_repo).await;

    (user.id, category_repo)
}

mod success {
    use super::*;
    use sqlx::test;

    #[test]
    async fn success(pool: PgPool) {
        let (user_id, category_repo) = init(pool).await;

        let res = category_repo.create(user_id, CreateModel::default()).await;
        assert!(res.is_ok())
    }

    #[test]
    async fn verify_output(pool: PgPool) {
        let (user_id, category_repo) = init(pool).await;

        let create_model = CreateModel {
            name: "Test Category".to_string(),
            position_key: generate_a_z(0).to_string(),
        };
        let category = category_repo
            .create(user_id, create_model.clone())
            .await
            .unwrap();

        let CreateModel { name, position_key } = create_model;
        assert_eq!(category.name, name);
        assert_eq!(category.position_key, position_key);
    }
}

mod constraint {
    use super::*;
    use sqlx::test;

    #[test]
    async fn user_not_exists(pool: PgPool) {
        let (_, category_repo) = init(pool).await;

        let res = category_repo
            .create(UserID::new_random(), CreateModel::default())
            .await;
        assert!(res.is_err());
        if let Err(err) = res {
            assert!(matches!(
                err,
                Error::Constraint(ConstraintViolation::ForeignKey {
                    resource: Resource::Category,
                    message: _
                })
            ))
        }
    }

    #[test]
    async fn errors_on_duplicate_category(pool: PgPool) {
        let (user_id, category_repo) = init(pool).await;

        category_repo
            .create(
                user_id,
                CreateModel {
                    name: "Test Category".to_string(),
                    ..Default::default()
                },
            )
            .await
            .unwrap();

        let res = category_repo
            .create(
                user_id,
                CreateModel {
                    name: "Test Category".to_string(),
                    ..Default::default()
                },
            )
            .await;
        assert!(res.is_err());
        if let Err(err) = res {
            assert!(matches!(
                err,
                Error::Constraint(ConstraintViolation::Unique {
                    resource: Resource::Category,
                    message: _
                })
            ))
        }
    }
}
