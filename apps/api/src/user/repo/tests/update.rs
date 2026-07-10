use sqlx::{PgPool, test};

use crate::{
    error::repo::{ConstraintViolation, Error, Resource},
    tests::helpers::create_user_model,
    user::{
        repo::{PgUserRepository, UpdateModel, UserRepository},
        types::UserID,
    },
};

#[test]
async fn update_existing(pool: PgPool) {
    let repo = PgUserRepository::init(pool.clone());

    let test_user = repo.create(create_user_model()).await.unwrap();

    let res = repo
        .update(
            test_user.id,
            UpdateModel {
                display_name: Some(String::from("Updated User")),
                ..Default::default()
            },
        )
        .await;
    assert!(res.is_ok());
}

#[test]
async fn update_nonexistent(pool: PgPool) {
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

#[test]
async fn update_is_idempotent(pool: PgPool) {
    let repo = PgUserRepository::init(pool.clone());

    let test_user = repo.create(create_user_model()).await.unwrap();

    let update_user = UpdateModel {
        display_name: Some(String::from("Updated User")),
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

#[test]
async fn update_full(pool: PgPool) {
    let repo = PgUserRepository::init(pool.clone());

    let test_user = repo.create(create_user_model()).await.unwrap();

    let res = repo
        .update(
            test_user.id,
            UpdateModel {
                display_name: Some(String::from("Updated User")),
            },
        )
        .await;
    assert!(res.is_ok());
    if let Ok(user) = res {
        assert_eq!(user.display_name, "Updated User");
    }
}
