use chrono_tz::Tz;

use crate::{
    error::{
        Resource,
        service::{Error, INVALID_SINGLE_LINE, TOO_LONG, ValidationError},
    },
    tag::{
        service::TagService,
        types::{
            TagID,
            route::{CreateRequest, UpdateRequest},
        },
    },
    tests::{
        mocks::{MockTagRepository, category::MockCategoryRepository},
        test_data::{NORMALIZATION_TEST_INPUT, SINGLE_LINE_TEST_INPUT},
    },
    types::extract::UserContext,
};

use super::init_test_setup;

async fn init() -> (
    UserContext,
    TagID,
    TagService<MockTagRepository, MockCategoryRepository>,
) {
    let tag_service = init_test_setup();

    let user_context = UserContext {
        id: tag_service.repo.add_user().await,
        tz: Tz::America__Chicago,
    };
    let tag = tag_service
        .create(
            user_context,
            CreateRequest {
                label: "Test Tag".to_string(),
                ..Default::default()
            },
        )
        .await
        .unwrap();

    (user_context, tag.id, tag_service)
}

fn valid_request() -> UpdateRequest {
    UpdateRequest {
        label: Some("Updated Tag".to_string()),
        ..Default::default()
    }
}

mod success {
    use super::*;
    use tokio::test;

    #[test]
    async fn success() {
        let (user_context, tag_id, tag_service) = init().await;

        let res = tag_service
            .update(tag_id, user_context, valid_request())
            .await;
        assert!(res.is_ok());
    }

    #[test]
    async fn no_op() {
        let (user_context, tag_id, tag_service) = init().await;
        let init_tag = tag_service.get(tag_id, user_context).await.unwrap();

        let tag = tag_service
            .update(tag_id, user_context, UpdateRequest::default())
            .await
            .unwrap();
        assert_eq!(tag, init_tag);
    }

    #[test]
    async fn is_idempotent() {
        let (user_context, tag_id, tag_service) = init().await;

        let update_request = UpdateRequest {
            label: Some("Updated Tag".to_string()),
            ..Default::default()
        };
        let first_update = tag_service
            .update(tag_id, user_context, update_request.clone())
            .await
            .unwrap();
        let second_update = tag_service
            .update(tag_id, user_context, update_request.clone())
            .await
            .unwrap();

        assert_eq!(first_update, second_update);
    }
}

mod existence {
    use super::*;
    use tokio::test;

    #[test]
    async fn not_owned() {
        let (_, tag_id, tag_service) = init().await;
        let user_context = UserContext {
            id: tag_service.repo.add_user().await,
            tz: Tz::America__Chicago,
        };

        let res = tag_service
            .update(tag_id, user_context, valid_request())
            .await;
        assert!(res.is_err());
        if let Err(err) = res {
            assert!(matches!(err, Error::NotFound(Resource::Tag)));
        }
    }

    #[test]
    async fn not_exists() {
        let (user_context, _, tag_service) = init().await;

        let res = tag_service
            .update(TagID::new_random(), user_context, valid_request())
            .await;
        assert!(res.is_err());
        if let Err(err) = res {
            assert!(matches!(err, Error::NotFound(Resource::Tag)))
        }
    }
}

mod error {
    use super::*;
    use tokio::test;

    #[test]
    async fn repo_backend_error() {
        let tag_service = TagService::init(
            MockTagRepository::new_backend_error(),
            MockCategoryRepository::new(),
        );
        let user_context = UserContext {
            id: tag_service.repo.add_user().await,
            tz: Tz::America__Chicago,
        };

        let res = tag_service
            .update(TagID::new_random(), user_context, valid_request())
            .await;
        assert!(res.is_err());
        if let Err(err) = res {
            assert!(matches!(err, Error::Internal(_)));
        }
    }

    #[test]
    async fn repo_programming_error() {
        let tag_service = TagService::init(
            MockTagRepository::new_internal_error(),
            MockCategoryRepository::new(),
        );
        let user_context = UserContext {
            id: tag_service.repo.add_user().await,
            tz: Tz::America__Chicago,
        };

        let res = tag_service
            .update(TagID::new_random(), user_context, valid_request())
            .await;
        assert!(res.is_err());
        if let Err(err) = res {
            assert!(matches!(err, Error::Internal(_)));
        }
    }
}

mod label {
    use super::*;
    use tokio::test;

    #[test]
    async fn normalize_label() {
        let (user_context, tag_id, tag_service) = init().await;

        for (name, input, expected) in NORMALIZATION_TEST_INPUT {
            let update_request = UpdateRequest {
                label: Some(input.to_string()),
                ..Default::default()
            };
            let tag = tag_service
                .update(tag_id, user_context, update_request)
                .await
                .unwrap();
            assert_eq!(
                tag.label, expected,
                "case: {name} - expected: {expected}; found {}",
                tag.label,
            )
        }
    }

    #[test]
    async fn errors_with_label_containing_whitespaces() {
        let (user_context, tag_id, tag_service) = init().await;

        for (name, input) in SINGLE_LINE_TEST_INPUT {
            let update_request = UpdateRequest {
                label: Some(input.to_string()),
                ..Default::default()
            };
            let res = tag_service
                .update(tag_id, user_context, update_request)
                .await;
            assert!(res.is_err(), "case: {name}; should have failed");
            if let Err(err) = res {
                assert!(
                    matches!(
                        err,
                        Error::Validation(ValidationError::InvalidValue {
                            field: "label",
                            reason: INVALID_SINGLE_LINE
                        })
                    ),
                    "case: {name}; got error: {err}",
                )
            }
        }
    }

    #[test]
    async fn errors_with_label_more_than_100_chars() {
        let (user_context, tag_id, tag_service) = init().await;

        let mut too_long_label = String::new();
        for _ in 0..12 {
            too_long_label.push_str("1234567890");
        }
        let update_request = UpdateRequest {
            label: Some(too_long_label),
            ..Default::default()
        };
        let res = tag_service
            .update(tag_id, user_context, update_request)
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
                "got error: {err}",
            )
        }
    }
}

mod category {
    use super::*;
    use tokio::test;

    #[test]
    async fn category_not_owned() {
        let (user_context, tag_id, tag_service) = init().await;
        let category = tag_service
            .repo
            .add_category(tag_service.repo.add_user().await)
            .await;

        let update_request = UpdateRequest {
            category: Some(Some(category.name)),
            ..Default::default()
        };
        let res = tag_service
            .update(tag_id, user_context, update_request)
            .await;
        assert!(res.is_err());
        if let Err(err) = res {
            assert!(matches!(err, Error::NotFound(Resource::Category)))
        }
    }

    #[test]
    async fn category_not_exists() {
        let (user_context, tag_id, tag_service) = init().await;

        let update_request = UpdateRequest {
            category: Some(Some("Not Exists".to_string())),
            ..Default::default()
        };
        let res = tag_service
            .update(tag_id, user_context, update_request)
            .await;
        assert!(res.is_err());
        if let Err(err) = res {
            assert!(matches!(err, Error::NotFound(Resource::Category)))
        }
    }
}
