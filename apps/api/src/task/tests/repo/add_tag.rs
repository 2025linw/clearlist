use sqlx::PgPool;

use crate::{
    error::{
        Resource,
        repo::{ConstraintViolation, Error},
    },
    tag::{
        repo::{PgTagRepository, TagRepository},
        types::{TagID, repo::CreateModel as TagCreateModel},
    },
    task::{
        repo::{PgTaskRepository, TaskRepository},
        types::{TaskID, repo::CreateModel},
    },
    tests::helpers::{create_test_user, soft_delete_task},
    user::{repo::PgUserRepository, types::UserID},
};

async fn init(pool: PgPool) -> (UserID, TaskID, TagID, PgTaskRepository) {
    let user_repo = PgUserRepository::init(pool.clone());
    let task_repo = PgTaskRepository::init(pool.clone());
    let tag_repo = PgTagRepository::init(pool.clone());

    let user = create_test_user(&user_repo).await;
    let task = task_repo
        .create(user.id, CreateModel::default())
        .await
        .unwrap();
    let tag = tag_repo
        .create(user.id, TagCreateModel::default())
        .await
        .unwrap();

    (user.id, task.id, tag.id, task_repo)
}

mod success {
    use super::*;
    use sqlx::test;

    #[test]
    async fn success(pool: PgPool) {
        let (user_id, task_id, tag_id, task_repo) = init(pool).await;

        let res = task_repo.add_tag(task_id, user_id, tag_id).await;
        assert!(res.is_ok());
    }

    #[test]
    async fn verify_works(pool: PgPool) {
        let (user_id, task_id, tag_id, task_repo) = init(pool).await;

        task_repo.add_tag(task_id, user_id, tag_id).await.unwrap();

        let tag = task_repo
            .list_tags(task_id, user_id)
            .await
            .unwrap()
            .first()
            .unwrap()
            .to_owned();
        assert_eq!(tag.id, tag_id);
    }

    #[test]
    async fn updates_updated_at(pool: PgPool) {
        let (user_id, task_id, tag_id, task_repo) = init(pool).await;
        let task_init = task_repo.get(task_id, user_id).await.unwrap().unwrap();

        task_repo.add_tag(task_id, user_id, tag_id).await.unwrap();
        let task = task_repo.get(task_id, user_id).await.unwrap().unwrap();
        assert!(task.updated_at > task_init.updated_at);
    }

    #[test]
    async fn is_idempotent(pool: PgPool) {
        let (user_id, task_id, tag_id, task_repo) = init(pool).await;

        task_repo.add_tag(task_id, user_id, tag_id).await.unwrap();
        let first_add = task_repo.get(task_id, user_id).await.unwrap().unwrap();
        task_repo.add_tag(task_id, user_id, tag_id).await.unwrap();
        let second_add = task_repo.get(task_id, user_id).await.unwrap().unwrap();

        assert_eq!(first_add.updated_at, second_add.updated_at);
    }
}

mod existence {
    use super::*;
    use sqlx::test;

    #[test]
    async fn task_soft_deleted(pool: PgPool) {
        let (user_id, task_id, tag_id, task_repo) = init(pool).await;
        soft_delete_task(&task_repo, task_id, user_id).await;

        let res = task_repo.add_tag(task_id, user_id, tag_id).await;
        assert!(res.is_err());
        if let Err(err) = res {
            assert!(matches!(
                err,
                Error::Constraint(ConstraintViolation::SoftDeleted(Resource::Task))
            ));
        }
    }

    #[test]
    async fn task_not_owned(pool: PgPool) {
        let (_, task_id, _, _) = init(pool.clone()).await; // other task
        let (user_id, _, tag_id, task_repo) = init(pool).await;

        let res = task_repo.add_tag(task_id, user_id, tag_id).await;
        assert!(res.is_err());
        if let Err(err) = res {
            assert!(matches!(
                err,
                Error::Constraint(ConstraintViolation::NotFound(Resource::Task))
            ));
        }
    }

    #[test]
    async fn task_not_exist(pool: PgPool) {
        let (user_id, _, tag_id, task_repo) = init(pool).await;

        let res = task_repo
            .add_tag(TaskID::new_random(), user_id, tag_id)
            .await;
        assert!(res.is_err());
        if let Err(err) = res {
            assert!(matches!(
                err,
                Error::Constraint(ConstraintViolation::NotFound(Resource::Task))
            ));
        }
    }

    #[test]
    async fn tag_not_owned(pool: PgPool) {
        let (_, _, tag_id, _) = init(pool.clone()).await; // other tag
        let (user_id, task_id, _, task_repo) = init(pool).await;

        let res = task_repo.add_tag(task_id, user_id, tag_id).await;
        assert!(res.is_err());
        if let Err(err) = res {
            assert!(matches!(
                err,
                Error::Constraint(ConstraintViolation::NotFound(Resource::Tag))
            ))
        }
    }

    #[test]
    async fn tag_not_exist(pool: PgPool) {
        let (user_id, task_id, _, task_repo) = init(pool).await;

        let res = task_repo
            .add_tag(task_id, user_id, TagID::new_random())
            .await;
        assert!(res.is_err());
        if let Err(err) = res {
            assert!(matches!(
                err,
                Error::Constraint(ConstraintViolation::NotFound(Resource::Tag))
            ))
        }
    }
}
