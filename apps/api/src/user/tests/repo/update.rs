use sqlx::{PgPool, postgres::types::PgInterval};

use crate::{
    error::{
        Resource,
        repo::{ConstraintViolation, Error},
    },
    user::{
        repo::{PgUserRepository, UserRepository},
        types::{
            UserID,
            repo::{CreateModel, UpdateModel},
        },
    },
};

async fn init(pool: PgPool) -> (UserID, PgUserRepository) {
    let user_repo = PgUserRepository::init(pool.clone());

    let user = user_repo.create(CreateModel::default()).await.unwrap();

    (user.id, user_repo)
}

mod success {
    use super::*;
    use sqlx::test;

    #[test]
    async fn success(pool: PgPool) {
        let (user_id, user_repo) = init(pool).await;

        let res = user_repo
            .update(
                user_id,
                UpdateModel {
                    display_name: Some("Updated User".to_string()),
                    ..Default::default()
                },
            )
            .await;
        assert!(res.is_ok());
    }

    #[test]
    async fn verify_output(pool: PgPool) {
        let (user_id, user_repo) = init(pool).await;

        let update_model = UpdateModel {
            display_name: Some("Updated User".to_string()),
            preferred_timezone: Some(Some("America/Chicago".to_string())),
            completed_task_retention: Some(Some(PgInterval {
                months: 0,
                days: 1,
                microseconds: 0,
            })),
        };
        let user = user_repo
            .update(user_id, update_model.clone())
            .await
            .unwrap();

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

    #[test]
    async fn updates_updated_at(pool: PgPool) {
        let (user_id, user_repo) = init(pool).await;
        let user_init = user_repo.get(user_id).await.unwrap().unwrap();

        let user = user_repo
            .update(
                user_id,
                UpdateModel {
                    display_name: Some("Updated User".to_string()),
                    ..Default::default()
                },
            )
            .await
            .unwrap();
        assert!(user.updated_at > user_init.updated_at);
    }

    #[test]
    async fn is_idempotent(pool: PgPool) {
        let (user_id, user_repo) = init(pool).await;

        let update_model = UpdateModel {
            display_name: Some("Updated User".to_string()),
            ..Default::default()
        };
        let first_update = user_repo
            .update(user_id, update_model.clone())
            .await
            .unwrap();
        let second_update = user_repo
            .update(user_id, update_model.clone())
            .await
            .unwrap();

        assert_eq!(first_update, second_update);
    }
}

mod existence {
    use super::*;
    use sqlx::test;

    #[test]
    async fn not_exists(pool: PgPool) {
        let (_, user_repo) = init(pool).await;

        let res = user_repo
            .update(
                UserID::new_random(),
                UpdateModel {
                    display_name: Some("Updated User".to_string()),
                    ..Default::default()
                },
            )
            .await;
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
    use super::*;
    use sqlx::test;

    #[test]
    async fn full_input(pool: PgPool) {
        let (user_id, user_repo) = init(pool).await;

        let update_model = UpdateModel {
            display_name: Some("Updated User".to_string()),
            preferred_timezone: Some(Some("America/Chicago".to_string())),
            completed_task_retention: Some(Some(PgInterval {
                months: 0,
                days: 1,
                microseconds: 0,
            })),
        };
        let res = user_repo.update(user_id, update_model).await;
        assert!(res.is_ok());
    }

    #[test]
    async fn null_input(pool: PgPool) {
        let (_, user_repo) = init(pool).await;
        let user = user_repo.create(CreateModel::default()).await.unwrap();

        let update_model = UpdateModel {
            preferred_timezone: Some(None),
            completed_task_retention: Some(None),
            ..Default::default()
        };
        let res = user_repo.update(user.id, update_model).await;
        assert!(res.is_ok());
        if let Ok(user) = res {
            assert!(user.preferred_timezone.is_none());
            assert!(user.completed_task_retention.is_none());
        }
    }
}
