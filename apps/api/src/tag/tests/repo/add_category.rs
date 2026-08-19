use sqlx::PgPool;

use crate::{
    error::{
        Resource,
        repo::{ConstraintViolation, Error},
    },
    tag::{
        repo::{PgTagRepository, TagRepository},
        types::repo::CreateModel,
    },
    tests::helpers::{create_test_user, generate_a_z},
    user::{repo::PgUserRepository, types::UserID},
};

async fn init(pool: PgPool) -> (UserID, PgTagRepository) {
    let user_repo = PgUserRepository::init(pool.clone());
    let tag_repo = PgTagRepository::init(pool.clone());

    let user = create_test_user(&user_repo).await;

    (user.id, tag_repo)
}

mod success {
    use super::*;
    use sqlx::test;

    #[test]
    async fn input(pool: PgPool) {
        let (user_id, tag_repo) = init(pool).await;

        let res = tag_repo
            .add_category(user_id, "Testing".to_string(), generate_a_z(0).to_string())
            .await;
        assert!(res.is_ok())
    }

    #[test]
    async fn verify_output(pool: PgPool) {
        let (user_id, tag_repo) = init(pool).await;

        let category_id = tag_repo
            .add_category(user_id, "Testing".to_string(), generate_a_z(0).to_string())
            .await
            .unwrap();

        tag_repo
            .create(
                user_id,
                CreateModel {
                    category_id: Some(category_id),
                    ..Default::default()
                },
            )
            .await
            .unwrap();
    }
}

mod constraint {
    use super::*;
    use sqlx::test;

    #[test]
    async fn user_not_exists(pool: PgPool) {
        let (_, tag_repo) = init(pool).await;

        let res = tag_repo
            .add_category(
                UserID::new_random(),
                "Test Category".to_string(),
                generate_a_z(0).to_string(),
            )
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
        let (user_id, tag_repo) = init(pool).await;

        tag_repo
            .add_category(
                user_id,
                "Test Category".to_string(),
                generate_a_z(0).to_string(),
            )
            .await
            .unwrap();

        let res = tag_repo
            .add_category(
                user_id,
                "Test Category".to_string(),
                generate_a_z(0).to_string(),
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
