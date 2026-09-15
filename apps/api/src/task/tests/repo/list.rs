use chrono::{DateTime, NaiveDate, Utc};
use sqlx::PgPool;

use crate::{
    tag::{
        repo::{PgTagRepository, TagRepository},
        types::TagID,
    },
    task::{
        repo::{PgTaskRepository, TaskRepository},
        types::{
            SortBy, TaskModel,
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
        field::{DateBound, DateFilter},
        order::SortOrder,
        pagination::SQLPagination,
    },
    user::{repo::PgUserRepository, types::UserID},
};

async fn init(pool: PgPool) -> (UserID, PgTaskRepository) {
    let user_repo = PgUserRepository::init(pool.clone());
    let task_repo = PgTaskRepository::init(pool.clone());

    let user = create_test_user(&user_repo).await;

    (user.id, task_repo)
}

mod success {
    use super::*;
    use sqlx::test;

    #[test]
    async fn success(pool: PgPool) {
        let (user_id, task_repo) = init(pool).await;

        seed_tasks(&task_repo, 25, user_id, None, default_task).await;

        let res = task_repo.list(user_id, None).await;
        assert!(res.is_ok());
    }

    #[test]
    async fn verify_output(pool: PgPool) {
        let (user_id, task_repo) = init(pool).await;

        let today = get_today_date_pg();
        task_repo
            .create(
                user_id,
                CreateModel {
                    title: "Test Task".to_string(),
                    notes: Some("Notes for 'Test Task'".to_string()),
                    start: Some(today),
                    has_time: true,
                    deadline: Some(get_today_date_pg().date_naive()),
                    position_key: generate_a_z(0).to_string(),
                },
            )
            .await
            .unwrap();

        let task = task_repo.list(user_id, None).await.unwrap().remove(0);
        assert_eq!(task.title, "Test Task");
        assert_eq!(task.notes.unwrap(), "Notes for 'Test Task'");
        assert_eq!(task.start_dt.unwrap(), today);
        assert!(task.has_time);
        assert_eq!(task.deadline.unwrap(), today.date_naive());
        assert_eq!(task.position_key, generate_a_z(0).to_string());
    }
}

mod filter {
    use super::*;
    use sqlx::test;

    struct StartCase {
        name: &'static str,
        date_filter: DateFilter<DateTime<Utc>>,
        check: fn(&[TaskModel]) -> bool,
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
                    tasks.iter().all(|task| {
                        task.start_dt.unwrap()
                            > DateTime::parse_from_rfc3339("2026-01-15T12:00:00Z")
                                .unwrap()
                                .to_utc()
                    })
                },
            },
            StartCase {
                name: "start greater than equal",
                date_filter: DateFilter::StartRange(DateBound::Inclusive(date_bound)),
                check: |tasks| {
                    tasks.iter().all(|task| {
                        task.start_dt.unwrap()
                            >= DateTime::parse_from_rfc3339("2026-01-15T12:00:00Z")
                                .unwrap()
                                .to_utc()
                    })
                },
            },
            StartCase {
                name: "start less than",
                date_filter: DateFilter::EndRange(DateBound::Exclusive(date_bound)),
                check: |tasks| {
                    tasks.iter().all(|task| {
                        task.start_dt.unwrap()
                            < DateTime::parse_from_rfc3339("2026-01-15T12:00:00Z")
                                .unwrap()
                                .to_utc()
                    })
                },
            },
            StartCase {
                name: "start less than equal",
                date_filter: DateFilter::EndRange(DateBound::Inclusive(date_bound)),
                check: |tasks| {
                    tasks.iter().all(|task| {
                        task.start_dt.unwrap()
                            <= DateTime::parse_from_rfc3339("2026-01-15T12:00:00Z")
                                .unwrap()
                                .to_utc()
                    })
                },
            },
        ];

        let (user_id, task_repo) = init(pool).await;

        seed_tasks(&task_repo, 31, user_id, None, task_with_start).await;

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

            let tasks = task_repo.list(user_id, Some(opts)).await.unwrap();
            assert!(check(&tasks), "failed for {name}");
        }
    }

    struct DeadlineCase {
        name: &'static str,
        date_filter: DateFilter<NaiveDate>,
        check: fn(&[TaskModel]) -> bool,
    }

    #[test]
    async fn filter_deadline(pool: PgPool) {
        let date_bound = NaiveDate::from_ymd_opt(2026, 1, 15).unwrap();

        let cases = vec![
            DeadlineCase {
                name: "deadline greater than",
                date_filter: DateFilter::StartRange(DateBound::Exclusive(date_bound)),
                check: |tasks| {
                    tasks.iter().all(|task| {
                        task.deadline.unwrap() > NaiveDate::from_ymd_opt(2026, 1, 15).unwrap()
                    })
                },
            },
            DeadlineCase {
                name: "deadline greater than equal",
                date_filter: DateFilter::StartRange(DateBound::Inclusive(date_bound)),
                check: |tasks| {
                    tasks.iter().all(|task| {
                        task.deadline.unwrap() >= NaiveDate::from_ymd_opt(2026, 1, 15).unwrap()
                    })
                },
            },
            DeadlineCase {
                name: "deadline less than",
                date_filter: DateFilter::EndRange(DateBound::Exclusive(date_bound)),
                check: |tasks| {
                    tasks.iter().all(|task| {
                        task.deadline.unwrap() < NaiveDate::from_ymd_opt(2026, 1, 15).unwrap()
                    })
                },
            },
            DeadlineCase {
                name: "deadline less than equal",
                date_filter: DateFilter::EndRange(DateBound::Inclusive(date_bound)),
                check: |tasks| {
                    tasks.iter().all(|task| {
                        task.deadline.unwrap() <= NaiveDate::from_ymd_opt(2026, 1, 15).unwrap()
                    })
                },
            },
        ];

        let (user_id, task_repo) = init(pool).await;

        seed_tasks(&task_repo, 31, user_id, None, task_with_deadline).await;

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

            let tasks = task_repo.list(user_id, Some(opts)).await.unwrap();
            assert!(check(&tasks), "failed for {name}");
        }
    }

    struct BoolCase {
        name: &'static str,
        bool_filter: bool,
        check: fn(&[TaskModel]) -> bool,
    }

    #[test]
    async fn filter_bool(pool: PgPool) {
        let cases = vec![
            BoolCase {
                name: "completed false",
                bool_filter: false,
                check: |tasks| tasks.iter().all(|task| task.completed_at.is_none()),
            },
            BoolCase {
                name: "completed true",
                bool_filter: true,
                check: |tasks| tasks.iter().all(|task| task.completed_at.is_some()),
            },
            BoolCase {
                name: "deleted false",
                bool_filter: false,
                check: |tasks| tasks.iter().all(|task| task.deleted_at.is_none()),
            },
            BoolCase {
                name: "deleted true",
                bool_filter: true,
                check: |tasks| tasks.iter().all(|task| task.deleted_at.is_some()),
            },
        ];

        let (user_id, task_repo) = init(pool).await;

        for completed in [false, true] {
            for deleted in [false, true] {
                seed_tasks(
                    &task_repo,
                    5,
                    user_id,
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

            let tasks = task_repo.list(user_id, Some(opts)).await.unwrap();
            assert!(check(&tasks), "failed for {name}");
        }
    }

    #[test]
    async fn filter_tags(pool: PgPool) {
        let tag_repo = PgTagRepository::init(pool.clone());
        let (user_id, task_repo) = init(pool).await;

        seed_tags(&tag_repo, 2, user_id, default_tag).await;
        let tags = tag_repo.list(user_id, None).await.unwrap();
        let tag_ids: Vec<TagID> = tags.iter().map(|tag| tag.id).collect();

        seed_tasks_with_tags(
            &task_repo,
            10,
            user_id,
            None,
            tag_ids[..1].to_vec(),
            default_task,
        )
        .await;
        seed_tasks_with_tags(
            &task_repo,
            10,
            user_id,
            None,
            tag_ids.to_vec(),
            default_task,
        )
        .await;

        let mut filter = Filter::new();
        filter.tags(tag_ids.to_vec());

        let tasks = task_repo
            .list(
                user_id,
                Some(QueryOpts {
                    filter,
                    ..Default::default()
                }),
            )
            .await
            .unwrap();
        assert_eq!(tasks.len(), 10);
    }
}

mod sort {
    use super::*;
    use sqlx::test;

    struct SortCase {
        name: &'static str,
        sort_by: SortBy,
        sort_order: SortOrder,
        check: fn(&[TaskModel]) -> bool,
    }

    #[test]
    async fn sort_variants(pool: PgPool) {
        let cases = vec![
            SortCase {
                name: "id asc",
                sort_by: SortBy::ID,
                sort_order: SortOrder::Ascending,
                check: |tasks| is_task_ordered(tasks, |task| task.id, SortOrder::Ascending),
            },
            SortCase {
                name: "id desc",
                sort_by: SortBy::ID,
                sort_order: SortOrder::Descending,
                check: |tasks| is_task_ordered(tasks, |task| task.id, SortOrder::Descending),
            },
            SortCase {
                name: "created asc",
                sort_by: SortBy::Created,
                sort_order: SortOrder::Ascending,
                check: |tasks| is_task_ordered(tasks, |task| task.created_at, SortOrder::Ascending),
            },
            SortCase {
                name: "created desc",
                sort_by: SortBy::Created,
                sort_order: SortOrder::Descending,
                check: |tasks| {
                    is_task_ordered(tasks, |task| task.created_at, SortOrder::Descending)
                },
            },
            SortCase {
                name: "updated asc",
                sort_by: SortBy::Updated,
                sort_order: SortOrder::Ascending,
                check: |tasks| is_task_ordered(tasks, |task| task.updated_at, SortOrder::Ascending),
            },
            SortCase {
                name: "updated desc",
                sort_by: SortBy::Updated,
                sort_order: SortOrder::Descending,
                check: |tasks| {
                    is_task_ordered(tasks, |task| task.updated_at, SortOrder::Descending)
                },
            },
            SortCase {
                name: "start asc",
                sort_by: SortBy::Start,
                sort_order: SortOrder::Ascending,
                check: |tasks| {
                    is_task_ordered(
                        tasks,
                        |task| nulls_last_key(task.start_dt, SortOrder::Ascending),
                        SortOrder::Ascending,
                    )
                },
            },
            SortCase {
                name: "start desc",
                sort_by: SortBy::Start,
                sort_order: SortOrder::Descending,
                check: |tasks| {
                    is_task_ordered(
                        tasks,
                        |task| nulls_last_key(task.start_dt, SortOrder::Descending),
                        SortOrder::Descending,
                    )
                },
            },
            SortCase {
                name: "deadline asc",
                sort_by: SortBy::Deadline,
                sort_order: SortOrder::Ascending,
                check: |tasks| {
                    is_task_ordered(
                        tasks,
                        |task| nulls_last_key(task.deadline, SortOrder::Ascending),
                        SortOrder::Ascending,
                    )
                },
            },
            SortCase {
                name: "deadline desc",
                sort_by: SortBy::Deadline,
                sort_order: SortOrder::Descending,
                check: |tasks| {
                    is_task_ordered(
                        tasks,
                        |task| nulls_last_key(task.deadline, SortOrder::Descending),
                        SortOrder::Descending,
                    )
                },
            },
            SortCase {
                name: "position asc",
                sort_by: SortBy::Position,
                sort_order: SortOrder::Ascending,
                check: |tasks| {
                    is_task_ordered(
                        tasks,
                        |task| task.position_key.clone(),
                        SortOrder::Ascending,
                    )
                },
            },
            SortCase {
                name: "position desc",
                sort_by: SortBy::Position,
                sort_order: SortOrder::Descending,
                check: |tasks| {
                    is_task_ordered(
                        tasks,
                        |task| task.position_key.clone(),
                        SortOrder::Descending,
                    )
                },
            },
        ];

        let (user_id, task_repo) = init(pool).await;

        seed_tasks(&task_repo, 10, user_id, None, default_task).await;
        seed_tasks(&task_repo, 10, user_id, None, task_with_start).await;
        seed_tasks(&task_repo, 10, user_id, None, task_with_deadline).await;

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

            let tasks = task_repo.list(user_id, Some(opts)).await.unwrap();
            assert!(check(&tasks), "failed for {name}");
        }
    }
}

