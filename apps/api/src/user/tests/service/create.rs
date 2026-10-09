use crate::{
    error::service::Error,
    tests::{helpers::get_today_date_pg, mocks::MockUserRepository},
    user::{
        service::UserService,
        types::{UserID, route::ProvisionRequest},
    },
};

use super::init_test_setup;

async fn init() -> UserService<MockUserRepository> {
    init_test_setup()
}

fn valid_request() -> ProvisionRequest {
    ProvisionRequest {
        id: UserID::new_random(),
        created_at: get_today_date_pg(),
    }
}

mod success {
    use super::*;
    use tokio::test;

    #[test]
    async fn success() {
        let user_service = init().await;

        let res = user_service.create(valid_request()).await;
        assert!(res.is_ok())
    }
}

mod error {
    use super::*;
    use tokio::test;

    #[test]
    async fn repo_backend_error() {
        let user_service = UserService::init(MockUserRepository::new_backend_error());

        let res = user_service.create(valid_request()).await;
        assert!(res.is_err());
        if let Err(err) = res {
            assert!(matches!(err, Error::Internal(_)));
        }
    }

    #[test]
    async fn repo_programming_error() {
        let user_service = UserService::init(MockUserRepository::new_programming_error());

        let res = user_service.create(valid_request()).await;
        assert!(res.is_err());
        if let Err(err) = res {
            assert!(matches!(err, Error::Internal(_)));
        }
    }
}
