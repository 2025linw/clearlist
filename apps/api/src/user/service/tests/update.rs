use tokio::test;

use crate::{
    error::{
        Resource,
        service::{Error, NO_EMPTY_STRING, NO_WHITESPACE, ValidationError},
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

#[test]
async fn success() {
    let service = UserService::init(MockUserRepository::new());

    let test_user = service.create(CreateRequest::default()).await.unwrap();

    let res = service.update(test_user.id, UpdateRequest::default()).await;
    assert!(res.is_ok());
}

#[test]
async fn not_exists() {
    let service = UserService::init(MockUserRepository::new());

    let res = service
        .update(UserID::new_v4(), UpdateRequest::default())
        .await;
    assert!(res.is_err());
    if let Err(err) = res {
        assert!(matches!(err, Error::NotFound(Resource::User)))
    }
}

#[test]
async fn no_ops() {
    let service = UserService::init(MockUserRepository::new());

    let test_user = service.create(CreateRequest::default()).await.unwrap();

    let update_request = UpdateRequest {
        display_name: None,
        ..Default::default()
    };
    let res = service.update(test_user.id, update_request).await;
    assert!(res.is_ok());
    if let Ok(user) = res {
        assert_eq!(user, test_user);
    }
}

#[test]
async fn is_idempotent() {
    let service = UserService::init(MockUserRepository::new());

    let test_user = service.create(CreateRequest::default()).await.unwrap();

    let update_request = UpdateRequest::default();
    let update_1 = service
        .update(test_user.id, update_request.clone())
        .await
        .unwrap();
    let update_2 = service
        .update(test_user.id, update_request.clone())
        .await
        .unwrap();

    assert_eq!(update_1, update_2);
}

// display_name tests
#[test]
async fn normalize_display_name() {
    let service = UserService::init(MockUserRepository::new());

    let test_user = service.create(CreateRequest::default()).await.unwrap();

    for (name, input, expected) in NORMALIZATION_TEST_INPUT {
        let update_request = UpdateRequest {
            display_name: Some(input.to_string()),
            preferred_timezone: None,
            completed_task_retention: None,
        };
        let user = service.update(test_user.id, update_request).await.unwrap();
        assert_eq!(
            user.display_name, expected,
            "case: {} - expected: {} (found: {})",
            name, expected, user.display_name
        );
    }
}

#[test]
async fn errors_on_blank_display_name() {
    let service = UserService::init(MockUserRepository::new());

    let test_user = service.create(CreateRequest::default()).await.unwrap();

    let update_request = UpdateRequest {
        display_name: Some("".to_string()),
        ..Default::default()
    };
    let res = service.update(test_user.id, update_request).await;
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
    let service = UserService::init(MockUserRepository::new());

    let test_user = service.create(CreateRequest::default()).await.unwrap();

    for (name, input) in CONTAINS_WHITESPACE_TEST_INPUT {
        let update_request = UpdateRequest {
            display_name: Some(input.to_string()),
            ..Default::default()
        };
        let res = service.update(test_user.id, update_request).await;
        assert!(res.is_err());
        if let Err(err) = res {
            assert!(
                matches!(
                    err,
                    Error::Validation(ValidationError::InvalidValue {
                        field: "display_name",
                        reason: NO_WHITESPACE
                    })
                ),
                "case: {}; got error: {}",
                name,
                err
            )
        }
    }
}

// repo errors
#[test]
async fn repo_backend_error() {
    let service = UserService::init(MockUserRepository::new_backend_error());

    let res = service
        .update(UserID::new_v4(), UpdateRequest::default())
        .await;
    assert!(res.is_err());
    if let Err(err) = res {
        assert!(matches!(err, Error::Internal(_)));
    }
}

#[test]
async fn repo_programming_error() {
    let service = UserService::init(MockUserRepository::new_programming_error());

    let res = service
        .update(UserID::new_v4(), UpdateRequest::default())
        .await;
    assert!(res.is_err());
    if let Err(err) = res {
        assert!(matches!(err, Error::Unhandled(_)));
    }
}
