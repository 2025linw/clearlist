use chrono_tz::Tz;

use crate::{
    error::service::Error,
    category::service::CategoryService, tests::mocks::category::MockCategoryRepository,
    types::extract::UserContext,
};

use super::init_test_setup;

async fn init() -> (UserContext, CategoryService<MockCategoryRepository>) {
    let tag_service = init_test_setup();

    let user_context = UserContext {
        id: tag_service.repo.add_user().await,
        tz: Tz::America__Chicago,
    };

    (user_context, tag_service)
}

mod success {
    use super::*;
    use tokio::test;

    #[test]
    async fn success() {
        let (user_context, tag_service) = init().await;

        let res = tag_service.list(user_context).await;
        assert!(res.is_ok())
    }
}

mod error {
    use super::*;
    use tokio::test;

    #[test]
    async fn repo_backend_error() {
        let category_service = CategoryService::init(MockCategoryRepository::new_backend_error());
        let user_context = UserContext {
            id: category_service.repo.add_user().await,
            tz: Tz::America__Chicago,
        };

        let res = category_service.list(user_context).await;
        assert!(res.is_err());
        if let Err(err) = res {
            assert!(matches!(err, Error::Internal(_)));
        }
    }

    #[test]
    async fn repo_programming_error() {
        let tag_service = CategoryService::init(MockCategoryRepository::new_internal_error());
        let user_context = UserContext {
            id: tag_service.repo.add_user().await,
            tz: Tz::America__Chicago,
        };

        let res = tag_service.list(user_context).await;
        assert!(res.is_err());
        if let Err(err) = res {
            assert!(matches!(err, Error::Internal(_)));
        }
    }
}
