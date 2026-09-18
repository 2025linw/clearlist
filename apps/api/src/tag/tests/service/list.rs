use chrono_tz::Tz;

use crate::{
    category::types::repo::CreateModel as CategoryCreateModel,
    error::service::{
        Error, INVALID_SINGLE_LINE, NO_EMPTY_STRING, NO_ZERO_LIMIT, NO_ZERO_PAGE, TOO_LONG,
        ValidationError,
    },
    tag::{
        service::TagService,
        types::{repo::QueryOpts, route::URLQueryOpts},
    },
    tests::{
        mocks::{MockTagRepository, category::MockCategoryRepository},
        test_data::NORMALIZATION_TEST_INPUT,
    },
    types::extract::UserContext,
};

use super::init_test_setup;

async fn init() -> (
    UserContext,
    TagService<MockTagRepository, MockCategoryRepository>,
) {
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

        let res = tag_service
            .list(user_context, URLQueryOpts::default())
            .await;
        assert!(res.is_ok());
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

        let res = tag_service
            .list(user_context, URLQueryOpts::default())
            .await;
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

        let res = tag_service
            .list(user_context, URLQueryOpts::default())
            .await;
        assert!(res.is_err());
        if let Err(err) = res {
            assert!(matches!(err, Error::Internal(_)));
        }
    }
}

mod pagination {
    use super::*;
    use tokio::test;

    #[test]
    async fn pagination_mapping() {
        let (user_context, tag_service) = init().await;

        let query = URLQueryOpts {
            page: Some(5),
            limit: Some(25),
            ..Default::default()
        };
        let res = tag_service.list(user_context, query).await;
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

        let res = tag_service
            .list(user_context, URLQueryOpts::default())
            .await;
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
        let res = tag_service.list(user_context, query).await;
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
        let res = tag_service.list(user_context, query).await;
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
        let res = tag_service.list(user_context, query).await;
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
    use crate::category::repo::CategoryRepository;

    use super::*;
    use tokio::test;

    #[test]
    async fn nonexistent_category_returns_empty() {
        let (user_context, tag_service) = init().await;

        let query = URLQueryOpts {
            category: Some("URLQueryOpts::default()xistent Category".to_string()),
            ..Default::default()
        };
        let tags = tag_service.list(user_context, query).await.unwrap();
        assert_eq!(tags.len(), 0);
    }

    #[test]
    async fn normalize_category() {
        let (user_context, tag_service) = init().await;

        let test_category = tag_service
            .category_repo
            .create(
                user_context.id,
                CategoryCreateModel {
                    name: "Test Text".to_string(),
                    ..Default::default()
                },
            )
            .await
            .unwrap();

        for (name, input, _) in NORMALIZATION_TEST_INPUT {
            let query = URLQueryOpts {
                category: Some(input.to_string()),
                ..Default::default()
            };
            tag_service.list(user_context, query).await.unwrap();

            let filter_category_id = tag_service
                .repo
                .get_last_filter()
                .await
                .filter
                .category
                .unwrap();
            assert_eq!(
                filter_category_id, test_category.id,
                "case: {name} - expected: {} (found: {filter_category_id})",
                test_category.id
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
        let res = tag_service.list(user_context, query).await;
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
        let res = tag_service.list(user_context, query).await;
        assert!(res.is_err());
        if let Err(err) = res {
            assert!(matches!(
                err,
                Error::Validation(ValidationError::InvalidValue {
                    field: "category",
                    reason: INVALID_SINGLE_LINE
                })
            ))
        }
    }

    #[test]
    async fn errors_with_category_more_than_100_chars() {
        let (user_context, tag_service) = init().await;

        let mut too_long_category = String::new();
        for _ in 0..12 {
            too_long_category.push_str("1234567890");
        }
        let query = URLQueryOpts {
            category: Some(too_long_category),
            ..Default::default()
        };
        let res = tag_service.list(user_context, query).await;
        assert!(res.is_err());
        if let Err(err) = res {
            assert!(matches!(
                err,
                Error::Validation(ValidationError::InvalidValue {
                    field: "category",
                    reason: TOO_LONG,
                })
            ),)
        }
    }
}
