use tokio::test;

use crate::{
    error::service::{Error, ValidationError},
    tests::mocks::user::MockUserRepository,
    user::service::{CreateRequest, UserService, UserServiceTrait},
};

#[test]
async fn success() {
    let service = UserService::init(MockUserRepository::new());

    let res = service.create(CreateRequest::default()).await;
    assert!(res.is_ok())
}

#[test]
async fn normalize_display_name() {
    let test_cases = vec![
        (
            "contains spaces",
            "  Test User     ".to_string(),
            "Test User",
        ),
        (
            "contains newlines",
            "\n\nTest User\n\n".to_string(),
            "Test User",
        ),
        ("contains tabs", "\t\tTest User\t".to_string(), "Test User"),
        (
            "combination",
            "\t   Test User\n   ".to_string(),
            "Test User",
        ),
    ];

    let service = UserService::init(MockUserRepository::new());

    for (name, input, expected) in test_cases {
        let create_request = CreateRequest {
            display_name: input,
            ..Default::default()
        };
        let user = service.create(create_request.clone()).await.unwrap();
        assert_eq!(
            user.display_name, expected,
            "case: {} - expected: {}; found: {}",
            name, expected, user.display_name
        );
    }
}

#[test]
async fn display_name_containing_whitespaces_errors() {
    let test_inputs = vec![
        ("contains newline", "Test\nUser".to_string()),
        ("contains tab", "Test\tUser".to_string()),
        ("combination", "Test\n\tUser".to_string()),
    ];

    let service = UserService::init(MockUserRepository::new());

    for (name, input) in test_inputs {
        let create_request = CreateRequest {
            display_name: input,
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
                        reason: _,
                    })
                ),
                "case: {}",
                name
            )
        }
    }
}

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
