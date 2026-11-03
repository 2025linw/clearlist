use sqlx::PgPool;

use crate::{
    category::{
        repo::{CategoryRepository, PgCategoryRepository},
        types::{
            CategoryID,
            repo::{CreateModel, UpdateModel},
        },
    },
    error::{
        Resource,
        repo::{ConstraintViolation, Error},
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

        let res = category_repo
            .update(
                category_id,
                user_id,
                UpdateModel {
                    name: Some("Updated Category".to_string()),
                    ..Default::default()
                },
            )
            .await;
        assert!(res.is_ok());
    }

    #[test]
    async fn verify_output(pool: PgPool) {
        let (user_id, category_id, category_repo) = init(pool).await;

        let update_model = UpdateModel {
            name: Some("Updated Category".to_string()),
            position_key: Some(generate_a_z(1).to_string()),
        };
        let category = category_repo
            .update(category_id, user_id, update_model.clone())
            .await
            .unwrap();

        let UpdateModel { name, position_key } = update_model;
        assert_eq!(category.name, name.unwrap());
        assert_eq!(category.position_key, position_key.unwrap());
    }
}

mod existence {
    use super::*;
    use sqlx::test;

    #[test]
    async fn not_owned(pool: PgPool) {
        let (_, category_id, _) = init(pool.clone()).await; // other category
        let (user_id, _, category_repo) = init(pool).await;

        let res = category_repo
            .update(
                category_id,
                user_id,
                UpdateModel {
                    name: Some("Updated Category".to_string()),
                    ..Default::default()
                },
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

    #[test]
    async fn not_exists(pool: PgPool) {
        let (user_id, _, category_repo) = init(pool).await;

        let res = category_repo
            .update(
                CategoryID::new_random(),
                user_id,
                UpdateModel {
                    name: Some("Updated Category".to_string()),
                    ..Default::default()
                },
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
}
