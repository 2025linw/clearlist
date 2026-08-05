use sqlx::PgPool;

use crate::{
    error::{
        Resource,
        repo::{ConstraintViolation, Error},
    },
    tag::{
        repo::{CreateModel, PgTagRepository, TagRepository},
        types::TagID,
    },
    tests::helpers::create_test_user,
    user::{repo::PgUserRepository, types::UserID},
};

async fn init(pool: PgPool) -> (UserID, TagID, PgTagRepository) {
    let user_repo = PgUserRepository::init(pool.clone());
    let tag_repo = PgTagRepository::init(pool.clone());

    let user = create_test_user(&user_repo).await;
    let tag = tag_repo
        .create(user.id, CreateModel::default())
        .await
        .unwrap();

    (user.id, tag.id, tag_repo)
}

mod success {
    use sqlx::test;

    use super::*;

    #[test]
    async fn success(pool: PgPool) {
        let (user_id, tag_id, tag_repo) = init(pool).await;

        let res = tag_repo.delete(tag_id, user_id).await;
        assert!(res.is_ok());
    }

    #[test]
    async fn verify_works(pool: PgPool) {
        let (user_id, tag_id, tag_repo) = init(pool).await;

        tag_repo.delete(tag_id, user_id).await.unwrap();
        let tag_opt = tag_repo.get(tag_id, user_id).await.unwrap();
        assert_eq!(tag_opt, None);
    }
}

mod existence {
    use sqlx::test;

    use super::*;

    #[test]
    async fn not_owned(pool: PgPool) {
        let (_, tag_id, _) = init(pool.clone()).await; // other tag
        let (user_id, _, tag_repo) = init(pool).await;

        let res = tag_repo.delete(tag_id, user_id).await;
        assert!(res.is_err());
        if let Err(err) = res {
            assert!(matches!(
                err,
                Error::Constraint(ConstraintViolation::NotFound(Resource::Tag))
            ))
        }
    }

    #[test]
    async fn not_exists(pool: PgPool) {
        let (user_id, _, tag_repo) = init(pool).await;

        let res = tag_repo.delete(TagID::new_v4(), user_id).await;
        assert!(res.is_err());
        if let Err(err) = res {
            assert!(matches!(
                err,
                Error::Constraint(ConstraintViolation::NotFound(Resource::Tag))
            ))
        }
    }
}
