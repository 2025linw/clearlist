use tokio::test;

use crate::{
    error::{
        Resource,
        service::{Error, ValidationError},
    },
    tests::mocks::user::MockUserRepository,
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

#[test]
async fn no_ops() {
    let service = UserService::init(MockUserRepository::new());

    let test_user = service.create(CreateRequest::default()).await.unwrap();

    let update_request = UpdateRequest {
        display_name: None,
        ..Default::default()
    };
    let res = service.update(test_user.id, update_request).await;
    assert!(res.is_err());
    if let Err(err) = res {
        assert!(matches!(err, Error::Validation(ValidationError::NoChanges)));
    }
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

    let test_user = service.create(CreateRequest::default()).await.unwrap();

    for (name, input, expected) in test_cases {
        let update_request = UpdateRequest {
            display_name: Some(input),
            preferred_timezone: None,
            completed_task_retention: None,
        };
        let user = service.update(test_user.id, update_request).await.unwrap();
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
        ("combindation", "Test\t\nUser".to_string()),
    ];

    let service = UserService::init(MockUserRepository::new());

    let test_user = service.create(CreateRequest::default()).await.unwrap();

    for (name, input) in test_inputs {
        let update_request = UpdateRequest {
            display_name: Some(input),
            preferred_timezone: None,
            completed_task_retention: None,
        };
        let res = service.update(test_user.id, update_request).await;
        assert!(res.is_err());
        if let Err(err) = res {
            assert!(
                matches!(
                    err,
                    Error::Validation(ValidationError::InvalidValue {
                        field: "display_name",
                        reason: _
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
