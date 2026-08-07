use crate::{
    error::{
        Resource,
        service::{Error, NO_EMPTY_STRING, NO_NONSPACE_WHITESPACE, ValidationError},
    },
    tests::{
        mocks::user::MockUserRepository,
        test_data::{CONTAINS_WHITESPACE_TEST_INPUT, NORMALIZATION_TEST_INPUT},
    },
    user::{
        service::{UserService, UserServiceTrait},
        types::{
            UserID,
            route::{CreateRequest, UpdateRequest},
        },
    },
};

async fn init() -> (UserID, UserService<MockUserRepository>) {
    let user_service = UserService::init(MockUserRepository::new());

    let user = user_service
        .create(CreateRequest {
            display_name: "Test User".to_string(),
            ..Default::default()
        })
        .await
        .unwrap();

    (user.id, user_service)
}

fn valid_request() -> UpdateRequest {
    UpdateRequest {
        display_name: Some("Updated User".to_string()),
        ..Default::default()
    }
}

mod success {
    use tokio::test;

    use super::*;

    #[test]
    async fn success() {
        let (user_id, user_service) = init().await;

        let res = user_service.update(user_id, valid_request()).await;
        assert!(res.is_ok());
    }

    #[test]
    async fn no_ops() {
        let (user_id, user_service) = init().await;
        let init_user = user_service.get(user_id).await.unwrap();

        let user = user_service
            .update(user_id, UpdateRequest::default())
            .await
            .unwrap();
        assert_eq!(user, init_user);
    }

    #[test]
    async fn is_idempotent() {
        let (user_id, user_service) = init().await;

        let update_request = UpdateRequest {
            display_name: Some("Updated User".to_string()),
            ..Default::default()
        };
        let first_update = user_service
            .update(user_id, update_request.clone())
            .await
            .unwrap();
        let second_update = user_service
            .update(user_id, update_request.clone())
            .await
            .unwrap();

        assert_eq!(first_update, second_update);
    }
}

mod existence {
    use tokio::test;

    use super::*;

    #[test]
    async fn not_exists() {
        let (_, user_service) = init().await;

        let res = user_service.update(UserID::new_v4(), valid_request()).await;
        assert!(res.is_err());
        if let Err(err) = res {
            assert!(matches!(err, Error::NotFound(Resource::User)))
        }
    }
}

mod error {
    use tokio::test;

    use super::*;

    #[test]
    async fn repo_backend_error() {
        let user_service = UserService::init(MockUserRepository::new_backend_error());

        let res = user_service.update(UserID::new_v4(), valid_request()).await;
        assert!(res.is_err());
        if let Err(err) = res {
            assert!(matches!(err, Error::Internal(_)));
        }
    }

    #[test]
    async fn repo_programming_error() {
        let user_service = UserService::init(MockUserRepository::new_programming_error());

        let res = user_service.update(UserID::new_v4(), valid_request()).await;
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
        let (user_id, user_service) = init().await;

        for (name, input, expected) in NORMALIZATION_TEST_INPUT {
            let update_request = UpdateRequest {
                display_name: Some(input.to_string()),
                ..Default::default()
            };
            let user = user_service.update(user_id, update_request).await.unwrap();
            assert_eq!(
                user.display_name, expected,
                "case: {} - expected: {} (found: {})",
                name, expected, user.display_name
            );
        }
    }

    #[test]
    async fn errors_on_blank_display_name() {
        let (user_id, user_service) = init().await;

        let update_request = UpdateRequest {
            display_name: Some("".to_string()),
            ..Default::default()
        };
        let res = user_service.update(user_id, update_request).await;
        assert!(res.is_err());
        if let Err(err) = res {
            assert!(matches!(
                err,
                Error::Validation(ValidationError::InvalidValue {
                    field: "display_name",
                    reason: NO_EMPTY_STRING,
                })
            ))
        }
    }

    #[test]
    async fn errors_with_display_name_containing_whitespaces() {
        let (user_id, user_service) = init().await;

        for (name, input) in CONTAINS_WHITESPACE_TEST_INPUT {
            let update_request = UpdateRequest {
                display_name: Some(input.to_string()),
                ..Default::default()
            };
            let res = user_service.update(user_id, update_request).await;
            assert!(res.is_err(), "case: {}; should have failed", name);
            if let Err(err) = res {
                assert!(
                    matches!(
                        err,
                        Error::Validation(ValidationError::InvalidValue {
                            field: "display_name",
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
}
