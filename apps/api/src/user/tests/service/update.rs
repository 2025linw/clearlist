use crate::{
    error::{
        Resource,
        service::{Error, INVALID_SINGLE_LINE, NO_EMPTY_STRING, ValidationError},
    },
    tests::{
        mocks::MockUserRepository,
        test_data::{NORMALIZATION_TEST_INPUT, SINGLE_LINE_TEST_INPUT},
    },
    user::{
        service::UserService,
        types::{
            UserID,
            route::{ProvisionRequest, UpdateRequest},
        },
    },
};

use super::init_test_setup;

async fn init() -> (UserID, UserService<MockUserRepository>) {
    let user_service = init_test_setup();

    let user = user_service
        .create(ProvisionRequest {
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
    use super::*;
    use tokio::test;

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
    use super::*;
    use tokio::test;

    #[test]
    async fn not_exists() {
        let (_, user_service) = init().await;

        let res = user_service
            .update(UserID::new_random(), valid_request())
            .await;
        assert!(res.is_err());
        if let Err(err) = res {
            assert!(matches!(err, Error::NotFound(Resource::User)))
        }
    }
}

mod error {
    use super::*;
    use tokio::test;

    #[test]
    async fn repo_backend_error() {
        let user_service = UserService::init(MockUserRepository::new_backend_error());

        let res = user_service
            .update(UserID::new_random(), valid_request())
            .await;
        assert!(res.is_err());
        if let Err(err) = res {
            assert!(matches!(err, Error::Internal(_)));
        }
    }

    #[test]
    async fn repo_programming_error() {
        let user_service = UserService::init(MockUserRepository::new_programming_error());

        let res = user_service
            .update(UserID::new_random(), valid_request())
            .await;
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
        let (user_id, user_service) = init().await;

        for (name, input, expected) in NORMALIZATION_TEST_INPUT {
            let update_request = UpdateRequest {
                display_name: Some(input.to_string()),
                ..Default::default()
            };
            let user = user_service.update(user_id, update_request).await.unwrap();
            assert_eq!(
                user.display_name, expected,
                "case: {name} - expected: {expected} (found: {})",
                user.display_name
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

        for (name, input) in SINGLE_LINE_TEST_INPUT {
            let update_request = UpdateRequest {
                display_name: Some(input.to_string()),
                ..Default::default()
            };
            let res = user_service.update(user_id, update_request).await;
            assert!(res.is_err(), "case: {name}; should have failed");
            if let Err(err) = res {
                assert!(
                    matches!(
                        err,
                        Error::Validation(ValidationError::InvalidValue {
                            field: "display_name",
                            reason: INVALID_SINGLE_LINE
                        })
                    ),
                    "case: {name}; got error: {err}",
                )
            }
        }
    }
}
