use sqlx::PgPool;

use crate::{
    tag::{
        repo::{PgTagRepository, QueryOpts, TagModel, TagRepository},
        types::{
            CategoryID,
            repo::{CreateModel, Filter},
        },
    },
    tests::helpers::{
        create_test_user, generate_a_z,
        tag::{
            default_tag, seed_tags, seed_tags_with_category, tag_with_priority_category,
            tag_with_workflow_category,
        },
    },
    types::pagination::SQLPagination,
    user::{repo::PgUserRepository, types::UserID},
};

async fn init(pool: PgPool) -> (UserID, PgTagRepository) {
    let user_repo = PgUserRepository::init(pool.clone());
    let tag_repo = PgTagRepository::init(pool.clone());

    let user = create_test_user(&user_repo).await;

    (user.id, tag_repo)
}

mod success {
    use sqlx::test;

    use super::*;

    #[test]
    async fn success(pool: PgPool) {
        let (user_id, tag_repo) = init(pool).await;

        seed_tags(&tag_repo, 25, user_id, default_tag).await;

        let res = tag_repo.list(user_id, None).await;
        assert!(res.is_ok());
    }

    #[test]
    async fn verify_output(pool: PgPool) {
        let (user_id, tag_repo) = init(pool).await;

        let category_id = tag_repo
            .add_category(user_id, "Testing".to_string(), generate_a_z(0).to_string())
            .await
            .unwrap();
        tag_repo
            .create(
                user_id,
                CreateModel {
                    label: "Test Tag".to_string(),
                    category_id: Some(category_id),
                    position_key: generate_a_z(0).to_string(),
                },
            )
            .await
            .unwrap();

        let tag = tag_repo.list(user_id, None).await.unwrap().remove(0);
        assert!(tag.label.starts_with("Test Tag"));
        assert_eq!(tag.category_id.unwrap(), category_id);
        assert_eq!(tag.category_name.unwrap(), "Testing");
        assert_eq!(tag.position_key, generate_a_z(0).to_string());
    }
}

mod filter {
    use sqlx::test;

    use super::*;

    struct CategoryCase {
        name: &'static str,
        category_id: CategoryID,
        check: fn(&[TagModel]) -> bool,
    }

    #[test]
    async fn filter_category(pool: PgPool) {
        let (user_id, tag_repo) = init(pool).await;

        let workflow_category_id = tag_repo
            .add_category(user_id, "Workflow".to_string(), generate_a_z(0).to_string())
            .await
            .unwrap();
        let priority_category_id = tag_repo
            .add_category(user_id, "Priority".to_string(), generate_a_z(1).to_string())
            .await
            .unwrap();

        seed_tags(&tag_repo, 5, user_id, default_tag).await;
        seed_tags_with_category(
            &tag_repo,
            5,
            user_id,
            workflow_category_id,
            tag_with_workflow_category,
        )
        .await;
        seed_tags_with_category(
            &tag_repo,
            5,
            user_id,
            priority_category_id,
            tag_with_priority_category,
        )
        .await;

        let cases = vec![
            CategoryCase {
                name: "Workflow category",
                category_id: workflow_category_id,
                check: |tags| {
                    tags.iter()
                        .all(|tag| matches!(tag.category_name.as_deref(), Some("Workflow")))
                },
            },
            CategoryCase {
                name: "Priority category",
                category_id: priority_category_id,
                check: |tags| {
                    tags.iter()
                        .all(|tag| matches!(tag.category_name.as_deref(), Some("Priority")))
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

            let tags = tag_repo.list(user_id, Some(opts)).await.unwrap();
            assert!(check(&tags), "failed for {}", name);
        }
    }
}

mod sort {
    use sqlx::test;

    use super::*;

    #[test]
    async fn sorts_by_category_pos_then_tag_pos(pool: PgPool) {
        // Default tag sort is category position first then tag position
        let (user_id, tag_repo) = init(pool).await;

        let test1_category_id = tag_repo
            .add_category(user_id, "Test1".to_string(), generate_a_z(0).to_string())
            .await
            .unwrap();
        let test2_category_id = tag_repo
            .add_category(user_id, "Test2".to_string(), generate_a_z(1).to_string())
            .await
            .unwrap();

        for i in 0..5 {
            tag_repo
                .create(
                    user_id,
                    CreateModel {
                        label: format!("Uncategorized {}", i),
                        position_key: generate_a_z(i).to_string(),
                        ..Default::default()
                    },
                )
                .await
                .unwrap();
        }
        for i in 0..5 {
            tag_repo
                .create(
                    user_id,
                    CreateModel {
                        label: format!("Test1 Category {}", i),
                        category_id: Some(test1_category_id),
                        position_key: generate_a_z(i).to_string(),
                    },
                )
                .await
                .unwrap();
        }
        for i in 0..5 {
            tag_repo
                .create(
                    user_id,
                    CreateModel {
                        label: format!("Test2 Category {}", i),
                        category_id: Some(test2_category_id),
                        position_key: generate_a_z(i).to_string(),
                    },
                )
                .await
                .unwrap();
        }

        let tags = tag_repo.list(user_id, None).await.unwrap();
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

mod pagination {
    use sqlx::test;

    use super::*;

    #[test]
    async fn pagination_limit(pool: PgPool) {
        let (user_id, tag_repo) = init(pool).await;

        seed_tags(&tag_repo, 25, user_id, default_tag).await;

        let mut pagination = SQLPagination::new();
        pagination.limit(5);

        let tags = tag_repo
            .list(
                user_id,
                Some(QueryOpts {
                    pagination,
                    ..Default::default()
                }),
            )
            .await
            .unwrap();
        assert_eq!(tags.len(), 5);
    }

    #[test]
    async fn pagination_offset(pool: PgPool) {
        let (user_id, tag_repo) = init(pool).await;

        seed_tags(&tag_repo, 25, user_id, default_tag).await;

        let ref_tags = tag_repo
            .list(user_id, Some(QueryOpts::default()))
            .await
            .unwrap();

        for offset in 1..=10 {
            let mut pagination = SQLPagination::new();
            pagination.offset(offset);

            let tags = tag_repo
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
            assert_eq!(&tags[0..5], &ref_tags[offset..(5 + offset)])
        }
    }
}
