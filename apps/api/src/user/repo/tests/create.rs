use sqlx::{PgPool, postgres::types::PgInterval, test};

use crate::{
    tests::helpers::get_today_date_pg,
    user::{
        repo::{CreateModel, PgUserRepository, UserRepository},
        types::UserID,
    },
};

// Input Tests
#[test]
async fn required_input(pool: PgPool) {
    let repo = PgUserRepository::init(pool.clone());

    let res = repo
        .create(CreateModel {
            id: UserID::new_v4(),
            display_name: "Test User".to_string(),
            created_at: get_today_date_pg(),
            ..Default::default()
        })
        .await;
    assert!(res.is_ok());
}

#[test]
async fn full_input(pool: PgPool) {
    let repo = PgUserRepository::init(pool.clone());

    let res = repo
        .create(CreateModel {
            id: UserID::new_v4(),
            display_name: "Test User".to_string(),
            preferred_timezone: Some("America/Chicago".to_string()),
            completed_task_retention: Some(PgInterval {
                months: 0,
                days: 1,
                microseconds: 0,
            }),
            created_at: get_today_date_pg(),
        })
        .await;
    assert!(res.is_ok());
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
    assert_eq!(test_user.id, create_user.id);
    assert_eq!(test_user.display_name, create_user.display_name);
    assert_eq!(
        test_user.completed_task_retention,
        create_user.completed_task_retention
    );
    assert_eq!(test_user.updated_at, create_user.created_at);
    assert_eq!(test_user.created_at, create_user.created_at);
}
