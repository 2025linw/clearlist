use crate::{
    error::service::{Error, NO_EMPTY_STRING, NO_NONSPACE_WHITESPACE, ValidationError},
    tests::{
        helpers::get_today_date_pg,
        mocks::user::MockUserRepository,
        test_data::{CONTAINS_WHITESPACE_TEST_INPUT, NORMALIZATION_TEST_INPUT},
    },
    user::{
        service::{CreateRequest, UserService, UserServiceTrait},
        types::UserID,
    },
};

async fn init() -> UserService<MockUserRepository> {
    UserService::init(MockUserRepository::new())
}

fn valid_request() -> CreateRequest {
    CreateRequest {
        id: UserID::new_v4(),
        display_name: "Test User".to_string(),
        preferred_timezone: None,
        completed_task_retention: None,
        created_at: get_today_date_pg(),
    }
}

mod success {
    use tokio::test;

    use super::*;

    #[test]
    async fn success() {
        let user_service = init().await;

        let res = user_service.create(valid_request()).await;
        assert!(res.is_ok())
    }
}

mod error {
    use tokio::test;

    use super::*;

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
            assert!(matches!(err, Error::Unhandled(_)));
        }
    }
}

mod display_name {
    use tokio::test;

    use super::*;

    #[test]
    async fn normalize_display_name() {
        let user_service = init().await;

        for (name, input, expected) in NORMALIZATION_TEST_INPUT {
            let create_request = CreateRequest {
                display_name: input.to_string(),
                ..Default::default()
            };
            let user = user_service.create(create_request.clone()).await.unwrap();
            assert_eq!(
                user.display_name, expected,
                "case: {} - expected: {} (found: {})",
                name, expected, user.display_name
            );
        }
    }

    #[test]
    async fn errors_on_blank_display_name() {
        let user_service = init().await;

        let create_request = CreateRequest {
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

        for (name, input) in CONTAINS_WHITESPACE_TEST_INPUT {
            let create_request = CreateRequest {
                display_name: input.to_string(),
                ..Default::default()
            };
            let res = user_service.create(create_request.clone()).await;
            assert!(res.is_err(), "case: {}; should have failed", name);
            if let Err(err) = res {
                assert!(
                    matches!(
                        err,
                        Error::Validation(ValidationError::InvalidValue {
                            field: "display_name",
                            reason: NO_NONSPACE_WHITESPACE,
                        })
                    ),
                    "case: {}; got error: {}",
                    name,
                    err
                )
            }
        }
    }
}
