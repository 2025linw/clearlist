use sqlx::PgPool;

use crate::{
    category::{repo::PgCategoryRepository, types::CategoryID},
    error::{
        Resource,
        repo::{ConstraintViolation, Error},
    },
    tag::{
        repo::{PgTagRepository, TagRepository},
        types::repo::CreateModel,
    },
    tests::helpers::{create_test_category, create_test_user, generate_a_z},
    user::{repo::PgUserRepository, types::UserID},
};

async fn init(pool: PgPool) -> (UserID, CategoryID, PgTagRepository) {
    let user_repo = PgUserRepository::init(pool.clone());
    let category_repo = PgCategoryRepository::init(pool.clone());
    let tag_repo = PgTagRepository::init(pool.clone());

    let user = create_test_user(&user_repo).await;
    let category = create_test_category(&category_repo, user.id).await;

    (user.id, category.id, tag_repo)
}

mod success {
    use super::*;
    use sqlx::test;

    #[test]
    async fn success(pool: PgPool) {
        let (user_id, _, tag_repo) = init(pool).await;

        let res = tag_repo.create(user_id, CreateModel::default()).await;
        assert!(res.is_ok());
    }

    #[test]
    async fn verify_output(pool: PgPool) {
        let (user_id, category_id, tag_repo) = init(pool).await;

        let create_model = CreateModel {
            label: "Test Tag".to_string(),
            category_id: Some(category_id),
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
        assert_eq!(tag.category_name.unwrap(), "Test Category");
        assert_eq!(tag.position_key, position_key);
    }
}

mod input {
    use super::*;
    use sqlx::test;

    #[test]
    async fn required_input(pool: PgPool) {
        let (user_id, _, tag_repo) = init(pool).await;

        let create_model = CreateModel {
            label: "Test Tag".to_string(),
            ..Default::default()
        };

        let res = tag_repo.create(user_id, create_model).await;
        assert!(res.is_ok());
    }

    #[test]
    async fn full_input(pool: PgPool) {
        let (user_id, category_id, tag_repo) = init(pool).await;

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
    use super::*;
    use sqlx::test;

    #[test]
    async fn user_not_exists(pool: PgPool) {
        let (_, _, tag_repo) = init(pool).await;

        let res = tag_repo
            .create(UserID::new_random(), CreateModel::default())
            .await;
        assert!(res.is_err());
        if let Err(err) = res {
            assert!(matches!(
                err,
                Error::Constraint(ConstraintViolation::ForeignKey {
                    resource: Resource::Tag,
                    message: _
                })
            ))
        }
    }

    #[test]
    async fn errors_on_duplicate_uncategorized_tag(pool: PgPool) {
        let (user_id, _, tag_repo) = init(pool).await;

        tag_repo
            .create(user_id, CreateModel::default())
            .await
            .unwrap();

        let res = tag_repo.create(user_id, CreateModel::default()).await;
        assert!(res.is_err());
        if let Err(err) = res {
            assert!(matches!(
                err,
                Error::Constraint(ConstraintViolation::Unique {
                    resource: Resource::Tag,
                    message: _
                })
            ))
        }
    }

    #[test]
    async fn errors_on_duplicate_categorized_tag(pool: PgPool) {
        let (user_id, category_id, tag_repo) = init(pool).await;

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
                Error::Constraint(ConstraintViolation::Unique {
                    resource: Resource::Tag,
                    message: _
                })
            ))
        }
    }
}
