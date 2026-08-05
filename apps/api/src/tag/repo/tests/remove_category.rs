use sqlx::PgPool;

use crate::{
    error::{
        Resource,
        repo::{ConstraintViolation, Error},
    },
    tag::{
        repo::{PgTagRepository, TagRepository},
        types::CategoryID,
    },
    tests::helpers::{create_test_user, generate_a_z},
    user::{repo::PgUserRepository, types::UserID},
};

async fn init(pool: PgPool) -> (UserID, CategoryID, PgTagRepository) {
    let user_repo = PgUserRepository::init(pool.clone());
    let tag_repo = PgTagRepository::init(pool.clone());

    let user = create_test_user(&user_repo).await;
    let category_id = tag_repo
        .add_category(user.id, "Testing".to_string(), generate_a_z(0).to_string())
        .await
        .unwrap();

    (user.id, category_id, tag_repo)
}

mod success {
    use sqlx::test;

    use super::*;

    #[test]
    async fn success(pool: PgPool) {
        let (user_id, category_id, tag_repo) = init(pool).await;

        let res = tag_repo.remove_category(category_id, user_id).await;
        assert!(res.is_ok());
    }

    #[test]
    async fn verify_works(pool: PgPool) {
        let (user_id, category_id, tag_repo) = init(pool).await;

        tag_repo
            .remove_category(category_id, user_id)
            .await
            .unwrap();
        let category_id_opt = tag_repo
            .get_category_id(user_id, "Testing".to_string())
            .await
            .unwrap();
        assert!(category_id_opt.is_none());
    }
}

mod existence {
    use sqlx::test;

    use super::*;

    #[test]
    async fn not_owned(pool: PgPool) {
        let (_, category_id, _) = init(pool.clone()).await;
        let (user_id, _, tag_repo) = init(pool).await;

        let res = tag_repo.remove_category(category_id, user_id).await;
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
        let (user_id, _, tag_repo) = init(pool).await;

        let res = tag_repo
            .remove_category(CategoryID::new_v4(), user_id)
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
