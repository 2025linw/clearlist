use tokio::test;

use crate::{
    error::service::{Error, NO_EMPTY_STRING, NO_WHITESPACE, ValidationError},
    tag::{
        repo::TagRepository,
        service::{TagService, TagServiceTrait},
        types::{route::URLQueryOpts, service::UserContext},
    },
    tests::{
        helpers::generate_a_z, mocks::tag::MockTagRepository, test_data::NORMALIZATION_TEST_INPUT,
    },
    user::types::UserID,
};

#[test]
async fn success() {
    let service = TagService::init(MockTagRepository::new());

    let res = service
        .list(
            UserContext {
                id: UserID::new_v4(),
            },
            None,
        )
        .await;
    assert!(res.is_ok());
}

// pagination tests
#[test]
async fn pagination_mapping() {
    let service = TagService::init(MockTagRepository::new());

    let query = URLQueryOpts {
        page: Some(5),
        limit: Some(25),
        ..Default::default()
    };
    let res = service
        .list(
            UserContext {
                id: UserID::new_v4(),
            },
            Some(query),
        )
        .await;
    assert!(res.is_ok());

    let last_filter_res = service.repo.get_last_filter().await;
    assert!(last_filter_res.is_some());
    if let Some(query) = last_filter_res {
        assert_eq!(query.pagination.limit, Some(25));
        assert_eq!(query.pagination.offset, Some(100));
    }
}

#[test]
async fn default_pagination() {
    let service = TagService::init(MockTagRepository::new());

    let res = service
        .list(
            UserContext {
                id: UserID::new_v4(),
            },
            None,
        )
        .await;
    assert!(res.is_ok());

    let last_filter_res = service.repo.get_last_filter().await;
    assert!(last_filter_res.is_some());
    if let Some(query) = last_filter_res {
        assert_eq!(query.pagination.limit, Some(25));
        assert_eq!(query.pagination.offset, Some(0));
    }
}

#[test]
async fn limit_caps_at_150() {
    let service = TagService::init(MockTagRepository::new());

    let query = URLQueryOpts {
        limit: Some(200),
        ..Default::default()
    };
    let res = service
        .list(
            UserContext {
                id: UserID::new_v4(),
            },
            Some(query),
        )
        .await;
    assert!(res.is_ok());

    let last_filter_res = service.repo.get_last_filter().await;
    assert!(last_filter_res.is_some());
    if let Some(query) = last_filter_res {
        assert_eq!(query.pagination.limit, Some(150));
        assert_eq!(query.pagination.offset, Some(0));
    }
}

#[test]
async fn errors_with_0_limit() {
    let service = TagService::init(MockTagRepository::new());

    let query = URLQueryOpts {
        limit: Some(0),
        ..Default::default()
    };
    let res = service
        .list(
            UserContext {
                id: UserID::new_v4(),
            },
            Some(query),
        )
        .await;
    assert!(res.is_err());
    if let Err(err) = res {
        assert!(matches!(
            err,
            Error::Validation(ValidationError::InvalidValue {
                field: "limit",
                reason: _
            })
        ))
    }
}

#[test]
async fn errors_with_0_page() {
    let service = TagService::init(MockTagRepository::new());

    let query = URLQueryOpts {
        page: Some(0),
        ..Default::default()
    };
    let res = service
        .list(
            UserContext {
                id: UserID::new_v4(),
            },
            Some(query),
        )
        .await;
    assert!(res.is_err());
    if let Err(err) = res {
        assert!(matches!(
            err,
            Error::Validation(ValidationError::InvalidValue {
                field: "page",
                reason: _
            })
        ))
    }
}

// filter tests
#[test]
async fn nonexistent_category_returns_empty() {
    let service = TagService::init(MockTagRepository::new());

    let query = URLQueryOpts {
        category: Some("Nonexistent Category".to_string()),
        ..Default::default()
    };
    let tags = service
        .list(
            UserContext {
                id: UserID::new_v4(),
            },
            Some(query),
        )
        .await
        .unwrap();
    assert_eq!(tags.len(), 0);
}

#[test]
async fn normalize_category() {
    let service = TagService::init(MockTagRepository::new());

    let test_user_id = service.repo.add_user().await;
    let test_category_id = service
        .repo
        .add_category(
            test_user_id,
            "Test Text".to_string(),
            generate_a_z(0).to_string(),
        )
        .await
        .unwrap();

    for (name, input, _) in NORMALIZATION_TEST_INPUT {
        let query = URLQueryOpts {
            category: Some(input.to_string()),
            ..Default::default()
        };
        service
            .list(UserContext { id: test_user_id }, Some(query))
            .await
            .unwrap();

        let filter_category_id = service
            .repo
            .get_last_filter()
            .await
            .unwrap()
            .filter
            .category
            .unwrap();
        assert_eq!(
            filter_category_id, test_category_id,
            "case: {} - expected: {}; found: {}",
            name, test_category_id, filter_category_id
        )
    }
}

#[test]
async fn errors_with_blank_category() {
    let service = TagService::init(MockTagRepository::new());

    let query = URLQueryOpts {
        category: Some("".to_string()),
        ..Default::default()
    };
    let res = service
        .list(
            UserContext {
                id: UserID::new_v4(),
            },
            Some(query),
        )
        .await;
    assert!(res.is_err());
    if let Err(err) = res {
        assert!(matches!(
            err,
            Error::Validation(ValidationError::InvalidValue {
                field: "category",
                reason: NO_EMPTY_STRING
            })
        ))
    }
}

#[test]
async fn errors_category_containing_whitespace() {
    let service = TagService::init(MockTagRepository::new());

    let query = URLQueryOpts {
        category: Some("Test\n\tCategory".to_string()),
        ..Default::default()
    };
    let res = service
        .list(
            UserContext {
                id: UserID::new_v4(),
            },
            Some(query),
        )
        .await;
    assert!(res.is_err());
    if let Err(err) = res {
        assert!(matches!(
            err,
            Error::Validation(ValidationError::InvalidValue {
                field: "category",
                reason: NO_WHITESPACE
            })
        ))
    }
}

// repo errors
#[test]
async fn repo_backend_error() {
    let service = TagService::init(MockTagRepository::new_backend_error());

    let res = service
        .list(
            UserContext {
                id: UserID::new_v4(),
            },
            None,
        )
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
        .list(
            UserContext {
                id: UserID::new_v4(),
            },
            None,
        )
        .await;
    assert!(res.is_err());
    if let Err(err) = res {
        assert!(matches!(err, Error::Unhandled(_)));
    }
}