mod pagination {
    use super::*;
    use sqlx::test;

    #[test]
    async fn pagination_limit(pool: PgPool) {
        let (user_id, task_repo) = init(pool).await;

        seed_tasks(&task_repo, 25, user_id, None, default_task).await;

        let mut pagination = SQLPagination::new();
        pagination.limit(5);

        let tasks = task_repo
            .list(
                user_id,
                Some(QueryOpts {
                    pagination,
                    ..Default::default()
                }),
            )
            .await
            .unwrap();
        assert_eq!(tasks.len(), 5);
    }

    #[test]
    async fn pagination_offset(pool: PgPool) {
        let (user_id, task_repo) = init(pool).await;

        seed_tasks(&task_repo, 25, user_id, None, default_task).await;

        let ref_tasks = task_repo
            .list(user_id, Some(QueryOpts::default()))
            .await
            .unwrap();

        for offset in 1..=10 {
            let mut pagination = SQLPagination::new();
            pagination.offset(offset);

            let tasks = task_repo
                .list(
                    user_id,
                    Some(QueryOpts {
                        pagination,
                        ..Default::default()
                    }),
                )
                .await
                .unwrap();
            let offset = offset as usize;
            assert_eq!(&tasks[0..5], &ref_tasks[offset..(5 + offset)])
        }
    }
}
