use sqlx::PgPool;

use crate::{
    error::{
        Resource,
        repo::{ConstraintViolation, Error},
    },
    tag::{
        repo::{CreateModel, PgTagRepository, TagRepository, UpdateModel},
        types::TagID,
    },
    tests::helpers::{create_test_user, generate_a_z},
    user::{repo::PgUserRepository, types::UserID},
};

async fn init(pool: PgPool) -> (UserID, TagID, PgTagRepository) {
    let user_repo = PgUserRepository::init(pool.clone());
    let tag_repo = PgTagRepository::init(pool.clone());

    let user = create_test_user(&user_repo).await;
    let tag = tag_repo
        .create(user.id, CreateModel::default())
        .await
        .unwrap();

    (user.id, tag.id, tag_repo)
}

mod success {
    use sqlx::test;

    use super::*;

    #[test]
    async fn success(pool: PgPool) {
        let (user_id, tag_id, tag_repo) = init(pool).await;

        let res = tag_repo
            .update(tag_id, user_id, UpdateModel::default())
            .await;
        assert!(res.is_ok());
    }

    #[test]
    async fn verify_output(pool: PgPool) {
        let (user_id, tag_id, tag_repo) = init(pool).await;

        let category_id = tag_repo
            .add_category(user_id, "Testing".to_string(), generate_a_z(0).to_string())
            .await
            .unwrap();
        let update_model = UpdateModel {
            label: Some("Updated Tag".to_string()),
            category_id: Some(Some(category_id)),
            position_key: Some(generate_a_z(1).to_string()),
        };
        let tag = tag_repo
            .update(tag_id, user_id, update_model.clone())
            .await
            .unwrap();

        let UpdateModel {
            label,
            category_id,
            position_key,
        } = update_model;
        assert_eq!(tag.label, label.unwrap());
        assert_eq!(tag.category_id, category_id.unwrap());
        assert_eq!(tag.category_name.unwrap(), "Testing");
        assert_eq!(tag.position_key, position_key.unwrap());
    }

    #[test]
    async fn updates_updated_at(pool: PgPool) {
        let (user_id, tag_id, tag_repo) = init(pool).await;
        let tag_init = tag_repo.get(tag_id, user_id).await.unwrap().unwrap();

        let tag = tag_repo
            .update(tag_id, user_id, UpdateModel::default())
            .await
            .unwrap();
        assert!(tag.updated_at > tag_init.updated_at);
    }

    #[test]
    async fn is_idempotent(pool: PgPool) {
        let (user_id, tag_id, tag_repo) = init(pool).await;

        let update_model = UpdateModel {
            label: Some("Updated Tag".to_string()),
            ..Default::default()
        };
        let first_update = tag_repo
            .update(tag_id, user_id, update_model.clone())
            .await
            .unwrap();
        let second_update = tag_repo
            .update(tag_id, user_id, update_model.clone())
            .await
            .unwrap();

        assert_eq!(first_update, second_update);
    }
}

mod existence {
    use sqlx::test;

    use super::*;

    #[test]
    async fn not_owned(pool: PgPool) {
        let (_, tag_id, _) = init(pool.clone()).await; // other tag
        let (user_id, _, tag_repo) = init(pool).await;

        let res = tag_repo
            .update(tag_id, user_id, UpdateModel::default())
            .await;
        assert!(res.is_err());
        if let Err(err) = res {
            assert!(matches!(
                err,
                Error::Constraint(ConstraintViolation::NotFound(Resource::Tag))
            ))
        }
    }

    #[test]
    async fn not_exists(pool: PgPool) {
        let (user_id, _, tag_repo) = init(pool).await;

        let res = tag_repo
            .update(TagID::new_v4(), user_id, UpdateModel::default())
            .await;
        assert!(res.is_err());
        if let Err(err) = res {
            assert!(matches!(
                err,
                Error::Constraint(ConstraintViolation::NotFound(Resource::Tag))
            ))
        }
    }
}

mod input {
    use sqlx::test;

    use super::*;

    #[test]
    async fn full_input(pool: PgPool) {
        let (user_id, tag_id, tag_repo) = init(pool).await;

        let category_id = tag_repo
            .add_category(user_id, "Testing".to_string(), generate_a_z(0).to_string())
            .await
            .unwrap();
        let update_model = UpdateModel {
            label: Some("Updated Tag".to_string()),
            category_id: Some(Some(category_id)),
            position_key: Some(format!("full{}", generate_a_z(1))),
        };
        let res = tag_repo.update(tag_id, user_id, update_model).await;
        assert!(res.is_ok());
    }

    #[test]
    async fn null_input(pool: PgPool) {
        let (user_id, _, tag_repo) = init(pool).await;
        let category_id = tag_repo
            .add_category(user_id, "Testing".to_string(), generate_a_z(0).to_string())
            .await
            .unwrap();
        let tag = tag_repo
            .create(
                user_id,
                CreateModel {
                    category_id: Some(category_id),
                    ..Default::default()
                },
            )
            .await
            .unwrap();

        let update_model = UpdateModel {
            category_id: Some(None),
            ..Default::default()
        };
        let res = tag_repo.update(tag.id, user_id, update_model).await;
        assert!(res.is_ok());
        if let Ok(tag) = res {
            assert!(tag.category_id.is_none());
            assert!(tag.category_name.is_none());
        }
    }
}

mod constraint {
    use sqlx::test;

    use super::*;

    #[test]
    async fn errors_on_duplicate_uncategorized_tag(pool: PgPool) {
        let (user_id, _, tag_repo) = init(pool).await;

        let category_id = tag_repo
            .add_category(user_id, "Testing".to_string(), generate_a_z(0).to_string())
            .await
            .unwrap();
        let tag = tag_repo
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
            .update(
                tag.id,
                user_id,
                UpdateModel {
                    label: None,
                    category_id: Some(None),
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

    #[test]
    async fn errors_on_duplicate_categorized_tag(pool: PgPool) {
        let (user_id, tag_id, tag_repo) = init(pool).await;

        let category_id = tag_repo
            .add_category(user_id, "Testing".to_string(), generate_a_z(0).to_string())
            .await
            .unwrap();
        tag_repo
            .update(
                tag_id,
                user_id,
                UpdateModel {
                    label: None,
                    category_id: Some(Some(category_id)),
                    ..Default::default()
                },
            )
            .await
            .unwrap();

        let tag = tag_repo
            .create(user_id, CreateModel::default())
            .await
            .unwrap();
        let res = tag_repo
            .update(
                tag.id,
                user_id,
                UpdateModel {
                    label: None,
                    category_id: Some(Some(category_id)),
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
