use crate::{
    error::service::{
        Error, NO_EMPTY_STRING, NO_NONSPACE_WHITESPACE, NO_ZERO_LIMIT, NO_ZERO_PAGE,
        ValidationError,
    },
    tag::{
        repo::TagRepository,
        service::{TagService, TagServiceTrait},
        types::{repo::QueryOpts, route::URLQueryOpts, service::UserContext},
    },
    tests::{
        helpers::generate_a_z, mocks::tag::MockTagRepository, test_data::NORMALIZATION_TEST_INPUT,
    },
    user::types::UserID,
};

async fn init() -> (UserContext, TagService<MockTagRepository>) {
    let tag_service = TagService::init(MockTagRepository::new());

    let user_context = UserContext {
        id: tag_service.repo.add_user().await,
    };

    (user_context, tag_service)
}

mod success {
    use tokio::test;

    use super::*;

    #[test]
    async fn success() {
        let (user_context, tag_service) = init().await;

        let res = tag_service.list(user_context, None).await;
        assert!(res.is_ok());
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

        let res = tag_service.list(user_context, None).await;
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

        let res = tag_service.list(user_context, None).await;
        assert!(res.is_err());
        if let Err(err) = res {
            assert!(matches!(err, Error::Unhandled(_)));
        }
    }
}

mod pagination {
    use tokio::test;

    use super::*;

    #[test]
    async fn pagination_mapping() {
        let (user_context, tag_service) = init().await;

        let query = URLQueryOpts {
            page: Some(5),
            limit: Some(25),
            ..Default::default()
        };
        let res = tag_service.list(user_context, Some(query)).await;
        assert!(res.is_ok());

        let last_query = tag_service.repo.get_last_filter().await;

        let QueryOpts {
            filter: _,
            pagination,
        } = last_query;
        assert_eq!(pagination.limit, Some(25));
        assert_eq!(pagination.offset, Some(100));
    }

    #[test]
    async fn default_pagination() {
        let (user_context, tag_service) = init().await;

        let res = tag_service.list(user_context, None).await;
        assert!(res.is_ok());

        let last_query = tag_service.repo.get_last_filter().await;

        let QueryOpts {
            filter: _,
            pagination,
        } = last_query;
        assert_eq!(pagination.limit, Some(25));
        assert_eq!(pagination.offset, Some(0));
    }

    #[test]
    async fn limit_caps_at_150() {
        let (user_context, tag_service) = init().await;

        let query = URLQueryOpts {
            limit: Some(200),
            ..Default::default()
        };
        let res = tag_service.list(user_context, Some(query)).await;
        assert!(res.is_ok());

        let last_query = tag_service.repo.get_last_filter().await;

        let QueryOpts {
            filter: _,
            pagination,
        } = last_query;
        assert_eq!(pagination.limit, Some(150));
        assert_eq!(pagination.offset, Some(0));
    }

    #[test]
    async fn errors_with_0_limit() {
        let (user_context, tag_service) = init().await;

        let query = URLQueryOpts {
            limit: Some(0),
            ..Default::default()
        };
        let res = tag_service.list(user_context, Some(query)).await;
        assert!(res.is_err());
        if let Err(err) = res {
            assert!(matches!(
                err,
                Error::Validation(ValidationError::InvalidValue {
                    field: "limit",
                    reason: NO_ZERO_LIMIT
                })
            ))
        }
    }

    #[test]
    async fn errors_with_0_page() {
        let (user_context, tag_service) = init().await;

        let query = URLQueryOpts {
            page: Some(0),
            ..Default::default()
        };
        let res = tag_service.list(user_context, Some(query)).await;
        assert!(res.is_err());
        if let Err(err) = res {
            assert!(matches!(
                err,
                Error::Validation(ValidationError::InvalidValue {
                    field: "page",
                    reason: NO_ZERO_PAGE
                })
            ))
        }
    }
}

mod filter {
    use tokio::test;

    use super::*;

    #[test]
    async fn nonexistent_category_returns_empty() {
        let (user_context, tag_service) = init().await;

        let query = URLQueryOpts {
            category: Some("Nonexistent Category".to_string()),
            ..Default::default()
        };
        let tags = tag_service.list(user_context, Some(query)).await.unwrap();
        assert_eq!(tags.len(), 0);
    }

    #[test]
    async fn normalize_category() {
        let (user_context, tag_service) = init().await;

        let test_category_id = tag_service
            .repo
            .add_category(
                user_context.id,
                "Test Text".to_string(),
                generate_a_z(0).to_string(),
            )
            .await
            .unwrap();

        for (name, input, _) in NORMALIZATION_TEST_INPUT {
            let query = URLQueryOpts {
                category: Some(input.to_string()),
                ..Default::default()
            };
            tag_service.list(user_context, Some(query)).await.unwrap();

            let filter_category_id = tag_service
                .repo
                .get_last_filter()
                .await
                .filter
                .category
                .unwrap();
            assert_eq!(
                filter_category_id, test_category_id,
                "case: {} - expected: {} (found: {})",
                name, test_category_id, filter_category_id
            )
        }
    }

    #[test]
    async fn errors_with_blank_category() {
        let (user_context, tag_service) = init().await;

        let query = URLQueryOpts {
            category: Some("".to_string()),
            ..Default::default()
        };
        let res = tag_service.list(user_context, Some(query)).await;
        assert!(res.is_err());
        if let Err(err) = res {
            assert!(matches!(
                err,
                Error::Validation(ValidationError::InvalidValue {
                    field: "category",
                    reason: NO_EMPTY_STRING
                })
            ))
        }
    }

    #[test]
    async fn errors_category_containing_whitespace() {
        let (user_context, tag_service) = init().await;

        let query = URLQueryOpts {
            category: Some("Test\n\tCategory".to_string()),
            ..Default::default()
        };
        let res = tag_service.list(user_context, Some(query)).await;
        assert!(res.is_err());
        if let Err(err) = res {
            assert!(matches!(
                err,
                Error::Validation(ValidationError::InvalidValue {
                    field: "category",
                    reason: NO_NONSPACE_WHITESPACE
                })
            ))
        }
    }
}
