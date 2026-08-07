use crate::{
    error::{Resource, service::Error},
    tag::{
        service::{TagService, TagServiceTrait},
        types::{TagID, route::CreateRequest, service::UserContext},
    },
    tests::mocks::tag::MockTagRepository,
    user::types::UserID,
};

async fn init() -> (UserContext, TagID, TagService<MockTagRepository>) {
    let tag_service = TagService::init(MockTagRepository::new());

    let user_context = UserContext {
        id: tag_service.repo.add_user().await,
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
    use tokio::test;

    use super::*;

    #[test]
    async fn success() {
        let (user_context, tag_id, tag_service) = init().await;

        let res = tag_service.get(tag_id, user_context).await;
        assert!(res.is_ok());
    }
}

mod existence {
    use tokio::test;

    use super::*;

    #[test]
    async fn not_owned() {
        let (_, tag_id, tag_service) = init().await;
        let user_context = UserContext {
            id: tag_service.repo.add_user().await,
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

        let res = tag_service.get(TagID::new_v4(), user_context).await;
        assert!(res.is_err());
        if let Err(err) = res {
            assert!(matches!(err, Error::NotFound(Resource::Tag)));
        }
    }
}

mod error {
    use tokio::test;

    use super::*;

    #[test]
    async fn repo_backend_error() {
        let tag_service = TagService::init(MockTagRepository::new_backend_error());
        let user_context = UserContext {
            id: UserID::new_v4(),
        };

        let res = tag_service.get(TagID::new_v4(), user_context).await;
        assert!(res.is_err());
        if let Err(err) = res {
            assert!(matches!(err, Error::Internal(_)));
        }
    }

    #[test]
    async fn repo_programming_error() {
        let tag_service = TagService::init(MockTagRepository::new_programming_error());
        let user_context = UserContext {
            id: UserID::new_v4(),
        };

        let res = tag_service.get(TagID::new_v4(), user_context).await;
        assert!(res.is_err());
        if let Err(err) = res {
            assert!(matches!(err, Error::Unhandled(_)));
        }
    }
}
