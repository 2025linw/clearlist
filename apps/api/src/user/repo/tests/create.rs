use sqlx::{PgPool, postgres::types::PgInterval};

use crate::{
    tests::helpers::get_today_date_pg,
    user::{
        repo::{CreateModel, PgUserRepository, UserRepository},
        types::UserID,
    },
};

async fn init(pool: PgPool) -> PgUserRepository {
    PgUserRepository::init(pool.clone())
}

mod success {
    use sqlx::test;

    use super::*;

    #[test]
    async fn success(pool: PgPool) {
        let user_repo = init(pool).await;

        let res = user_repo.create(CreateModel::default()).await;
        assert!(res.is_ok());
    }

    #[test]
    async fn verify_output(pool: PgPool) {
        let user_repo = init(pool).await;

        let create_model = CreateModel {
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
        let test_user = user_repo.create(create_model.clone()).await.unwrap();

        let CreateModel {
            id,
            display_name,
            preferred_timezone,
            completed_task_retention,
            created_at,
        } = create_model;
        assert_eq!(test_user.id, id);
        assert_eq!(test_user.display_name, display_name);
        assert_eq!(test_user.preferred_timezone, preferred_timezone);
        assert_eq!(test_user.completed_task_retention, completed_task_retention);
        assert_eq!(test_user.updated_at, created_at);
        assert_eq!(test_user.created_at, created_at);
    }
}

mod input {
    use sqlx::test;

    use super::*;

    #[test]
    async fn required_input(pool: PgPool) {
        let user_repo = init(pool).await;

        let create_model = CreateModel {
            id: UserID::new_v4(),
            display_name: "Test User".to_string(),
            created_at: get_today_date_pg(),
            ..Default::default()
        };
        let res = user_repo.create(create_model).await;
        assert!(res.is_ok());
    }

    #[test]
    async fn full_input(pool: PgPool) {
        let user_repo = init(pool).await;

        let create_model = CreateModel {
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
        let res = user_repo.create(create_model).await;
        assert!(res.is_ok());
    }
}
