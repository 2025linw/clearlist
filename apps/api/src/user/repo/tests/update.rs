use sqlx::{PgPool, postgres::types::PgInterval};

use crate::{
    error::{
        Resource,
        repo::{ConstraintViolation, Error},
    },
    user::{
        repo::{CreateModel, PgUserRepository, UpdateModel, UserRepository},
        types::UserID,
    },
};

mod success {
    use sqlx::test;

    use super::*;

    #[test]
    async fn success(pool: PgPool) {
        let repo = PgUserRepository::init(pool.clone());

        let test_user = repo.create(CreateModel::default()).await.unwrap();

        let res = repo.update(test_user.id, UpdateModel::default()).await;
        assert!(res.is_ok());
    }

    #[test]
    async fn verify_output(pool: PgPool) {
        let repo = PgUserRepository::init(pool.clone());

        let test_user = repo.create(CreateModel::default()).await.unwrap();

        let update_model = UpdateModel {
            display_name: Some("Updated User".to_string()),
            preferred_timezone: Some(Some("America/Chicago".to_string())),
            completed_task_retention: Some(Some(PgInterval {
                months: 0,
                days: 1,
                microseconds: 0,
            })),
        };
        let user = repo
            .update(test_user.id, update_model.clone())
            .await
            .unwrap();
        {
            let UpdateModel {
                display_name,
                preferred_timezone,
                completed_task_retention,
            } = update_model;

            assert_eq!(user.display_name, display_name.unwrap());
            assert_eq!(user.preferred_timezone, preferred_timezone.unwrap());
            assert_eq!(
                user.completed_task_retention,
                completed_task_retention.unwrap()
            );
        }
    }

    #[test]
    async fn updates_updated_at(pool: PgPool) {
        let repo = PgUserRepository::init(pool.clone());

        let test_user = repo.create(CreateModel::default()).await.unwrap();

        let user = repo
            .update(test_user.id, UpdateModel::default())
            .await
            .unwrap();
        assert!(user.updated_at > test_user.updated_at);
    }

    #[test]
    async fn is_idempotent(pool: PgPool) {
        let repo = PgUserRepository::init(pool.clone());

        let test_user = repo.create(CreateModel::default()).await.unwrap();

        let update_model = UpdateModel {
            display_name: Some("Updated User".to_string()),
            ..Default::default()
        };
        let update_1 = repo
            .update(test_user.id, update_model.clone())
            .await
            .unwrap();
        let update_2 = repo
            .update(test_user.id, update_model.clone())
            .await
            .unwrap();

        assert_eq!(update_1, update_2);
    }
}

mod existence {
    use sqlx::test;

    use super::*;

    #[test]
    async fn not_exists(pool: PgPool) {
        let repo = PgUserRepository::init(pool.clone());

        let res = repo.update(UserID::new_v4(), UpdateModel::default()).await;
        assert!(res.is_err());
        if let Err(err) = res {
            assert!(matches!(
                err,
                Error::Constraint(ConstraintViolation::NotFound(Resource::User))
            ))
        }
    }
}

mod input {
    use sqlx::test;

    use super::*;

    #[test]
    async fn full_input(pool: PgPool) {
        let repo = PgUserRepository::init(pool.clone());

        let test_user = repo.create(CreateModel::default()).await.unwrap();

        let res = repo
            .update(
                test_user.id,
                UpdateModel {
                    display_name: Some("Updated User".to_string()),
                    preferred_timezone: Some(Some("America/Chicago".to_string())),
                    completed_task_retention: Some(Some(PgInterval {
                        months: 0,
                        days: 1,
                        microseconds: 0,
                    })),
                },
            )
            .await;
        assert!(res.is_ok());
    }

    #[test]
    async fn null_input(pool: PgPool) {
        let repo = PgUserRepository::init(pool.clone());

        let retention_interval = PgInterval {
            months: 0,
            days: 1,
            microseconds: 0,
        };
        let test_user = repo
            .create(CreateModel {
                preferred_timezone: Some("America/Chicago".to_string()),
                completed_task_retention: Some(retention_interval),
                ..Default::default()
            })
            .await
            .unwrap();

        let res = repo
            .update(
                test_user.id,
                UpdateModel {
                    preferred_timezone: Some(None),
                    completed_task_retention: Some(None),
                    ..Default::default()
                },
            )
            .await;
        assert!(res.is_ok());
        if let Ok(user) = res {
            assert!(user.preferred_timezone.is_none());
            assert!(user.completed_task_retention.is_none());
        }
    }
}
