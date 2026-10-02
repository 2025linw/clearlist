use chrono_tz::Tz;

use crate::{
    error::{Resource, service::Error},
    tag::{
        service::TagService,
        types::{TagID, route::CreateRequest},
    },
    tests::mocks::{MockTagRepository, category::MockCategoryRepository},
    types::extract::UserContext,
};

use super::init_test_setup;

async fn init() -> (
    UserContext,
    TagID,
    TagService<MockTagRepository, MockCategoryRepository>,
) {
    let tag_service = init_test_setup();

    let user_context = UserContext {
        id: tag_service.repo.add_user().await,
        tz: Tz::America__Chicago,
    };
    let tag = tag_service
        .create(
            user_context,
            CreateRequest {
                label: "Test Tag".to_string(),
                ..Default::default()
            },
        )
        .await
        .unwrap();

    (user_context, tag.id, tag_service)
}

mod success {
    use super::*;
    use tokio::test;

    #[test]
    async fn success() {
        let (user_context, tag_id, tag_service) = init().await;

        let res = tag_service.get(tag_id, user_context).await;
        assert!(res.is_ok());
    }
}

mod existence {
    use super::*;
    use tokio::test;

    #[test]
    async fn not_owned() {
        let (_, tag_id, tag_service) = init().await;
        let user_context = UserContext {
            id: tag_service.repo.add_user().await,
            tz: Tz::America__Chicago,
        };

        let res = tag_service.get(tag_id, user_context).await;
        assert!(res.is_err());
        if let Err(err) = res {
            assert!(matches!(err, Error::NotFound(Resource::Tag)));
        }
    }

    #[test]
    async fn not_exists() {
        let (user_context, _, tag_service) = init().await;

        let res = tag_service.get(TagID::new_random(), user_context).await;
        assert!(res.is_err());
        if let Err(err) = res {
            assert!(matches!(err, Error::NotFound(Resource::Tag)));
        }
    }
}

mod error {
    use super::*;
    use tokio::test;

    #[test]
    async fn repo_backend_error() {
        let tag_service = TagService::init(
            MockTagRepository::new_backend_error(),
            MockCategoryRepository::new(),
        );
        let user_context = UserContext {
            id: tag_service.repo.add_user().await,
            tz: Tz::America__Chicago,
        };

        let res = tag_service.get(TagID::new_random(), user_context).await;
        assert!(res.is_err());
        if let Err(err) = res {
            assert!(matches!(err, Error::Internal(_)));
        }
    }

    #[test]
    async fn repo_programming_error() {
        let tag_service = TagService::init(
            MockTagRepository::new_internal_error(),
            MockCategoryRepository::new(),
        );
        let user_context = UserContext {
            id: tag_service.repo.add_user().await,
            tz: Tz::America__Chicago,
        };

        let res = tag_service.get(TagID::new_random(), user_context).await;
        assert!(res.is_err());
        if let Err(err) = res {
            assert!(matches!(err, Error::Internal(_)));
        }
    }
}
