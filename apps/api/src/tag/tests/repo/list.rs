use sqlx::PgPool;

use crate::{
    category::{
        repo::PgCategoryRepository,
        types::{CategoryID, CategoryModel},
    },
    tag::{
        repo::{PgTagRepository, TagRepository},
        types::{
            TagModel,
            repo::{CreateModel, Filter, QueryOpts},
        },
    },
    tests::helpers::{
        create_test_category, create_test_user, generate_a_z,
        tag::{default_tag, seed_tags, seed_tags_with_category, tag_with_category},
    },
    types::repo::Pagination,
    user::{repo::PgUserRepository, types::UserID},
};

async fn init(pool: PgPool) -> (UserID, CategoryModel, PgTagRepository) {
    let user_repo = PgUserRepository::init(pool.clone());
    let category_repo = PgCategoryRepository::init(pool.clone());
    let tag_repo = PgTagRepository::init(pool.clone());

    let user = create_test_user(&user_repo).await;
    let category = create_test_category(&category_repo, user.id).await;

    (user.id, category, tag_repo)
}

mod success {
    use super::*;
    use sqlx::test;

    #[test]
    async fn success(pool: PgPool) {
        let (user_id, _, tag_repo) = init(pool).await;

        seed_tags(&tag_repo, 25, user_id, default_tag).await;

        let res = tag_repo.list(user_id, None).await;
        assert!(res.is_ok());
    }

    #[test]
    async fn verify_output(pool: PgPool) {
        let (user_id, category, tag_repo) = init(pool).await;

        tag_repo
            .create(
                user_id,
                CreateModel {
                    label: "Test Tag".to_string(),
                    category_id: Some(category.id),
                    position_key: generate_a_z(0).to_string(),
                },
            )
            .await
            .unwrap();

        let tag = tag_repo.list(user_id, None).await.unwrap().remove(0);
        assert!(tag.label.starts_with("Test Tag"));
        assert_eq!(tag.category_id.unwrap(), category.id);
        assert_eq!(tag.category_name.unwrap(), "Test Category");
        assert_eq!(tag.position_key, generate_a_z(0).to_string());
    }
}

mod filter {
    use super::*;
    use sqlx::test;

    struct CategoryCase {
        name: &'static str,
        category_id: CategoryID,
        check: fn(&[TagModel]) -> bool,
    }

    #[test]
    async fn filter_category(pool: PgPool) {
        let (user_id, category, tag_repo) = init(pool).await;

        seed_tags(&tag_repo, 5, user_id, default_tag).await;
        seed_tags_with_category(&tag_repo, 5, user_id, category.id, tag_with_category).await;

        let cases = vec![CategoryCase {
            name: "With category",
            category_id: category.id,
            check: |tags| {
                tags.iter()
                    .all(|tag| matches!(tag.category_name.as_deref(), Some("Test Category")))
            },
        }];

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
            assert!(check(&tags), "failed for {name}");
        }
    }
}

mod sort {
    use super::*;
    use sqlx::test;

    #[test]
    async fn sorts_by_category_pos_then_tag_pos(pool: PgPool) {
        // Default tag sort is category position first then tag position
        let (user_id, category, tag_repo) = init(pool).await;

        for i in 0..5 {
            tag_repo
                .create(
                    user_id,
                    CreateModel {
                        label: format!("Uncategorized {i}"),
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
                        label: format!("Categorized {i}"),
                        category_id: Some(category.id),
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
                    .map(|id| if id == &category.id { 0 } else { 1 }),
                &a.position_key,
            );
            let b_key = (
                b.category_id
                    .as_ref()
                    .map(|id| if id == &category.id { 0 } else { 1 }),
                &b.position_key,
            );

            a_key <= b_key
        }))
    }
}

mod pagination {
    use super::*;
    use sqlx::test;

    #[test]
    async fn pagination_limit(pool: PgPool) {
        let (user_id, _, tag_repo) = init(pool).await;

        seed_tags(&tag_repo, 25, user_id, default_tag).await;

        let mut pagination = Pagination::new();
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
        let (user_id, _, tag_repo) = init(pool).await;

        seed_tags(&tag_repo, 25, user_id, default_tag).await;

        let ref_tags = tag_repo
            .list(user_id, Some(QueryOpts::default()))
            .await
            .unwrap();

        for offset in 1..=10 {
            let mut pagination = Pagination::new();
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
