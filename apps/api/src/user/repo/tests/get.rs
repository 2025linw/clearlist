use chrono_tz::Tz;
use sqlx::{PgPool, postgres::types::PgInterval, test};

use crate::{
    tests::helpers::get_today_date_pg,
    user::{
        repo::{CreateModel, PgUserRepository, UserRepository},
        types::UserID,
    },
};

// Existence Tests
#[test]
async fn exists(pool: PgPool) {
    let repo = PgUserRepository::init(pool.clone());

    let test_user = repo.create(CreateModel::default()).await.unwrap();

    let res = repo.get(test_user.id).await;
    assert!(res.is_ok());
    if let Ok(user_opt) = res {
        assert!(user_opt.is_some());
    }
}

#[test]
async fn not_exists(pool: PgPool) {
    let repo = PgUserRepository::init(pool.clone());

    let res = repo.get(UserID::new_v4()).await;
    assert!(res.is_ok());
    if let Ok(user_opt) = res {
        assert!(user_opt.is_none());
    }
}

// Output Tests
#[test]
async fn verify_output(pool: PgPool) {
    let repo = PgUserRepository::init(pool.clone());

    let create_user = CreateModel {
        id: UserID::new_v4(),
        display_name: "Test User".to_string(),
        preferred_timezone: Some("America/Chicago".to_string()),
        completed_task_retention: Some(PgInterval {
            months: 0,
            days: 1,
            microseconds: 0,
        }),
        created_at: get_today_date_pg(),
    };
    let test_user = repo.create(create_user.clone()).await.unwrap();

    let user = repo.get(test_user.id).await.unwrap().unwrap();
    assert_eq!(user.id, create_user.id);
    assert_eq!(user.display_name, create_user.display_name);
    assert_eq!(
        user.preferred_timezone,
        Some(Tz::America__Chicago.to_string())
    );
    assert_eq!(
        user.completed_task_retention,
        create_user.completed_task_retention
    );
    assert_eq!(user.created_at, create_user.created_at);
}
