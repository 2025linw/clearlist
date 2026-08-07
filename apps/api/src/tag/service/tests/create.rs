use crate::{
    error::service::{Error, NO_EMPTY_STRING, NO_NONSPACE_WHITESPACE, TOO_LONG, ValidationError},
    tag::{
        service::{TagService, TagServiceTrait, UserContext},
        types::route::CreateRequest,
    },
    tests::{
        mocks::tag::MockTagRepository,
        test_data::{CONTAINS_WHITESPACE_TEST_INPUT, NORMALIZATION_TEST_INPUT},
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

fn valid_request() -> CreateRequest {
    CreateRequest {
        label: "Test Tag".to_string(),
        ..Default::default()
    }
}

mod success {
    use tokio::test;

    use super::*;

    #[test]
    async fn success() {
        let (user_context, tag_service) = init().await;

        let res = tag_service.create(user_context, valid_request()).await;
        assert!(res.is_ok())
    }
}

mod error {
    use tokio::test;

    use super::*;

    #[test]
    async fn user_not_exists() {
        let (_, tag_service) = init().await;
        let user_context = UserContext {
            id: UserID::new_v4(),
        };

        let res = tag_service.create(user_context, valid_request()).await;
        assert!(res.is_err());
        if let Err(err) = res {
            assert!(matches!(err, Error::Internal(_)));
        }
    }

    #[test]
    async fn repo_backend_error() {
        let tag_service = TagService::init(MockTagRepository::new_backend_error());
        let user_context = UserContext {
            id: UserID::new_v4(),
        };

        let res = tag_service.create(user_context, valid_request()).await;
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

        let res = tag_service.create(user_context, valid_request()).await;
        assert!(res.is_err());
        if let Err(err) = res {
            assert!(matches!(err, Error::Unhandled(_)));
        }
    }
}

mod label {
    use tokio::test;

    use super::*;

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
                "case: {} - expected: {} (found: {})",
                name, expected, tag.label
            );
        }
    }

    #[test]
    async fn errors_with_label_containing_whitespaces() {
        let (user_context, tag_service) = init().await;

        for (name, input) in CONTAINS_WHITESPACE_TEST_INPUT {
            let create_request = CreateRequest {
                label: input.to_string(),
                ..Default::default()
            };
            let res = tag_service.create(user_context, create_request).await;
            assert!(res.is_err(), "case: {}; should have failed", name);
            if let Err(err) = res {
                assert!(
                    matches!(
                        err,
                        Error::Validation(ValidationError::InvalidValue {
                            field: "label",
                            reason: NO_NONSPACE_WHITESPACE
                        })
                    ),
                    "case: {}; got error: {}",
                    name,
                    err
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
                "got error: {}",
                err
            )
        }
    }
}

mod category {
    use tokio::test;

    use super::*;

    #[test]
    async fn normalize_category() {
        let (user_context, tag_service) = init().await;

        for (name, input, expected) in NORMALIZATION_TEST_INPUT {
            let create_request = CreateRequest {
                category: Some(input.to_string()),
                ..Default::default()
            };
            let tag = tag_service
                .create(user_context, create_request)
                .await
                .unwrap();
            let category = tag.category_name.unwrap();
            assert_eq!(
                category, expected,
                "case: {} - expected: {} (found: {})",
                name, expected, category
            );
        }
    }

    #[test]
    async fn errors_on_blank_category() {
        let (user_context, tag_service) = init().await;

        let create_request = CreateRequest {
            category: Some("".to_string()),
            ..Default::default()
        };
        let res = tag_service.create(user_context, create_request).await;
        assert!(res.is_err());
        if let Err(err) = res {
            assert!(matches!(
                err,
                Error::Validation(ValidationError::InvalidValue {
                    field: "category",
                    reason: NO_EMPTY_STRING,
                })
            ))
        }
    }

    #[test]
    async fn errors_with_category_containing_whitespaces() {
        let (user_context, tag_service) = init().await;

        for (name, input) in CONTAINS_WHITESPACE_TEST_INPUT {
            let create_request = CreateRequest {
                category: Some(input.to_string()),
                ..Default::default()
            };
            let res = tag_service.create(user_context, create_request).await;
            assert!(res.is_err(), "case: {}; should have failed", name);
            if let Err(err) = res {
                assert!(
                    matches!(
                        err,
                        Error::Validation(ValidationError::InvalidValue {
                            field: "category",
                            reason: NO_NONSPACE_WHITESPACE
                        })
                    ),
                    "case: {}; got error: {}",
                    name,
                    err
                )
            }
        }
    }

    #[test]
    async fn errors_with_category_more_than_100_chars() {
        let (user_context, tag_service) = init().await;

        let mut too_long_category = String::new();
        for _ in 0..12 {
            too_long_category.push_str("1234567890");
        }
        let create_request = CreateRequest {
            category: Some(too_long_category),
            ..Default::default()
        };
        let res = tag_service.create(user_context, create_request).await;
        assert!(res.is_err());
        if let Err(err) = res {
            assert!(
                matches!(
                    err,
                    Error::Validation(ValidationError::InvalidValue {
                        field: "category",
                        reason: TOO_LONG,
                    })
                ),
                "got error: {}",
                err
            )
        }
    }
}
