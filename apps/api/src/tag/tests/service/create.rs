use chrono_tz::Tz;

use crate::{
    error::{
        Resource,
        service::{Error, INVALID_SINGLE_LINE, TOO_LONG, ValidationError},
    },
    tag::{service::TagService, types::route::CreateRequest},
    tests::{
        mocks::{MockTagRepository, category::MockCategoryRepository},
        test_data::{NORMALIZATION_TEST_INPUT, SINGLE_LINE_TEST_INPUT},
    },
    types::extract::UserContext,
    user::types::UserID,
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

fn valid_request() -> CreateRequest {
    CreateRequest {
        label: "Test Tag".to_string(),
        ..Default::default()
    }
}

mod success {
    use super::*;
    use tokio::test;

    #[test]
    async fn success() {
        let (user_context, tag_service) = init().await;

        let res = tag_service.create(user_context, valid_request()).await;
        assert!(res.is_ok())
    }
}

mod error {
    use super::*;
    use tokio::test;

    #[test]
    async fn user_not_exists() {
        let (_, tag_service) = init().await;
        let user_context = UserContext {
            id: UserID::new_random(),
            tz: Tz::America__Chicago,
        };

        let res = tag_service.create(user_context, valid_request()).await;
        assert!(res.is_err());
        if let Err(err) = res {
            assert!(matches!(err, Error::Internal(_)));
        }
    }

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

        let res = tag_service.create(user_context, valid_request()).await;
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

        let res = tag_service.create(user_context, valid_request()).await;
        assert!(res.is_err());
        if let Err(err) = res {
            assert!(matches!(err, Error::Internal(_)));
        }
    }
}

mod label {
    use super::*;
    use tokio::test;

    #[test]
    async fn normalize_label() {
        let (user_context, tag_service) = init().await;

        for (name, input, expected) in NORMALIZATION_TEST_INPUT {
            let create_request = CreateRequest {
                label: input.to_string(),
                ..Default::default()
            };
            let tag = tag_service
                .create(user_context, create_request)
                .await
                .unwrap();
            assert_eq!(
                tag.label, expected,
                "case: {name} - expected: {expected} (found: {})",
                tag.label
            );
        }
    }

    #[test]
    async fn errors_with_label_containing_whitespaces() {
        let (user_context, tag_service) = init().await;

        for (name, input) in SINGLE_LINE_TEST_INPUT {
            let create_request = CreateRequest {
                label: input.to_string(),
                ..Default::default()
            };
            let res = tag_service.create(user_context, create_request).await;
            assert!(res.is_err(), "case: {name}; should have failed");
            if let Err(err) = res {
                assert!(
                    matches!(
                        err,
                        Error::Validation(ValidationError::InvalidValue {
                            field: "label",
                            reason: INVALID_SINGLE_LINE
                        })
                    ),
                    "case: {name}; got error: {err}",
                )
            }
        }
    }

    #[test]
    async fn errors_with_label_more_than_100_chars() {
        let (user_context, tag_service) = init().await;

        let mut too_long_label = String::new();
        for _ in 0..12 {
            too_long_label.push_str("1234567890");
        }
        let create_request = CreateRequest {
            label: too_long_label,
            ..Default::default()
        };
        let res = tag_service.create(user_context, create_request).await;
        assert!(res.is_err());
        if let Err(err) = res {
            assert!(
                matches!(
                    err,
                    Error::Validation(ValidationError::InvalidValue {
                        field: "label",
                        reason: TOO_LONG,
                    })
                ),
                "got error: {err}",
            )
        }
    }
}

mod category {
    use super::*;
    use tokio::test;

    #[test]
    async fn category_not_owned() {
        let (user_context, tag_service) = init().await;
        let category = tag_service
            .repo
            .add_category(tag_service.repo.add_user().await)
            .await;

        let create_request = CreateRequest {
            label: "Test Tag".to_string(),
            category: Some(category.name),
            ..Default::default()
        };
        let res = tag_service.create(user_context, create_request).await;
        assert!(res.is_err());
        if let Err(err) = res {
            assert!(matches!(err, Error::NotFound(Resource::Category)), "{err}")
        }
    }

    #[test]
    async fn category_not_exists() {
        let (user_context, tag_service) = init().await;

        let create_request = CreateRequest {
            label: "Test Tag".to_string(),
            category: Some("Not Exists".to_string()),
            ..Default::default()
        };
        let res = tag_service.create(user_context, create_request).await;
        assert!(res.is_err());
        if let Err(err) = res {
            assert!(matches!(err, Error::NotFound(Resource::Category)))
        }
    }
}
