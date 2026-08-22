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
    tests::helpers::create_test_user,
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

        category_repo
            .create(
                user_id,
                CreateModel {
                    name: "Testing".to_string(),
                    ..Default::default()
                },
            )
            .await
            .unwrap();

        let res = category_repo
            .get_id_by_name(user_id, "Testing".to_string())
            .await;
        assert!(res.is_ok());
    }

    #[test]
    async fn verify_output(pool: PgPool) {
        let (user_id, category_repo) = init(pool).await;

        let category = category_repo
            .create(
                user_id,
                CreateModel {
                    name: "Testing".to_string(),
                    ..Default::default()
                },
            )
            .await
            .unwrap();

        let id = category_repo
            .get_id_by_name(user_id, "Testing".to_string())
            .await
            .unwrap();
        assert_eq!(id, category.id);
    }
}

mod existence {
    use super::*;
    use sqlx::test;

    #[test]
    async fn not_owned(pool: PgPool) {
        let (user_id, category_repo) = init(pool.clone()).await;
        category_repo
            .create(
                user_id,
                CreateModel {
                    name: "Testing".to_string(),
                    ..Default::default()
                },
            )
            .await
            .unwrap();
        let (user_id, category_repo) = init(pool.clone()).await;

        let res = category_repo
            .get_id_by_name(user_id, "Testing".to_string())
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
        let (user_id, category_repo) = init(pool).await;

        let res = category_repo
            .get_id_by_name(user_id, "Random Category".to_string())
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
