use tokio::test;

use crate::{
    error::service::{Error, NO_EMPTY_STRING, NO_WHITESPACE, ValidationError},
    tests::{
        mocks::user::MockUserRepository,
        test_data::{CONTAINS_WHITESPACE_TEST_INPUT, NORMALIZATION_TEST_INPUT},
    },
    user::service::{CreateRequest, UserService, UserServiceTrait},
};

#[test]
async fn success() {
    let service = UserService::init(MockUserRepository::new());

    let res = service.create(CreateRequest::default()).await;
    assert!(res.is_ok())
}

// display_name tests
#[test]
async fn normalize_display_name() {
    let service = UserService::init(MockUserRepository::new());

    for (name, input, expected) in NORMALIZATION_TEST_INPUT {
        let create_request = CreateRequest {
            display_name: input.to_string(),
            ..Default::default()
        };
        let user = service.create(create_request.clone()).await.unwrap();
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

    let create_request = CreateRequest {
        display_name: "".to_string(),
        ..Default::default()
    };
    let res = service.create(create_request).await;
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
    let service = UserService::init(MockUserRepository::new());

    for (name, input) in CONTAINS_WHITESPACE_TEST_INPUT {
        let create_request = CreateRequest {
            display_name: input.to_string(),
            ..Default::default()
        };
        let res = service.create(create_request.clone()).await;
        assert!(res.is_err());
        if let Err(err) = res {
            assert!(
                matches!(
                    err,
                    Error::Validation(ValidationError::InvalidValue {
                        field: "display_name",
                        reason: NO_WHITESPACE,
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

    let res = service.create(CreateRequest::default()).await;
    assert!(res.is_err());
    if let Err(err) = res {
        assert!(matches!(err, Error::Internal(_)));
    }
}

#[test]
async fn repo_programming_error() {
    let service = UserService::init(MockUserRepository::new_programming_error());

    let res = service.create(CreateRequest::default()).await;
    assert!(res.is_err());
    if let Err(err) = res {
        assert!(matches!(err, Error::Unhandled(_)));
    }
}
