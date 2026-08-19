use crate::{
    error::service::{Error, NO_ZERO_LIMIT, NO_ZERO_PAGE, RANGE_OVERSPECIFIED, ValidationError},
    tag::types::TagID,
    task::{
        service::TaskService,
        types::{
            SortBy,
            repo::{Filter, QueryOpts, Sort},
            route::URLQueryOpts,
        },
    },
    tests::{
        mocks::MockTaskRepository,
        test_data::{create_overspecified_deadline_range, create_overspecified_start_range},
    },
    types::{extract::UserContext, order::SortOrder, query::DateFilter},
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

mod success {
    use super::*;
    use tokio::test;

    #[test]
    async fn success() {
        let (user_context, task_service) = init().await;

        let res = task_service.list(user_context, None).await;
        assert!(res.is_ok());
    }
}

mod error {
    use super::*;
    use tokio::test;

    #[test]
    async fn repo_backend_error() {
        let task_service = TaskService::init(MockTaskRepository::new_backend_error());
        let user_context = UserContext {
            id: UserID::new_random(),
            tz: task_service.repo.tz(),
        };

        let res = task_service.list(user_context, None).await;
        assert!(res.is_err());
        if let Err(err) = res {
            assert!(matches!(err, Error::Internal(_)));
        }
    }

    #[test]
    async fn repo_programming_error() {
        let task_service = TaskService::init(MockTaskRepository::new_programming_error());
        let user_context = UserContext {
            id: UserID::new_random(),
            tz: task_service.repo.tz(),
        };

        let res = task_service.list(user_context, None).await;
        assert!(res.is_err());
        if let Err(err) = res {
            assert!(matches!(err, Error::Internal(_)));
        }
    }
}

mod pagination {
    use super::*;
    use tokio::test;

    #[test]
    async fn pagination_mapping() {
        let (user_context, task_service) = init().await;

        let query = URLQueryOpts {
            page: Some(5),
            limit: Some(25),
            ..Default::default()
        };
        let res = task_service.list(user_context, Some(query)).await;
        assert!(res.is_ok());

        let last_query = task_service.repo.get_last_filter().await;
        let QueryOpts {
            filter: _,
            sort: _,
            pagination,
        } = last_query;

        assert_eq!(pagination.limit, Some(25));
        assert_eq!(pagination.offset, Some(100));
    }

    #[test]
    async fn default_pagination() {
        let (user_context, task_service) = init().await;

        let res = task_service.list(user_context, None).await;
        assert!(res.is_ok());

        let last_query = task_service.repo.get_last_filter().await;

        let QueryOpts {
            filter: _,
            sort: _,
            pagination,
        } = last_query;
        assert_eq!(pagination.limit, Some(25));
        assert_eq!(pagination.offset, Some(0));
    }

    #[test]
    async fn limit_caps_at_150() {
        let (user_context, task_service) = init().await;

        let query = URLQueryOpts {
            limit: Some(200),
            ..Default::default()
        };
        let res = task_service.list(user_context, Some(query)).await;
        assert!(res.is_ok());

        let last_query = task_service.repo.get_last_filter().await;

        let QueryOpts {
            filter: _,
            sort: _,
            pagination,
        } = last_query;
        assert_eq!(pagination.limit, Some(150));
        assert_eq!(pagination.offset, Some(0));
    }

    #[test]
    async fn errors_with_0_limit() {
        let (user_context, task_service) = init().await;

        let query = URLQueryOpts {
            limit: Some(0),
            ..Default::default()
        };
        let res = task_service.list(user_context, Some(query)).await;
        assert!(res.is_err());
        if let Err(err) = res {
            assert!(matches!(
                err,
                Error::Validation(ValidationError::InvalidValue {
                    field: "limit",
                    reason: NO_ZERO_LIMIT
                })
            ))
        }
    }

    #[test]
    async fn errors_with_0_page() {
        let (user_context, task_service) = init().await;

        let query = URLQueryOpts {
            page: Some(0),
            ..Default::default()
        };
        let res = task_service.list(user_context, Some(query)).await;
        assert!(res.is_err());
        if let Err(err) = res {
            assert!(matches!(
                err,
                Error::Validation(ValidationError::InvalidValue {
                    field: "page",
                    reason: NO_ZERO_PAGE
                })
            ))
        }
    }
}

mod sort {
    use super::*;
    use tokio::test;

    #[test]
    async fn default_sort() {
        let (user_context, task_service) = init().await;

        let res = task_service.list(user_context, None).await;
        assert!(res.is_ok());

        let last_query = task_service.repo.get_last_filter().await;

        let QueryOpts {
            filter: _,
            sort: Sort { sort },
            pagination: _,
        } = last_query;
        assert_eq!(sort, Some((SortBy::Updated, SortOrder::Descending)));
    }

    #[test]
    async fn sort_matrix() {
        let (user_context, task_service) = init().await;

        let test_cases = [
            SortBy::ID,
            SortBy::Created,
            SortBy::Updated,
            SortBy::Start,
            SortBy::Deadline,
            SortBy::Position,
        ];
        for sort_by in test_cases {
            for sort_order in [SortOrder::Ascending, SortOrder::Descending] {
                let query = URLQueryOpts {
                    sort_by: Some(sort_by.clone()),
                    sort_order: Some(sort_order.clone()),
                    ..Default::default()
                };
                task_service.list(user_context, Some(query)).await.unwrap();

                let last_query = task_service.repo.get_last_filter().await;

                let QueryOpts {
                    filter: _,
                    sort: Sort { sort },
                    pagination: _,
                } = last_query;
                if let Some((by, order)) = sort {
                    assert_eq!(by, sort_by);
                    assert_eq!(order, sort_order);
                }
            }
        }
    }
}

mod filter {
    use super::*;
    use tokio::test;

    #[test]
    async fn default_filters() {
        let (user_context, task_service) = init().await;

        let res = task_service.list(user_context, None).await;
        assert!(res.is_ok());

        let last_query = task_service.repo.get_last_filter().await;

        let QueryOpts {
            filter: Filter {
                completed, deleted, ..
            },
            sort: _,
            pagination: _,
        } = last_query;
        assert_eq!(completed, Some(false));
        assert_eq!(deleted, Some(false));
    }

    #[test]
    async fn sets_completed_correctly() {
        let (user_context, task_service) = init().await;

        for input in [false, true] {
            let query = URLQueryOpts {
                completed: Some(input),
                ..Default::default()
            };
            let res = task_service.list(user_context, Some(query)).await;
            assert!(res.is_ok());

            let last_query = task_service.repo.get_last_filter().await;

            let QueryOpts {
                filter:
                    Filter {
                        completed,
                        deleted: _,
                        ..
                    },
                sort: _,
                pagination: _,
            } = last_query;
            assert_eq!(completed, Some(input));
        }
    }

    #[test]
    async fn sets_deleted_correctly() {
        let (user_context, task_service) = init().await;

        for input in [false, true] {
            let query = URLQueryOpts {
                deleted: Some(input),
                ..Default::default()
            };
            let res = task_service.list(user_context, Some(query)).await;
            assert!(res.is_ok());

            let last_query = task_service.repo.get_last_filter().await;

            let QueryOpts {
                filter:
                    Filter {
                        completed: _,
                        deleted,
                        ..
                    },
                sort: _,
                pagination: _,
            } = last_query;
            assert_eq!(deleted, Some(input));
        }
    }

    #[test]
    async fn deduplicate_tags() {
        let (user_context, task_service) = init().await;

        let tag_id = TagID::new_random();
        let query = URLQueryOpts {
            tags: Some(vec![tag_id, tag_id, tag_id]),
            ..Default::default()
        };
        let res = task_service.list(user_context, Some(query)).await;
        assert!(res.is_ok());

        let last_query = task_service.repo.get_last_filter().await;

        let QueryOpts {
            filter: Filter { tags, .. },
            ..
        } = last_query;
        assert!(tags.is_some());
        if let Some(tags) = tags {
            assert_eq!(tags.len(), 1);
            assert_eq!(tags[0], tag_id);
        }
    }

    #[test]
    async fn errors_with_overspecified_start_range() {
        let (user_context, task_service) = init().await;

        let test_cases = create_overspecified_start_range();
        for input in test_cases {
            let query = URLQueryOpts {
                start: Some(DateFilter::BracketInterval(input)),
                ..Default::default()
            };

            let res = task_service.list(user_context, Some(query)).await;
            assert!(res.is_err());
            if let Err(err) = res {
                assert!(matches!(
                    err,
                    Error::Validation(ValidationError::InvalidValue {
                        field: "start",
                        reason: RANGE_OVERSPECIFIED,
                    })
                ))
            }
        }
    }

    #[test]
    async fn errors_with_overspecified_deadline_range() {
        let (user_context, task_service) = init().await;

        let test_cases = create_overspecified_deadline_range();
        for input in test_cases {
            let query = URLQueryOpts {
                deadline: Some(DateFilter::BracketInterval(input)),
                ..Default::default()
            };

            let res = task_service.list(user_context, Some(query)).await;
            assert!(res.is_err());
            if let Err(err) = res {
                assert!(matches!(
                    err,
                    Error::Validation(ValidationError::InvalidValue {
                        field: "deadline",
                        reason: RANGE_OVERSPECIFIED,
                    })
                ))
            }
        }
    }
}
