use chrono::{DateTime, NaiveDate, Utc};
use sqlx::{PgPool, test};

use crate::{
    tag::{
        repo::{PgTagRepository, TagRepository},
        types::TagID,
    },
    task::{
        repo::{PgTaskRepository, TaskModel, TaskRepository},
        types::{
            SortBy,
            repo::{CreateModel, Filter, QueryOpts, Sort},
        },
    },
    tests::helpers::{
        create_test_user, generate_a_z, get_today_date_pg, is_task_ordered, nulls_last_key,
        tag::{default_tag, seed_tags},
        task::{
            default_task, seed_tasks, seed_tasks_with_tags, task_with_deadline, task_with_start,
        },
    },
    types::{
        date::{DateBound, DateFilter, StartPrecision},
        order::SortOrder,
        pagination::SQLPagination,
    },
    user::repo::PgUserRepository,
};

struct SortCase {
    name: &'static str,
    sort_by: SortBy,
    sort_order: SortOrder,
    check: fn(&[TaskModel]),
}

struct StartCase {
    name: &'static str,
    date_filter: DateFilter<DateTime<Utc>>,
    check: fn(&[TaskModel]),
}

struct DeadlineCase {
    name: &'static str,
    date_filter: DateFilter<NaiveDate>,
    check: fn(&[TaskModel]),
}

struct BoolCase {
    name: &'static str,
    bool_filter: bool,
    check: fn(&[TaskModel]),
}

// Input Tests
#[test]
async fn pagination_limit(pool: PgPool) {
    let user_repo = PgUserRepository::init(pool.clone());
    let repo = PgTaskRepository::init(pool.clone());

    let test_user = create_test_user(&user_repo).await;
    seed_tasks(&repo, 25, test_user.id, None, default_task).await;

    let mut pagination = SQLPagination::new();
    pagination.limit(5);

    let res = repo
        .list(
            test_user.id,
            Some(QueryOpts {
                pagination,
                ..Default::default()
            }),
        )
        .await;
    assert!(res.is_ok());
    if let Ok(tasks) = res {
        assert_eq!(tasks.len(), 5);
    }
}

#[test]
async fn pagination_offset(pool: PgPool) {
    let user_repo = PgUserRepository::init(pool.clone());
    let repo = PgTaskRepository::init(pool.clone());

    let test_user = create_test_user(&user_repo).await;
    seed_tasks(&repo, 25, test_user.id, None, default_task).await;

    let ref_tasks = repo
        .list(test_user.id, Some(QueryOpts::default()))
        .await
        .unwrap();

    for offset in 1..=10 {
        let mut pagination = SQLPagination::new();
        pagination.offset(offset);

        let res = repo
            .list(
                test_user.id,
                Some(QueryOpts {
                    pagination,
                    ..Default::default()
                }),
            )
            .await;
        assert!(res.is_ok());
        if let Ok(tasks) = res {
            let offset = offset as usize;
            assert_eq!(&tasks[0..5], &ref_tasks[offset..(5 + offset)])
        }
    }
}

