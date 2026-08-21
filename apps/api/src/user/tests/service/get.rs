use crate::{
    error::{Resource, service::Error},
    tests::mocks::MockUserRepository,
    user::{
        service::UserService,
        types::{UserID, route::ProvisionRequest},
    },
};

use super::init_test_setup;

async fn init() -> (UserID, UserService<MockUserRepository>) {
    let user_service = init_test_setup();

    let user = user_service
        .create(ProvisionRequest {
            display_name: "Test User".to_string(),
            ..Default::default()
        })
        .await
        .unwrap();

    (user.id, user_service)
}

mod success {
    use super::*;
    use tokio::test;

    #[test]
    async fn success() {
        let (user_id, user_service) = init().await;

        let res = user_service.get(user_id).await;
        assert!(res.is_ok());
    }
}

mod existence {
    use super::*;
    use tokio::test;

    #[test]
    async fn not_exists() {
        let (_, user_service) = init().await;

        let res = user_service.get(UserID::new_random()).await;
        assert!(res.is_err());
        if let Err(err) = res {
            assert!(matches!(err, Error::NotFound(Resource::User)))
        }
    }
}

mod error {
    use super::*;
    use tokio::test;

    #[test]
    async fn repo_backend_error() {
        let user_service = UserService::init(MockUserRepository::new_backend_error());

        let res = user_service.get(UserID::new_random()).await;
        assert!(res.is_err());
        if let Err(err) = res {
            assert!(matches!(err, Error::Internal(_)));
        }
    }

    #[test]
    async fn repo_programming_error() {
        let user_service = UserService::init(MockUserRepository::new_programming_error());

        let res = user_service.get(UserID::new_random()).await;
        assert!(res.is_err());
        if let Err(err) = res {
            assert!(matches!(err, Error::Internal(_)));
        }
    }
}
