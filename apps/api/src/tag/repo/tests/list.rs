use sqlx::{PgPool, test};

use crate::{
    tag::{
        repo::{PgTagRepository, QueryOpts, TagModel, TagRepository},
        types::{
            TagCategoryID,
            repo::{CreateModel, Filter},
        },
    },
    tests::helpers::{
        create_test_user, generate_a_z,
        tag::{
            default_tag, full_tag, seed_tags, seed_tags_with_category, tag_with_priority_category,
            tag_with_workflow_category,
        },
    },
    types::pagination::SQLPagination,
    user::repo::PgUserRepository,
};

struct CategoryCase {
    name: &'static str,
    category_id: TagCategoryID,
    check: fn(&[TagModel]),
}

// Input Tests
#[test]
async fn pagination_limit(pool: PgPool) {
    let user_repo = PgUserRepository::init(pool.clone());
    let repo = PgTagRepository::init(pool.clone());

    let test_user = create_test_user(&user_repo).await;
    seed_tags(&repo, 25, test_user.id, default_tag).await;

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
    if let Ok(tags) = res {
        assert_eq!(tags.len(), 5);
    }
}

#[test]
async fn pagination_offset(pool: PgPool) {
    let user_repo = PgUserRepository::init(pool.clone());
    let repo = PgTagRepository::init(pool.clone());

    let test_user = create_test_user(&user_repo).await;
    seed_tags(&repo, 25, test_user.id, default_tag).await;

    let ref_tags = repo
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
        if let Ok(tags) = res {
            let offset = offset as usize;
            assert_eq!(&tags[0..5], &ref_tags[offset..(5 + offset)])
        }
    }
}

#[test]
async fn filter_category(pool: PgPool) {
    let user_repo = PgUserRepository::init(pool.clone());
    let repo = PgTagRepository::init(pool.clone());

    let test_user = create_test_user(&user_repo).await;
    let workflow_category_id = repo
        .add_category(
            test_user.id,
            "Workflow".to_string(),
            generate_a_z(0).to_string(),
        )
        .await
        .unwrap();
    let priority_category_id = repo
        .add_category(
            test_user.id,
            "Priority".to_string(),
            generate_a_z(1).to_string(),
        )
        .await
        .unwrap();
    seed_tags(&repo, 5, test_user.id, default_tag).await;
    seed_tags_with_category(
        &repo,
        5,
        test_user.id,
        workflow_category_id,
        tag_with_workflow_category,
    )
    .await;
    seed_tags_with_category(
        &repo,
        5,
        test_user.id,
        priority_category_id,
        tag_with_priority_category,
    )
    .await;

    let cases = vec![
        CategoryCase {
            name: "Workflow category",
            category_id: workflow_category_id,
            check: |tags| {
                assert!(
                    tags.iter()
                        .all(|tag| matches!(tag.category_name.as_deref(), Some("Workflow")))
                )
            },
        },
        CategoryCase {
            name: "Priority category",
            category_id: priority_category_id,
            check: |tags| {
                assert!(
                    tags.iter()
                        .all(|tag| matches!(tag.category_name.as_deref(), Some("Priority")))
                )
            },
        },
    ];
    for case in cases {
        let CategoryCase {
            name,
            category_id,
            check,
        } = case;

        let mut filter = Filter::new();
        filter.category(category_id);

        let opts = QueryOpts {
            filter,
            ..Default::default()
        };

        let res = repo.list(test_user.id, Some(opts)).await;
        assert!(res.is_ok(), "request failed for {}", name);
        if let Ok(tags) = res {
            check(&tags);
        }
    }
}

// Output Tests
#[test]
async fn verify_output(pool: PgPool) {
    let user_repo = PgUserRepository::init(pool.clone());
    let repo = PgTagRepository::init(pool.clone());

    let test_user = create_test_user(&user_repo).await;
    let test_category = repo
        .add_category(
            test_user.id,
            "Testing".to_string(),
            generate_a_z(0).to_string(),
        )
        .await
        .unwrap();
    seed_tags_with_category(&repo, 25, test_user.id, test_category, full_tag).await;

    let res = repo.list(test_user.id, None).await;
    assert!(res.is_ok());
    if let Ok(tags) = res {
        for tag in tags {
            assert!(tag.label.starts_with("Full Tag"));
            assert_eq!(tag.category_id, Some(test_category));
            assert_eq!(tag.category_name.as_deref(), Some("Testing"));
        }
    }
}

// Behavior Tests
#[test]
async fn works(pool: PgPool) {
    let user_repo = PgUserRepository::init(pool.clone());
    let repo = PgTagRepository::init(pool.clone());

    let test_user = create_test_user(&user_repo).await;
    seed_tags(&repo, 25, test_user.id, default_tag).await;

    let res = repo.list(test_user.id, None).await;
    assert!(res.is_ok());
}

#[test]
async fn sorts_by_category_pos_then_tag_pos(pool: PgPool) {
    let user_repo = PgUserRepository::init(pool.clone());
    let repo = PgTagRepository::init(pool.clone());

    let test_user = create_test_user(&user_repo).await;
    let test1_category_id = repo
        .add_category(
            test_user.id,
            "Test1".to_string(),
            generate_a_z(0).to_string(),
        )
        .await
        .unwrap();
    let test2_category_id = repo
        .add_category(
            test_user.id,
            "Test2".to_string(),
            generate_a_z(1).to_string(),
        )
        .await
        .unwrap();
    for i in 0..5 {
        repo.create(
            test_user.id,
            CreateModel {
                label: format!("Test Tag {}", i),
                position_key: generate_a_z(i).to_string(),
                ..Default::default()
            },
        )
        .await
        .unwrap();
    }
    for i in 0..5 {
        repo.create(
            test_user.id,
            CreateModel {
                label: format!("Testing {}", i),
                category_id: Some(test1_category_id),
                position_key: generate_a_z(i).to_string(),
            },
        )
        .await
        .unwrap();
    }
    for i in 0..5 {
        repo.create(
            test_user.id,
            CreateModel {
                label: format!("Group {}", i),
                position_key: generate_a_z(i).to_string(),
                category_id: Some(test2_category_id),
            },
        )
        .await
        .unwrap();
    }

    let res = repo.list(test_user.id, None).await;
    assert!(res.is_ok());
    if let Ok(tags) = res {
        assert!(tags.is_sorted_by(|a, b| {
            let a_key = (
                a.category_id
                    .as_ref()
                    .map(|id| if id == &test1_category_id { 0 } else { 1 }),
                &a.position_key,
            );
            let b_key = (
                b.category_id
                    .as_ref()
                    .map(|id| if id == &test1_category_id { 0 } else { 1 }),
                &b.position_key,
            );

            a_key <= b_key
        }))
    }
}
