use tokio::test;

use crate::{
    error::{
        Resource,
        service::{Error, NO_WHITESPACE, TOO_LONG, ValidationError},
    },
    tag::{
        service::{TagService, TagServiceTrait},
        types::{
            TagID,
            route::{CreateRequest, UpdateRequest},
        },
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
    let test_tag = service
        .create(test_user_id, CreateRequest::default())
        .await
        .unwrap();

    let res = service
        .update(test_tag.id, test_user_id, UpdateRequest::default())
        .await;
    assert!(res.is_ok());
}

#[test]
async fn not_owned() {
    let service = TagService::init(MockTagRepository::new());

    let other_user_id = service.repo.add_user().await;
    let other_tag = service
        .create(other_user_id, CreateRequest::default())
        .await
        .unwrap();

    let res = service
        .update(other_tag.id, UserID::new_v4(), UpdateRequest::default())
        .await;
    assert!(res.is_err());
    if let Err(err) = res {
        assert!(matches!(err, Error::NotFound(Resource::Tag)));
    }
}

#[test]
async fn not_exists() {
    let service = TagService::init(MockTagRepository::new());

    let test_user_id = UserID::new_v4();

    let res = service
        .update(TagID::new_v4(), test_user_id, UpdateRequest::default())
        .await;
    assert!(res.is_err());
    if let Err(err) = res {
        assert!(matches!(err, Error::NotFound(Resource::Tag)))
    }
}

#[test]
async fn no_ops() {
    let service = TagService::init(MockTagRepository::new());

    let test_user_id = service.repo.add_user().await;
    let test_tag = service
        .create(test_user_id, CreateRequest::default())
        .await
        .unwrap();

    let update_request = UpdateRequest {
        label: None,
        ..Default::default()
    };
    let res = service
        .update(test_tag.id, test_user_id, update_request)
        .await;
    assert!(res.is_ok());
    if let Ok(tag) = res {
        assert_eq!(tag, test_tag);
    }
}

#[test]
async fn is_idempotent() {
    let service = TagService::init(MockTagRepository::new());

    let test_user_id = service.repo.add_user().await;
    let test_tag = service
        .create(test_user_id, CreateRequest::default())
        .await
        .unwrap();

    let update_request = UpdateRequest::default();
    let update_1 = service
        .update(test_tag.id, test_user_id, update_request.clone())
        .await
        .unwrap();
    let update_2 = service
        .update(test_tag.id, test_user_id, update_request.clone())
        .await
        .unwrap();

    assert_eq!(update_1, update_2);
}

// label tests
#[test]
async fn normalize_label() {
    let service = TagService::init(MockTagRepository::new());

    let test_user_id = service.repo.add_user().await;
    let test_tag = service
        .create(test_user_id, CreateRequest::default())
        .await
        .unwrap();

    for (name, input, expected) in NORMALIZATION_TEST_INPUT {
        let update_request = UpdateRequest {
            label: Some(input.to_string()),
            ..Default::default()
        };
        let tag = service
            .update(test_tag.id, test_user_id, update_request)
            .await
            .unwrap();
        assert_eq!(
            tag.label, expected,
            "case: {} - expected: {}; found {}",
            name, expected, tag.label,
        )
    }
}

#[test]
async fn errors_with_label_containing_whitespaces() {
    let service = TagService::init(MockTagRepository::new());

    let test_user_id = service.repo.add_user().await;
    let test_tag = service
        .create(test_user_id, CreateRequest::default())
        .await
        .unwrap();

    for (name, input) in CONTAINS_WHITESPACE_TEST_INPUT {
        let update_request = UpdateRequest {
            label: Some(input.to_string()),
            ..Default::default()
        };
        let res = service
            .update(test_tag.id, test_user_id, update_request)
            .await;
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

    let test_user_id = service.repo.add_user().await;
    let test_tag = service
        .create(test_user_id, CreateRequest::default())
        .await
        .unwrap();

    let mut too_long_label = String::new();
    for _ in 0..12 {
        too_long_label.push_str("1234567890");
    }
    let update_request = UpdateRequest {
        label: Some(too_long_label),
        ..Default::default()
    };
    let res = service
        .update(test_tag.id, test_user_id, update_request)
        .await;
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
    let test_tag = service
        .create(test_user_id, CreateRequest::default())
        .await
        .unwrap();

    for (name, input, expected) in NORMALIZATION_TEST_INPUT {
        let update_request = UpdateRequest {
            category: Some(Some(input.to_string())),
            ..Default::default()
        };
        let tag = service
            .update(test_tag.id, test_user_id, update_request)
            .await
            .unwrap();
        assert_eq!(
            tag.category_name.unwrap(),
            expected,
            "case: {} - expected: {}; found {}",
            name,
            expected,
            tag.label,
        )
    }
}

#[test]
async fn errors_on_blank_category() {
    let service = TagService::init(MockTagRepository::new());

    let test_user_id = service.repo.add_user().await;
    let test_tag = service
        .create(test_user_id, CreateRequest::default())
        .await
        .unwrap();

    let update_request = UpdateRequest {
        category: Some(Some("".to_string())),
        ..Default::default()
    };
    let res = service
        .update(test_tag.id, test_user_id, update_request)
        .await;
    assert!(res.is_err());
    if let Err(err) = res {
        assert!(matches!(
            err,
            Error::Validation(ValidationError::InvalidValue {
                field: "category",
                reason: _
            })
        ))
    }
}

#[test]
async fn errors_with_category_containing_whitespaces() {
    let service = TagService::init(MockTagRepository::new());

    let test_user_id = service.repo.add_user().await;
    let test_tag = service
        .create(test_user_id, CreateRequest::default())
        .await
        .unwrap();

    for (name, input) in CONTAINS_WHITESPACE_TEST_INPUT {
        let update_request = UpdateRequest {
            category: Some(Some(input.to_string())),
            ..Default::default()
        };
        let res = service
            .update(test_tag.id, test_user_id, update_request)
            .await;
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

    let test_user_id = service.repo.add_user().await;
    let test_tag = service
        .create(test_user_id, CreateRequest::default())
        .await
        .unwrap();

    let mut too_long_label = String::new();
    for _ in 0..12 {
        too_long_label.push_str("1234567890");
    }
    let update_request = UpdateRequest {
        category: Some(Some(too_long_label)),
        ..Default::default()
    };
    let res = service
        .update(test_tag.id, test_user_id, update_request)
        .await;
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
        .update(TagID::new_v4(), UserID::new_v4(), UpdateRequest::default())
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
        .update(TagID::new_v4(), UserID::new_v4(), UpdateRequest::default())
        .await;
    assert!(res.is_err());
    if let Err(err) = res {
        assert!(matches!(err, Error::Unhandled(_)));
    }
}
