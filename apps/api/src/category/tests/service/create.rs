use chrono_tz::Tz;

use crate::{
    category::{service::CategoryService, types::route::CreateRequest},
    error::service::{Error, INVALID_SINGLE_LINE, NO_EMPTY_STRING, TOO_LONG, ValidationError},
    tests::{
        mocks::category::MockCategoryRepository,
        test_data::{NORMALIZATION_TEST_INPUT, SINGLE_LINE_TEST_INPUT},
    },
    types::extract::UserContext,
    user::types::UserID,
};

use super::init_test_setup;

async fn init() -> (UserContext, CategoryService<MockCategoryRepository>) {
    let category_service = init_test_setup();

    let user_context = UserContext {
        id: category_service.repo.add_user().await,
        tz: Tz::America__Chicago,
    };

    (user_context, category_service)
}

fn valid_request() -> CreateRequest {
    CreateRequest {
        name: "Test Category".to_string(),
        ..Default::default()
    }
}

mod success {
    use super::*;
    use tokio::test;

    #[test]
    async fn success() {
        let (user_context, category_service) = init().await;

        let res = category_service.create(user_context, valid_request()).await;
        assert!(res.is_ok())
    }
}

mod error {
    use super::*;
    use tokio::test;

    #[test]
    async fn user_not_exists() {
        let (_, category_service) = init().await;
        let user_context = UserContext {
            id: UserID::new_random(),
            tz: Tz::America__Chicago,
        };

        let res = category_service.create(user_context, valid_request()).await;
        assert!(res.is_err());
        if let Err(err) = res {
            assert!(matches!(err, Error::Internal(_)));
        }
    }

    #[test]
    async fn repo_backend_error() {
        let category_service = CategoryService::init(MockCategoryRepository::new_backend_error());
        let user_context = UserContext {
            id: category_service.repo.add_user().await,
            tz: Tz::America__Chicago,
        };

        let res = category_service.create(user_context, valid_request()).await;
        assert!(res.is_err());
        if let Err(err) = res {
            assert!(matches!(err, Error::Internal(_)));
        }
    }

    #[test]
    async fn repo_programming_error() {
        let category_service = CategoryService::init(MockCategoryRepository::new_internal_error());
        let user_context = UserContext {
            id: category_service.repo.add_user().await,
            tz: Tz::America__Chicago,
        };

        let res = category_service.create(user_context, valid_request()).await;
        assert!(res.is_err());
        if let Err(err) = res {
            assert!(matches!(err, Error::Internal(_)));
        }
    }
}

mod name {
    use super::*;
    use tokio::test;

    #[test]
    async fn normalize_name() {
        let (user_context, category_service) = init().await;

        for (name, input, expected) in NORMALIZATION_TEST_INPUT {
            let create_request = CreateRequest {
                name: input.to_string(),
                ..Default::default()
            };
            let category = category_service
                .create(user_context, create_request)
                .await
                .unwrap();
            let category_name = category.name;
            assert_eq!(
                category_name, expected,
                "case: {name} - expected: {expected} (found: {category_name})",
            );
            category_service.delete(category.id, user_context).await.unwrap();
        }
    }

    #[test]
    async fn errors_on_blank_name() {
        let (user_context, category_service) = init().await;

        let create_request = CreateRequest {
            name: "".to_string(),
            ..Default::default()
        };
        let res = category_service.create(user_context, create_request).await;
        assert!(res.is_err());
        if let Err(err) = res {
            assert!(matches!(
                err,
                Error::Validation(ValidationError::InvalidValue {
                    field: "name",
                    reason: NO_EMPTY_STRING,
                })
            ))
        }
    }

    #[test]
    async fn errors_with_name_containing_whitespaces() {
        let (user_context, category_service) = init().await;

        for (name, input) in SINGLE_LINE_TEST_INPUT {
            let create_request = CreateRequest {
                name: input.to_string(),
                ..Default::default()
            };
            let res = category_service.create(user_context, create_request).await;
            assert!(res.is_err(), "case: {name}; should have failed");
            if let Err(err) = res {
                assert!(
                    matches!(
                        err,
                        Error::Validation(ValidationError::InvalidValue {
                            field: "name",
                            reason: INVALID_SINGLE_LINE
                        })
                    ),
                    "case: {name}; got error: {err}",
                )
            }
        }
    }

    #[test]
    async fn errors_with_name_more_than_100_chars() {
        let (user_context, category_service) = init().await;

        let mut too_long_category = String::new();
        for _ in 0..12 {
            too_long_category.push_str("1234567890");
        }
        let create_request = CreateRequest {
            name: too_long_category,
            ..Default::default()
        };
        let res = category_service.create(user_context, create_request).await;
        assert!(res.is_err());
        if let Err(err) = res {
            assert!(
                matches!(
                    err,
                    Error::Validation(ValidationError::InvalidValue {
                        field: "name",
                        reason: TOO_LONG,
                    })
                ),
                "got error: {err}",
            )
        }
    }
}
