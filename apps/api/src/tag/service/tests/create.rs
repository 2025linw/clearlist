use tokio::test;

use crate::{
    error::service::{Error, NO_EMPTY_STRING, NO_WHITESPACE, TOO_LONG, ValidationError},
    tag::{
        service::{TagService, TagServiceTrait},
        types::route::CreateRequest,
    },
    tests::{
        mocks::tag::MockTagRepository,
        test_data::{CONTAINS_WHITESPACE_TEST_INPUT, NORMALIZATION_TEST_INPUT},
    },
    user::types::UserID,
};

#[test]
async fn success() {
    let service = TagService::init(MockTagRepository::new());

    let test_user_id = service.repo.add_user().await;

    let res = service.create(test_user_id, CreateRequest::default()).await;
    assert!(res.is_ok())
}

#[test]
async fn user_not_exists() {
    let service = TagService::init(MockTagRepository::new());

    let res = service
        .create(UserID::new_v4(), CreateRequest::default())
        .await;
    assert!(res.is_err());
    if let Err(err) = res {
        assert!(matches!(err, Error::Internal(_)));
    }
}

// label tests
#[test]
async fn normalize_label() {
    let service = TagService::init(MockTagRepository::new());

    let test_user_id = service.repo.add_user().await;

    for (name, input, expected) in NORMALIZATION_TEST_INPUT {
        let create_request = CreateRequest {
            label: input.to_string(),
            ..Default::default()
        };
        let tag = service
            .create(test_user_id, create_request.clone())
            .await
            .unwrap();
        assert_eq!(
            tag.label, expected,
            "case: {} - expected: {}; found: {}",
            name, expected, tag.label
        );
    }
}

#[test]
async fn errors_with_label_containing_whitespaces() {
    let service = TagService::init(MockTagRepository::new());

    for (name, input) in CONTAINS_WHITESPACE_TEST_INPUT {
        let create_request = CreateRequest {
            label: input.to_string(),
            ..Default::default()
        };
        let res = service.create(UserID::new_v4(), create_request).await;
        assert!(res.is_err());
        if let Err(err) = res {
            assert!(
                matches!(
                    err,
                    Error::Validation(ValidationError::InvalidValue {
                        field: "label",
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

#[test]
async fn errors_with_label_more_than_100_chars() {
    let service = TagService::init(MockTagRepository::new());

    let mut too_long_label = String::new();
    for _ in 0..12 {
        too_long_label.push_str("1234567890");
    }
    let create_request = CreateRequest {
        label: too_long_label,
        ..Default::default()
    };
    let res = service.create(UserID::new_v4(), create_request).await;
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

// category tests
#[test]
async fn normalize_category() {
    let service = TagService::init(MockTagRepository::new());

    let test_user_id = service.repo.add_user().await;

    for (name, input, expected) in NORMALIZATION_TEST_INPUT {
        let create_request = CreateRequest {
            category: Some(input.to_string()),
            ..Default::default()
        };
        let tag = service
            .create(test_user_id, create_request.clone())
            .await
            .unwrap();
        let category = tag.category_name.unwrap();
        assert_eq!(
            category, expected,
            "case: {} - expected: {}; found: {}",
            name, expected, category
        );
    }
}

#[test]
async fn errors_on_blank_category() {
    let service = TagService::init(MockTagRepository::new());

    let create_request = CreateRequest {
        category: Some("".to_string()),
        ..Default::default()
    };
    let res = service.create(UserID::new_v4(), create_request).await;
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
    let service = TagService::init(MockTagRepository::new());

    for (name, input) in CONTAINS_WHITESPACE_TEST_INPUT {
        let create_request = CreateRequest {
            category: Some(input.to_string()),
            ..Default::default()
        };
        let res = service.create(UserID::new_v4(), create_request).await;
        assert!(res.is_err());
        if let Err(err) = res {
            assert!(
                matches!(
                    err,
                    Error::Validation(ValidationError::InvalidValue {
                        field: "category",
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

#[test]
async fn errors_with_category_more_than_100_chars() {
    let service = TagService::init(MockTagRepository::new());

    let mut too_long_category = String::new();
    for _ in 0..12 {
        too_long_category.push_str("1234567890");
    }
    let create_request = CreateRequest {
        category: Some(too_long_category),
        ..Default::default()
    };
    let res = service.create(UserID::new_v4(), create_request).await;
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

// repo errors
#[test]
async fn repo_backend_error() {
    let service = TagService::init(MockTagRepository::new_backend_error());

    let res = service
        .create(UserID::new_v4(), CreateRequest::default())
        .await;
    assert!(res.is_err());
    if let Err(err) = res {
        assert!(matches!(err, Error::Internal(_)));
    }
}

#[test]
async fn repo_programming_error() {
    let service = TagService::init(MockTagRepository::new_programming_error());

    let res = service
        .create(UserID::new_v4(), CreateRequest::default())
        .await;
    assert!(res.is_err());
    if let Err(err) = res {
        assert!(matches!(err, Error::Unhandled(_)));
    }
}
