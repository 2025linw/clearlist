use sqlx::PgPool;

use crate::{
    error::{
        Resource,
        repo::{ConstraintViolation, Error},
    },
    tag::repo::{CreateModel, PgTagRepository, TagRepository},
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
    use sqlx::test;

    use super::*;

    #[test]
    async fn success(pool: PgPool) {
        let (user_id, tag_repo) = init(pool).await;

        let res = tag_repo.create(user_id, CreateModel::default()).await;
        assert!(res.is_ok());
    }

    #[test]
    async fn verify_output(pool: PgPool) {
        let (user_id, tag_repo) = init(pool).await;

        let test_category_id = tag_repo
            .add_category(user_id, "Testing".to_string(), generate_a_z(0).to_string())
            .await
            .unwrap();
        let create_model = CreateModel {
            label: "Test Tag".to_string(),
            category_id: Some(test_category_id),
            position_key: generate_a_z(0).to_string(),
        };
        let tag = tag_repo
            .create(user_id, create_model.clone())
            .await
            .unwrap();

        let CreateModel {
            label,
            category_id,
            position_key,
        } = create_model;
        assert_eq!(tag.label, label);
        assert_eq!(tag.category_id, category_id);
        assert_eq!(tag.category_name.unwrap(), "Testing");
        assert_eq!(tag.position_key, position_key);
    }
}

mod input {
    use sqlx::test;

    use super::*;

    #[test]
    async fn required_input(pool: PgPool) {
        let (user_id, tag_repo) = init(pool).await;

        let create_model = CreateModel {
            label: "Test Tag".to_string(),
            ..Default::default()
        };

        let res = tag_repo.create(user_id, create_model).await;
        assert!(res.is_ok());
    }

    #[test]
    async fn full_input(pool: PgPool) {
        let (user_id, tag_repo) = init(pool).await;

        let category_id = tag_repo
            .add_category(user_id, "Testing".to_string(), generate_a_z(0).to_string())
            .await
            .unwrap();
        let create_model = CreateModel {
            label: "Test Tag".to_string(),
            category_id: Some(category_id),
            position_key: generate_a_z(0).to_string(),
        };
        let res = tag_repo.create(user_id, create_model).await;
        assert!(res.is_ok());
    }
}

mod constraint {
    use sqlx::test;

    use super::*;

    #[test]
    async fn user_not_exists(pool: PgPool) {
        let (_, tag_repo) = init(pool).await;

        let res = tag_repo
            .create(UserID::new_v4(), CreateModel::default())
            .await;
        assert!(res.is_err());
        if let Err(err) = res {
            assert!(matches!(
                err,
                Error::Constraint(ConstraintViolation::MissingUser)
            ))
        }
    }

    #[test]
    async fn errors_on_duplicate_uncategorized_tag(pool: PgPool) {
        let (user_id, tag_repo) = init(pool).await;

        tag_repo
            .create(user_id, CreateModel::default())
            .await
            .unwrap();

        let res = tag_repo.create(user_id, CreateModel::default()).await;
        assert!(res.is_err());
        if let Err(err) = res {
            assert!(matches!(
                err,
                Error::Constraint(ConstraintViolation::Unique(Resource::Tag))
            ))
        }
    }

    #[test]
    async fn errors_on_duplicate_categorized_tag(pool: PgPool) {
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

        let res = tag_repo
            .create(
                user_id,
                CreateModel {
                    category_id: Some(category_id),
                    ..Default::default()
                },
            )
            .await;
        assert!(res.is_err());
        if let Err(err) = res {
            assert!(matches!(
                err,
                Error::Constraint(ConstraintViolation::Unique(Resource::Tag))
            ))
        }
    }
}
