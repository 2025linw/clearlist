use sqlx::{PgPool, postgres::types::PgInterval};

use crate::{
    tests::helpers::get_today_date_pg,
    user::{
        repo::{CreateModel, PgUserRepository, UserRepository},
        types::UserID,
    },
};

async fn init(pool: PgPool) -> (UserID, PgUserRepository) {
    let user_repo = PgUserRepository::init(pool.clone());

    let user = user_repo
        .create(CreateModel {
            display_name: "Test User".to_string(),
            ..Default::default()
        })
        .await
        .unwrap();

    (user.id, user_repo)
}

mod success {
    use sqlx::test;

    use super::*;

    #[test]
    async fn success(pool: PgPool) {
        let (user_id, user_repo) = init(pool).await;

        let res = user_repo.get(user_id).await;
        assert!(res.is_ok());
    }

    #[test]
    async fn verify_output(pool: PgPool) {
        let (_, user_repo) = init(pool).await;

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

        let user = user_repo.get(test_user.id).await.unwrap().unwrap();

        let CreateModel {
            id,
            display_name,
            preferred_timezone,
            completed_task_retention,
            created_at,
        } = create_model;
        assert_eq!(user.id, id);
        assert_eq!(user.display_name, display_name);
        assert_eq!(user.preferred_timezone, preferred_timezone);
        assert_eq!(user.completed_task_retention, completed_task_retention);
        assert_eq!(user.created_at, created_at);
    }
}

mod existence {
    use sqlx::test;

    use super::*;

    #[test]
    async fn not_exists(pool: PgPool) {
        let (_, user_repo) = init(pool).await;

        let res = user_repo.get(UserID::new_v4()).await;
        assert!(res.is_ok());
        if let Ok(user_opt) = res {
            assert!(user_opt.is_none());
        }
    }
}
