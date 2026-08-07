use crate::{
    error::{
        Resource,
        service::{Error, NO_NONSPACE_WHITESPACE, ValidationError},
    },
    tag::types::TagID,
    task::{
        service::{TaskService, TaskServiceTrait},
        types::{route::CreateRequest, service::UserContext},
    },
    tests::{
        helpers::get_today_date_pg,
        mocks::task::MockTaskRepository,
        test_data::{
            CONTAINS_MULTILINE_TEST_INPUT, CONTAINS_WHITESPACE_TEST_INPUT, NORMALIZATION_TEST_INPUT,
        },
    },
    types::date::Start,
    user::types::UserID,
};

async fn init() -> (UserContext, TaskService<MockTaskRepository>) {
    let task_service = TaskService::init(MockTaskRepository::new());

    let user_context = UserContext {
        id: task_service.repo.add_user().await,
        tz: task_service.repo.tz(),
    };

    (user_context, task_service)
}

fn valid_request() -> CreateRequest {
    CreateRequest {
        title: "Test Task".to_string(),
        ..Default::default()
    }
}

mod success {
    use tokio::test;

    use super::*;

    #[test]
    async fn success() {
        let (user_context, task_service) = init().await;

        let res = task_service.create(user_context, valid_request()).await;
        assert!(res.is_ok())
    }
}

mod error {
    use tokio::test;

    use super::*;

    #[test]
    async fn user_not_exists() {
        let (_, task_service) = init().await;
        let user_context = UserContext {
            id: UserID::new_v4(),
            tz: task_service.repo.tz(),
        };

        let res = task_service.create(user_context, valid_request()).await;
        assert!(res.is_err());
        if let Err(err) = res {
            assert!(matches!(err, Error::Internal(_)));
        }
    }

    #[test]
    async fn repo_backend_error() {
        let task_service = TaskService::init(MockTaskRepository::new_backend_error());
        let user_context = UserContext {
            id: UserID::new_v4(),
            tz: task_service.repo.tz(),
        };

        let res = task_service.create(user_context, valid_request()).await;
        assert!(res.is_err());
        if let Err(err) = res {
            assert!(matches!(err, Error::Internal(_)));
        }
    }

    #[test]
    async fn repo_programming_error() {
        let task_service = TaskService::init(MockTaskRepository::new_programming_error());
        let user_context = UserContext {
            id: UserID::new_v4(),
            tz: task_service.repo.tz(),
        };

        let res = task_service.create(user_context, valid_request()).await;
        assert!(res.is_err());
        if let Err(err) = res {
            assert!(matches!(err, Error::Unhandled(_)));
        }
    }
}

mod title {
    use tokio::test;

    use super::*;

    #[test]
    async fn normalize_title() {
        let (user_context, task_service) = init().await;

        for (name, input, expected) in NORMALIZATION_TEST_INPUT {
            let create_request = CreateRequest {
                title: input.to_string(),
                ..Default::default()
            };
            let task = task_service
                .create(user_context, create_request)
                .await
                .unwrap();
            assert_eq!(
                task.title, expected,
                "case: {} - expected: {} (found: {})",
                name, expected, task.title
            );
        }
    }

    #[test]
    async fn errors_with_title_containing_whitespaces() {
        let (user_context, task_service) = init().await;

        for (name, input) in CONTAINS_WHITESPACE_TEST_INPUT {
            let create_request = CreateRequest {
                title: input.to_string(),
                ..Default::default()
            };
            let res = task_service.create(user_context, create_request).await;
            assert!(res.is_err(), "case: {}; should have failed", name);
            if let Err(err) = res {
                assert!(
                    matches!(
                        err,
                        Error::Validation(ValidationError::InvalidValue {
                            field: "title",
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

mod notes {
    use tokio::test;

    use crate::error::service::NO_NONMULTILINE_WHITESPACE;

    use super::*;

    #[test]
    async fn normalize_notes() {
        let (user_context, task_service) = init().await;

        for (name, input, expected) in NORMALIZATION_TEST_INPUT {
            let create_request = CreateRequest {
                notes: Some(input.to_string()),
                ..Default::default()
            };
            let task = task_service
                .create(user_context, create_request)
                .await
                .unwrap();
            let notes = task.notes.unwrap();
            assert_eq!(
                notes, expected,
                "case: {} - expected: {} (found: {})",
                name, expected, notes
            );
        }
    }

    #[test]
    async fn errors_on_notes_containing_invalid_whitespace() {
        let (user_context, task_service) = init().await;

        for (name, input) in CONTAINS_MULTILINE_TEST_INPUT {
            let create_request = CreateRequest {
                notes: Some(input.to_string()),
                ..Default::default()
            };
            let res = task_service.create(user_context, create_request).await;
            assert!(res.is_err(), "case: {}; should have failed", name);
            if let Err(err) = res {
                assert!(
                    matches!(
                        err,
                        Error::Validation(ValidationError::InvalidValue {
                            field: "notes",
                            reason: NO_NONMULTILINE_WHITESPACE
                        })
                    ),
                    "case: {}; got error: {}",
                    name,
                    err
                );
            }
        }
    }
}

mod start {
    use tokio::test;

    use super::*;

    #[test]
    async fn normalize_date_only_start() {
        let (user_context, task_service) = init().await;

        let date = get_today_date_pg().date_naive();
        let res = task_service
            .create(
                user_context,
                CreateRequest {
                    start: Some(Start::Date(date)),
                    ..Default::default()
                },
            )
            .await;
        assert!(res.is_ok());
        if let Ok(task) = res {
            assert_eq!(task.start, Some(Start::Date(date)));
        }
    }

    #[test]
    async fn preserves_datetime_start() {
        let (user_context, task_service) = init().await;

        let utc_dt = get_today_date_pg();
        let res = task_service
            .create(
                user_context,
                CreateRequest {
                    start: Some(Start::DateTime(utc_dt)),
                    ..Default::default()
                },
            )
            .await;
        assert!(res.is_ok());
        if let Ok(task) = res {
            assert_eq!(task.start, Some(Start::DateTime(utc_dt)))
        }
    }
}

mod tags {
    use tokio::test;

    use super::*;

    #[test]
    async fn deduplicate_tags() {
        let (user_context, task_service) = init().await;

        let tag_id = task_service.repo.add_tag(user_context.id).await;
        let res = task_service
            .create(
                user_context,
                CreateRequest {
                    title: "Test Task".to_string(),
                    tags: vec![tag_id, tag_id, tag_id],
                    ..Default::default()
                },
            )
            .await;
        assert!(res.is_ok());

        let tags = task_service.repo.get_last_multi_tag().await;
        assert_eq!(tags.len(), 1);
    }

    #[test]
    async fn tag_not_owned() {
        let (user_context, task_service) = init().await;
        let tag_id = task_service
            .repo
            .add_tag(task_service.repo.add_user().await)
            .await;

        let res = task_service
            .create(
                user_context,
                CreateRequest {
                    title: "Test Task".to_string(),
                    tags: vec![tag_id],
                    ..Default::default()
                },
            )
            .await;
        assert!(res.is_err());
        if let Err(err) = res {
            assert!(matches!(err, Error::NotFound(Resource::Tag)))
        }
    }

    #[test]
    async fn tag_not_exists() {
        let (user_context, task_service) = init().await;

        let create_request = CreateRequest {
            tags: vec![TagID::new_v4()],
            ..Default::default()
        };
        let res = task_service.create(user_context, create_request).await;
        assert!(res.is_err());
        if let Err(err) = res {
            assert!(matches!(err, Error::NotFound(Resource::Tag)))
        }
    }
}
