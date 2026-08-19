use sqlx::PgPool;

use crate::{
    tag::repo::{PgTagRepository, TagRepository},
    tests::helpers::{create_test_user, generate_a_z},
    user::{repo::PgUserRepository, types::UserID},
};

async fn init(pool: PgPool) -> (UserID, PgTagRepository) {
    let user_repo = PgUserRepository::init(pool.clone());
    let tag_repo = PgTagRepository::init(pool.clone());

    let user = create_test_user(&user_repo).await;

    (user.id, tag_repo)
}

mod success {
    use super::*;
    use sqlx::test;

    #[test]
    async fn success(pool: PgPool) {
        let (user_id, tag_repo) = init(pool).await;

        tag_repo
            .add_category(user_id, "Testing".to_string(), generate_a_z(0).to_string())
            .await
            .unwrap();

        let res = tag_repo
            .get_category_id(user_id, "Testing".to_string())
            .await;
        assert!(res.is_ok());
    }

    #[test]
    async fn verify_output(pool: PgPool) {
        let (user_id, tag_repo) = init(pool).await;

        let category_id = tag_repo
            .add_category(user_id, "Testing".to_string(), generate_a_z(0).to_string())
            .await
            .unwrap();

        let res_category_id = tag_repo
            .get_category_id(user_id, "Testing".to_string())
            .await
            .unwrap()
            .unwrap();
        assert_eq!(res_category_id, category_id);
    }
}

mod existence {
    use super::*;
    use sqlx::test;

    #[test]
    async fn not_owned(pool: PgPool) {
        let (user_id, tag_repo) = init(pool.clone()).await;
        tag_repo
            .add_category(user_id, "Testing".to_string(), generate_a_z(0).to_string())
            .await
            .unwrap();
        let (user_id, tag_repo) = init(pool.clone()).await;

        let category_id_opt = tag_repo
            .get_category_id(user_id, "Testing".to_string())
            .await
            .unwrap();
        assert!(category_id_opt.is_none())
    }

    #[test]
    async fn not_exists(pool: PgPool) {
        let (user_id, tag_repo) = init(pool).await;

        let category_id_opt = tag_repo
            .get_category_id(user_id, "Random Category".to_string())
            .await
            .unwrap();
        assert!(category_id_opt.is_none())
    }
}
