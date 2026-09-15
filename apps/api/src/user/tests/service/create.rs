use crate::{
    error::service::{Error, INVALID_SINGLE_LINE, NO_EMPTY_STRING, ValidationError},
    tests::{
        helpers::get_today_date_pg,
        mocks::MockUserRepository,
        test_data::{NORMALIZATION_TEST_INPUT, SINGLE_LINE_TEST_INPUT},
    },
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
        display_name: "Test User".to_string(),
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

mod display_name {
    use super::*;
    use tokio::test;

    #[test]
    async fn normalize_display_name() {
        let user_service = init().await;

        for (name, input, expected) in NORMALIZATION_TEST_INPUT {
            let create_request = ProvisionRequest {
                display_name: input.to_string(),
                ..Default::default()
            };
            let user = user_service.create(create_request.clone()).await.unwrap();
            assert_eq!(
                user.display_name, expected,
                "case: {name} - expected: {expected} (found: {})",
                user.display_name
            );
        }
    }

    #[test]
    async fn errors_on_blank_display_name() {
        let user_service = init().await;

        let create_request = ProvisionRequest {
            display_name: "".to_string(),
            ..Default::default()
        };
        let res = user_service.create(create_request).await;
        assert!(res.is_err());
        if let Err(err) = res {
            assert!(matches!(
                err,
                Error::Validation(ValidationError::InvalidValue {
                    field: "display_name",
                    reason: NO_EMPTY_STRING,
                })
            ),)
        }
    }

    #[test]
    async fn errors_with_display_name_containing_whitespaces() {
        let user_service = init().await;

        for (name, input) in SINGLE_LINE_TEST_INPUT {
            let create_request = ProvisionRequest {
                display_name: input.to_string(),
                ..Default::default()
            };
            let res = user_service.create(create_request.clone()).await;
            assert!(res.is_err(), "case: {name}; should have failed");
            if let Err(err) = res {
                assert!(
                    matches!(
                        err,
                        Error::Validation(ValidationError::InvalidValue {
                            field: "display_name",
                            reason: INVALID_SINGLE_LINE,
                        })
                    ),
                    "case: {name}; got error: {err}",
                )
            }
        }
    }
}