#[test]
async fn sort_variants(pool: PgPool) {
    let cases = vec![
        SortCase {
            name: "id asc",
            sort_by: SortBy::ID,
            sort_order: SortOrder::Ascending,
            check: |tasks| {
                assert!(
                    is_task_ordered(tasks, |task| task.id, SortOrder::Ascending),
                    "id asc"
                )
            },
        },
        SortCase {
            name: "id desc",
            sort_by: SortBy::ID,
            sort_order: SortOrder::Descending,
            check: |tasks| {
                assert!(
                    is_task_ordered(tasks, |task| task.id, SortOrder::Descending),
                    "id desc"
                )
            },
        },
        SortCase {
            name: "created asc",
            sort_by: SortBy::Created,
            sort_order: SortOrder::Ascending,
            check: |tasks| {
                assert!(
                    is_task_ordered(tasks, |task| task.created_at, SortOrder::Ascending),
                    "created asc"
                )
            },
        },
        SortCase {
            name: "created desc",
            sort_by: SortBy::Created,
            sort_order: SortOrder::Descending,
            check: |tasks| {
                assert!(
                    is_task_ordered(tasks, |task| task.created_at, SortOrder::Descending),
                    "created desc"
                )
            },
        },
        SortCase {
            name: "updated asc",
            sort_by: SortBy::Updated,
            sort_order: SortOrder::Ascending,
            check: |tasks| {
                assert!(
                    is_task_ordered(tasks, |task| task.updated_at, SortOrder::Ascending),
                    "updated asc"
                )
            },
        },
        SortCase {
            name: "updated desc",
            sort_by: SortBy::Updated,
            sort_order: SortOrder::Descending,
            check: |tasks| {
                assert!(
                    is_task_ordered(tasks, |task| task.updated_at, SortOrder::Descending),
                    "updated desc"
                )
            },
        },
        SortCase {
            name: "start asc",
            sort_by: SortBy::Start,
            sort_order: SortOrder::Ascending,
            check: |tasks| {
                assert!(
                    is_task_ordered(
                        tasks,
                        |task| nulls_last_key(task.start_dt, SortOrder::Ascending),
                        SortOrder::Ascending
                    ),
                    "start asc"
                )
            },
        },
        SortCase {
            name: "start desc",
            sort_by: SortBy::Start,
            sort_order: SortOrder::Descending,
            check: |tasks| {
                assert!(
                    is_task_ordered(
                        tasks,
                        |task| nulls_last_key(task.start_dt, SortOrder::Descending),
                        SortOrder::Descending
                    ),
                    "start desc"
                )
            },
        },
        SortCase {
            name: "deadline asc",
            sort_by: SortBy::Deadline,
            sort_order: SortOrder::Ascending,
            check: |tasks| {
                assert!(
                    is_task_ordered(
                        tasks,
                        |task| nulls_last_key(task.deadline, SortOrder::Ascending),
                        SortOrder::Ascending
                    ),
                    "deadline asc"
                )
            },
        },
        SortCase {
            name: "deadline desc",
            sort_by: SortBy::Deadline,
            sort_order: SortOrder::Descending,
            check: |tasks| {
                assert!(
                    is_task_ordered(
                        tasks,
                        |task| nulls_last_key(task.deadline, SortOrder::Descending),
                        SortOrder::Descending
                    ),
                    "deadline desc"
                )
            },
        },
        SortCase {
            name: "position asc",
            sort_by: SortBy::Position,
            sort_order: SortOrder::Ascending,
            check: |tasks| {
                assert!(
                    is_task_ordered(
                        tasks,
                        |task| task.position_key.clone(),
                        SortOrder::Ascending,
                    ),
                    "position asc"
                )
            },
        },
        SortCase {
            name: "position desc",
            sort_by: SortBy::Position,
            sort_order: SortOrder::Descending,
            check: |tasks| {
                assert!(
                    is_task_ordered(
                        tasks,
                        |task| task.position_key.clone(),
                        SortOrder::Descending,
                    ),
                    "position desc"
                )
            },
        },
    ];

    let user_repo = PgUserRepository::init(pool.clone());
    let repo = PgTaskRepository::init(pool.clone());

    let test_user = create_test_user(&user_repo).await;
    seed_tasks(&repo, 10, test_user.id, None, default_task).await;
    seed_tasks(&repo, 10, test_user.id, None, task_with_start).await;
    seed_tasks(&repo, 10, test_user.id, None, task_with_deadline).await;

    for case in cases {
        let SortCase {
            name,
            sort_by,
            sort_order,
            check,
        } = case;

        let opts = QueryOpts {
            sort: Sort::new(Some(sort_by), sort_order),
            ..Default::default()
        };

        let res = repo.list(test_user.id, Some(opts)).await;
        assert!(res.is_ok(), "request failed for {}", name);
        if let Ok(tasks) = res {
            check(&tasks);
        }
    }
}

