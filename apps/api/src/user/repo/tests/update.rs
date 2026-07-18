use sqlx::{PgPool, postgres::types::PgInterval, test};

use crate::{
    error::repo::{ConstraintViolation, Error, Resource},
    user::{
        repo::{CreateModel, PgUserRepository, UpdateModel, UserRepository},
        types::UserID,
    },
};

// Existence Tests
#[test]
async fn exists(pool: PgPool) {
    let repo = PgUserRepository::init(pool.clone());

    let test_user = repo.create(CreateModel::default()).await.unwrap();

    let res = repo.update(test_user.id, UpdateModel::default()).await;
    assert!(res.is_ok());
}

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

// Input Tests
#[test]
async fn full_input(pool: PgPool) {
    let repo = PgUserRepository::init(pool.clone());

    let test_user = repo.create(CreateModel::default()).await.unwrap();

    let res = repo
        .update(
            test_user.id,
            UpdateModel {
                display_name: Some("Updated User".to_string()),
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
            completed_task_retention: Some(retention_interval),
            ..Default::default()
        })
        .await
        .unwrap();

    let res = repo
        .update(
            test_user.id,
            UpdateModel {
                completed_task_retention: Some(None),
                ..Default::default()
            },
        )
        .await;
    assert!(res.is_ok());
    if let Ok(user) = res {
        assert!(user.completed_task_retention.is_none());
    }
}

// Output Tests
#[test]
async fn verify_output(pool: PgPool) {
    let repo = PgUserRepository::init(pool.clone());

    let test_user = repo.create(CreateModel::default()).await.unwrap();

    let update_user = UpdateModel {
        display_name: Some("Updated User".to_string()),
        completed_task_retention: Some(Some(PgInterval {
            months: 0,
            days: 1,
            microseconds: 0,
        })),
    };
    let user = repo
        .update(test_user.id, update_user.clone())
        .await
        .unwrap();
    assert_eq!(user.display_name, update_user.display_name.unwrap());
    assert_eq!(
        user.completed_task_retention,
        update_user.completed_task_retention.unwrap()
    );
}

// Behavior Tests
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

    let update_user = UpdateModel {
        display_name: Some("Updated User".to_string()),
        ..Default::default()
    };
    let update_1 = repo
        .update(test_user.id, update_user.clone())
        .await
        .unwrap();
    let update_2 = repo
        .update(test_user.id, update_user.clone())
        .await
        .unwrap();

    assert_eq!(update_1, update_2);
}
