use sqlx::PgPool;

use crate::{
    category::{
        repo::{CategoryRepository, PgCategoryRepository},
        types::{CategoryID, repo::CreateModel},
    },
    error::{
        Resource,
        repo::{ConstraintViolation, Error},
    },
    tests::helpers::create_test_user,
    user::{repo::PgUserRepository, types::UserID},
};

async fn init(pool: PgPool) -> (UserID, CategoryID, PgCategoryRepository) {
    let user_repo = PgUserRepository::init(pool.clone());
    let category_repo = PgCategoryRepository::init(pool.clone());

    let user = create_test_user(&user_repo).await;
    let category = category_repo
        .create(
            user.id,
            CreateModel {
                name: "Test Category".to_string(),
                ..Default::default()
            },
        )
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

        let res = category_repo.delete(category_id, user_id).await;
        assert!(res.is_ok());
    }

    #[test]
    async fn verify_works(pool: PgPool) {
        let (user_id, category_id, category_repo) = init(pool).await;

        category_repo.delete(category_id, user_id).await.unwrap();
        let res = category_repo
            .get_id_by_name(user_id, "Testing".to_string())
            .await;
        assert!(res.is_err());
    }
}

mod existence {
    use super::*;
    use sqlx::test;

    #[test]
    async fn not_owned(pool: PgPool) {
        let (_, category_id, _) = init(pool.clone()).await;
        let (user_id, _, category_repo) = init(pool).await;

        let res = category_repo.delete(category_id, user_id).await;
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
            .delete(CategoryID::new_random(), user_id)
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