#[test]
async fn filter_start(pool: PgPool) {
    let date_bound = DateTime::parse_from_rfc3339("2026-01-15T12:00:00Z")
        .unwrap()
        .to_utc();

    let cases = vec![
        StartCase {
            name: "start greater than",
            date_filter: DateFilter::StartRange(DateBound::Exclusive(date_bound)),
            check: |tasks| {
                assert!(tasks.iter().all(|task| {
                    task.start_dt.unwrap()
                        > DateTime::parse_from_rfc3339("2026-01-15T12:00:00Z")
                            .unwrap()
                            .to_utc()
                }))
            },
        },
        StartCase {
            name: "start greater than equal",
            date_filter: DateFilter::StartRange(DateBound::Inclusive(date_bound)),
            check: |tasks| {
                assert!(tasks.iter().all(|task| {
                    task.start_dt.unwrap()
                        >= DateTime::parse_from_rfc3339("2026-01-15T12:00:00Z")
                            .unwrap()
                            .to_utc()
                }))
            },
        },
        StartCase {
            name: "start less than",
            date_filter: DateFilter::EndRange(DateBound::Exclusive(date_bound)),
            check: |tasks| {
                assert!(tasks.iter().all(|task| {
                    task.start_dt.unwrap()
                        < DateTime::parse_from_rfc3339("2026-01-15T12:00:00Z")
                            .unwrap()
                            .to_utc()
                }))
            },
        },
        StartCase {
            name: "start less than equal",
            date_filter: DateFilter::EndRange(DateBound::Inclusive(date_bound)),
            check: |tasks| {
                assert!(tasks.iter().all(|task| {
                    task.start_dt.unwrap()
                        <= DateTime::parse_from_rfc3339("2026-01-15T12:00:00Z")
                            .unwrap()
                            .to_utc()
                }))
            },
        },
    ];

    let user_repo = PgUserRepository::init(pool.clone());
    let repo = PgTaskRepository::init(pool.clone());

    let test_user = create_test_user(&user_repo).await;
    seed_tasks(&repo, 31, test_user.id, None, task_with_start).await;

    for case in cases {
        let StartCase {
            name,
            date_filter,
            check,
        } = case;

        let mut filter = Filter::new();
        filter.start(date_filter);

        let opts = QueryOpts {
            filter,
            ..Default::default()
        };

        let res = repo.list(test_user.id, Some(opts)).await;
        assert!(res.is_ok(), "request failed for {}", name);
        if let Ok(tasks) = res {
            check(&tasks);
        }
    }
}

#[test]
async fn filter_deadline(pool: PgPool) {
    let date_bound = NaiveDate::from_ymd_opt(2026, 1, 15).unwrap();

    let cases =
        vec![
            DeadlineCase {
                name: "deadline greater than",
                date_filter: DateFilter::StartRange(DateBound::Exclusive(date_bound)),
                check: |tasks| {
                    assert!(tasks.iter().all(|task| task.deadline.unwrap()
                        > NaiveDate::from_ymd_opt(2026, 1, 15).unwrap()))
                },
            },
            DeadlineCase {
                name: "deadline greater than equal",
                date_filter: DateFilter::StartRange(DateBound::Inclusive(date_bound)),
                check: |tasks| {
                    assert!(tasks.iter().all(|task| task.deadline.unwrap()
                        >= NaiveDate::from_ymd_opt(2026, 1, 15).unwrap()))
                },
            },
            DeadlineCase {
                name: "deadline less than",
                date_filter: DateFilter::EndRange(DateBound::Exclusive(date_bound)),
                check: |tasks| {
                    assert!(tasks.iter().all(|task| task.deadline.unwrap()
                        < NaiveDate::from_ymd_opt(2026, 1, 15).unwrap()))
                },
            },
            DeadlineCase {
                name: "deadline less than equal",
                date_filter: DateFilter::EndRange(DateBound::Inclusive(date_bound)),
                check: |tasks| {
                    assert!(tasks.iter().all(|task| task.deadline.unwrap()
                        <= NaiveDate::from_ymd_opt(2026, 1, 15).unwrap()))
                },
            },
        ];

    let user_repo = PgUserRepository::init(pool.clone());
    let repo = PgTaskRepository::init(pool.clone());

    let test_user = create_test_user(&user_repo).await;
    seed_tasks(&repo, 31, test_user.id, None, task_with_deadline).await;

    for case in cases {
        let DeadlineCase {
            name,
            date_filter,
            check,
        } = case;

        let mut filter = Filter::new();
        filter.deadline(date_filter);

        let opts = QueryOpts {
            filter,
            ..Default::default()
        };

        let res = repo.list(test_user.id, Some(opts)).await;
        assert!(res.is_ok(), "request failed for {}", name);
        if let Ok(tasks) = res {
            check(&tasks);
        }
    }
}

