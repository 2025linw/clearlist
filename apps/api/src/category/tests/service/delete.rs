use chrono_tz::Tz;

use crate::{
    category::{
        service::CategoryService,
        types::{CategoryID, route::CreateRequest},
    },
    error::{Resource, service::Error},
    tests::mocks::category::MockCategoryRepository,
    types::extract::UserContext,
};

use super::init_test_setup;

async fn init() -> (
    UserContext,
    CategoryID,
    CategoryService<MockCategoryRepository>,
) {
    let category_service = init_test_setup();

    let user_context = UserContext {
        id: category_service.repo.add_user().await,
        tz: Tz::America__Chicago,
    };
    let category = category_service
        .create(
            user_context,
            CreateRequest {
                name: "Test Category".to_string(),
                ..Default::default()
            },
        )
        .await
        .unwrap();

    (user_context, category.id, category_service)
}

mod success {
    use super::*;
    use tokio::test;

    #[test]
    async fn success() {
        let (user_context, category_id, category_service) = init().await;

        let res = category_service.delete(category_id, user_context).await;
        assert!(res.is_ok());
    }

    #[test]
    async fn is_idempotent() {
        let (user_context, category_id, category_service) = init().await;

        let res = category_service.delete(category_id, user_context).await;
        assert!(res.is_ok());
        let res = category_service.delete(category_id, user_context).await;
        assert!(res.is_ok());

        let res = category_service.get(category_id, user_context).await;
        assert!(res.is_err());
        if let Err(err) = res {
            assert!(matches!(err, Error::NotFound(Resource::Category)))
        }
    }
}

mod existence {
    use super::*;
    use tokio::test;

    #[test]
    async fn not_owned() {
        let (_, category_id, category_service) = init().await;
        let user_context = UserContext {
            id: category_service.repo.add_user().await,
            tz: Tz::America__Chicago,
        };

        let res = category_service.delete(category_id, user_context).await;
        assert!(res.is_ok());
    }

    #[test]
    async fn not_exists() {
        let (user_context, _, category_service) = init().await;

        let res = category_service
            .delete(CategoryID::new_random(), user_context)
            .await;
        assert!(res.is_ok());
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

        let res = category_service
            .delete(CategoryID::new_random(), user_context)
            .await;
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

        let res = tag_service
            .delete(CategoryID::new_random(), user_context)
            .await;
        assert!(res.is_err());
        if let Err(err) = res {
            assert!(matches!(err, Error::Internal(_)));
        }
    }
}
