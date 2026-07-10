use sqlx::{PgPool, test};

use crate::{
    tag::{
        repo::{
            Model, PgTagRepository, QueryOpts, TagRepository,
            types::{Filter, Pagination, Sort},
        },
        types::SortBy,
    },
    tests::helpers::{
        create_test_user, is_tag_ordered,
        tag::{default_tag, seed_tags, tag_with_workflow_category, tag_with_workflow_priority},
    },
    types::order::SortOrder,
    user::repo::PgUserRepository,
};

struct SortCase {
    name: &'static str,
    sort_by: SortBy,
    sort_order: SortOrder,
    check: fn(&[Model]),
}

struct CategoryCase {
    name: &'static str,
    filter: String,
    check: fn(&[Model]),
}

#[test]
async fn list_tags(pool: PgPool) {
    let user_repo = PgUserRepository::init(pool.clone());
    let repo = PgTagRepository::init(pool.clone());

    let user = create_test_user(&user_repo).await;
    seed_tags(&repo, 25, user.id, default_tag).await;

    let res = repo.list(user.id, None).await;
    assert!(res.is_ok());
}

#[test]
async fn list_returns_correct_format(pool: PgPool) {
    let user_repo = PgUserRepository::init(pool.clone());
    let repo = PgTagRepository::init(pool.clone());

    let user = create_test_user(&user_repo).await;
    seed_tags(&repo, 25, user.id, default_tag).await;

    let res = repo.list(user.id, None).await;
    assert!(res.is_ok());
    if let Ok(tags) = res {
        for tag in tags {
            assert_eq!(tag.label, "");
            assert!(tag.category.is_none());
        }
    }
}

#[test]
async fn list_limit(pool: PgPool) {
    let user_repo = PgUserRepository::init(pool.clone());
    let repo = PgTagRepository::init(pool.clone());

    let user = create_test_user(&user_repo).await;
    seed_tags(&repo, 25, user.id, default_tag).await;

    let res = repo
        .list(
            user.id,
            Some(QueryOpts {
                pagination: Pagination::new(Some(5), None),
                ..Default::default()
            }),
        )
        .await;
    assert!(res.is_ok());
    if let Ok(tags) = res {
        assert_eq!(tags.len(), 5);
    }
}

#[test]
async fn list_offset(pool: PgPool) {
    let user_repo = PgUserRepository::init(pool.clone());
    let repo = PgTagRepository::init(pool.clone());

    let user = create_test_user(&user_repo).await;
    seed_tags(&repo, 25, user.id, default_tag).await;

    let ref_tags = repo
        .list(user.id, Some(QueryOpts::default()))
        .await
        .unwrap();

    for offset in 1..=10 {
        let res = repo
            .list(
                user.id,
                Some(QueryOpts {
                    pagination: Pagination::new(None, Some(offset)),
                    ..Default::default()
                }),
            )
            .await;
        assert!(res.is_ok());
        if let Ok(tags) = res {
            assert_eq!(&tags[0..5], &ref_tags[offset..(5 + offset)])
        }
    }
}

#[test]
async fn list_sorts(pool: PgPool) {
    let cases = vec![
        SortCase {
            name: "id asc",
            sort_by: SortBy::ID,
            sort_order: SortOrder::Ascending,
            check: |tags| {
                assert!(
                    is_tag_ordered(tags, |tag| tag.id, SortOrder::Ascending),
                    "id asc"
                )
            },
        },
        SortCase {
            name: "id desc",
            sort_by: SortBy::ID,
            sort_order: SortOrder::Descending,
            check: |tags| {
                assert!(
                    is_tag_ordered(tags, |tag| tag.id, SortOrder::Descending),
                    "id desc"
                )
            },
        },
        SortCase {
            name: "created asc",
            sort_by: SortBy::Created,
            sort_order: SortOrder::Ascending,
            check: |tags| {
                assert!(
                    is_tag_ordered(tags, |tag| tag.created_at, SortOrder::Ascending),
                    "created asc"
                )
            },
        },
        SortCase {
            name: "created desc",
            sort_by: SortBy::Created,
            sort_order: SortOrder::Descending,
            check: |tags| {
                assert!(
                    is_tag_ordered(tags, |tag| tag.created_at, SortOrder::Descending),
                    "created desc"
                )
            },
        },
        SortCase {
            name: "updated asc",
            sort_by: SortBy::Updated,
            sort_order: SortOrder::Ascending,
            check: |tags| {
                assert!(
                    is_tag_ordered(tags, |tag| tag.updated_at, SortOrder::Ascending),
                    "updated asc"
                )
            },
        },
        SortCase {
            name: "updated desc",
            sort_by: SortBy::Updated,
            sort_order: SortOrder::Descending,
            check: |tags| {
                assert!(
                    is_tag_ordered(tags, |tag| tag.updated_at, SortOrder::Descending),
                    "updated desc"
                )
            },
        },
        SortCase {
            name: "position asc",
            sort_by: SortBy::Position,
            sort_order: SortOrder::Ascending,
            check: |tags| {
                assert!(
                    is_tag_ordered(tags, |tag| tag.position_key.clone(), SortOrder::Ascending),
                    "position asc"
                )
            },
        },
        SortCase {
            name: "position desc",
            sort_by: SortBy::Position,
            sort_order: SortOrder::Descending,
            check: |tags| {
                assert!(
                    is_tag_ordered(tags, |tag| tag.position_key.clone(), SortOrder::Descending),
                    "position desc"
                )
            },
        },
    ];

    let user_repo = PgUserRepository::init(pool.clone());
    let repo = PgTagRepository::init(pool.clone());

    let user = create_test_user(&user_repo).await;
    seed_tags(&repo, 10, user.id, default_tag).await;

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

        let res = repo.list(user.id, Some(opts)).await;
        assert!(res.is_ok(), "request failed for {}", name);
        if let Ok(tags) = res {
            check(&tags);
        }
    }
}

#[test]
async fn list_filter_category(pool: PgPool) {
    let cases = vec![
        CategoryCase {
            name: "Workflow category",
            filter: "Workflow".to_string(),
            check: |tags| {
                assert!(
                    tags.iter()
                        .all(|tag| matches!(tag.category.as_deref(), Some("Workflow")))
                )
            },
        },
        CategoryCase {
            name: "Priority category",
            filter: "Priority".to_string(),
            check: |tags| {
                assert!(
                    tags.iter()
                        .all(|tag| matches!(tag.category.as_deref(), Some("Priority")))
                )
            },
        },
    ];

    let user_repo = PgUserRepository::init(pool.clone());
    let repo = PgTagRepository::init(pool.clone());

    let user = create_test_user(&user_repo).await;
    seed_tags(&repo, 5, user.id, default_tag).await;
    seed_tags(&repo, 5, user.id, tag_with_workflow_category).await;
    seed_tags(&repo, 5, user.id, tag_with_workflow_priority).await;

    for case in cases {
        let CategoryCase {
            name,
            filter,
            check,
        } = case;

        let opts = QueryOpts {
            filter: Filter::new().category(filter),
            ..Default::default()
        };

        let res = repo.list(user.id, Some(opts)).await;
        assert!(res.is_ok(), "request failed for {}", name);
        if let Ok(tags) = res {
            check(&tags);
        }
    }
}