#[test]
async fn filter_bool(pool: PgPool) {
    let cases = vec![
        BoolCase {
            name: "completed false",
            bool_filter: false,
            check: |tasks| assert!(tasks.iter().all(|task| task.completed_at.is_none())),
        },
        BoolCase {
            name: "completed true",
            bool_filter: true,
            check: |tasks| assert!(tasks.iter().all(|task| task.completed_at.is_some())),
        },
        BoolCase {
            name: "deleted false",
            bool_filter: false,
            check: |tasks| assert!(tasks.iter().all(|task| task.deleted_at.is_none())),
        },
        BoolCase {
            name: "deleted true",
            bool_filter: true,
            check: |tasks| assert!(tasks.iter().all(|task| task.deleted_at.is_some())),
        },
    ];

    let user_repo = PgUserRepository::init(pool.clone());
    let repo = PgTaskRepository::init(pool.clone());

    let test_user = create_test_user(&user_repo).await;
    for completed in [false, true] {
        for deleted in [false, true] {
            seed_tasks(
                &repo,
                5,
                test_user.id,
                Some((completed, deleted)),
                default_task,
            )
            .await;
        }
    }

    for case in cases {
        let BoolCase {
            name,
            bool_filter,
            check,
        } = case;

        let mut filter = Filter::new();
        let opts = if name.contains("completed") {
            filter.completed(bool_filter);

            QueryOpts {
                filter,
                ..Default::default()
            }
        } else if name.contains("deleted") {
            filter.deleted(bool_filter);

            QueryOpts {
                filter,
                ..Default::default()
            }
        } else {
            unreachable!()
        };

        let res = repo.list(test_user.id, Some(opts)).await;
        assert!(res.is_ok(), "request failed for {}", name);
        if let Ok(tasks) = res {
            check(&tasks);
        }
    }
}

#[test]
async fn filter_tags(pool: PgPool) {
    let user_repo = PgUserRepository::init(pool.clone());
    let tag_repo = PgTagRepository::init(pool.clone());
    let repo = PgTaskRepository::init(pool.clone());

    let test_user = create_test_user(&user_repo).await;
    seed_tags(&tag_repo, 2, test_user.id, default_tag).await;
    let tags = tag_repo.list(test_user.id, None).await.unwrap();
    let tag_ids: Vec<TagID> = tags.iter().map(|tag| tag.id).collect();
    seed_tasks_with_tags(
        &repo,
        10,
        test_user.id,
        None,
        tag_ids[..1].to_vec(),
        default_task,
    )
    .await;
    seed_tasks_with_tags(
        &repo,
        10,
        test_user.id,
        None,
        tag_ids.to_vec(),
        default_task,
    )
    .await;

    let mut filter = Filter::new();
    filter.tags(tag_ids.to_vec());

    let res = repo
        .list(
            test_user.id,
            Some(QueryOpts {
                filter,
                ..Default::default()
            }),
        )
        .await;
    assert!(res.is_ok());
    if let Ok(task) = res {
        assert_eq!(task.len(), 10);
    }
}

// Output Tests
#[test]
async fn verify_output(pool: PgPool) {
    let user_repo = PgUserRepository::init(pool.clone());
    let repo = PgTaskRepository::init(pool.clone());

    let test_user = create_test_user(&user_repo).await;
    repo.create(
        test_user.id,
        CreateModel {
            title: "Test Task".to_string(),
            notes: Some("Notes for 'Test Task'".to_string()),
            start: Some(get_today_date_pg()),
            start_precision: StartPrecision::DateTime,
            deadline: Some(get_today_date_pg().date_naive()),
            position_key: generate_a_z(0).to_string(),
        },
    )
    .await
    .unwrap();

    let res = repo.list(test_user.id, None).await;
    assert!(res.is_ok());
    if let Ok(tasks) = res {
        let task = tasks[0].clone();

        assert!(!task.title.is_empty());
        assert!(task.notes.is_some());
        assert!(task.start_dt.is_some());
        assert!(task.has_time);
        assert!(task.deadline.is_some());
        assert!(task.completed_at.is_none());
        assert!(task.deleted_at.is_none());
    }
}

// Behavior Tests
#[test]
async fn works(pool: PgPool) {
    let user_repo = PgUserRepository::init(pool.clone());
    let repo = PgTaskRepository::init(pool.clone());

    let test_user = create_test_user(&user_repo).await;
    seed_tasks(&repo, 25, test_user.id, None, default_task).await;

    let res = repo.list(test_user.id, None).await;
    assert!(res.is_ok());
}
