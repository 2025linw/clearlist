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

async fn init(pool: PgPool) -> (UserID, TaskID, Vec<TagID>, PgTaskRepository) {
    let user_repo = PgUserRepository::init(pool.clone());
    let task_repo = PgTaskRepository::init(pool.clone());
    let tag_repo = PgTagRepository::init(pool.clone());

    let user = create_test_user(&user_repo).await;
    let task = task_repo
        .create(user.id, CreateModel::default())
        .await
        .unwrap();
    let mut tags = Vec::with_capacity(5);
    for n in 0..5 {
        let tag = tag_repo
            .create(
                user.id,
                TagCreateModel {
                    label: format!("Tag {n}"),
                    ..Default::default()
                },
            )
            .await
            .unwrap();

        tags.push(tag);
    }

    (
        user.id,
        task.id,
        tags.iter().map(|tag| tag.id).collect(),
        task_repo,
    )
}

mod success {
    use super::*;
    use sqlx::test;

    #[test]
    async fn success(pool: PgPool) {
        let (user_id, task_id, tag_ids, task_repo) = init(pool).await;

        let res = task_repo.set_tags(task_id, user_id, tag_ids).await;
        assert!(res.is_ok());
    }

    #[test]
    async fn verify_works(pool: PgPool) {
        let (user_id, task_id, tag_ids, task_repo) = init(pool).await;

        let tags = task_repo
            .set_tags(task_id, user_id, tag_ids.clone())
            .await
            .unwrap();
        assert_eq!(tags.len(), tag_ids.len());
        for tag in tags {
            assert!(tag_ids.contains(&tag.id));
        }

        let tags = task_repo.list_tags(task_id, user_id).await.unwrap();
        assert_eq!(tags.len(), tag_ids.len());
        for tag in tags {
            assert!(tag_ids.contains(&tag.id));
        }
    }

    #[test]
    async fn updates_updated_at(pool: PgPool) {
        let (user_id, task_id, tag_ids, task_repo) = init(pool).await;
        let task_init = task_repo.get(task_id, user_id).await.unwrap().unwrap();

        task_repo.set_tags(task_id, user_id, tag_ids).await.unwrap();

        let task = task_repo.get(task_id, user_id).await.unwrap().unwrap();
        assert!(task.updated_at > task_init.updated_at);
    }

    #[test]
    async fn is_idempotent(pool: PgPool) {
        let (user_id, task_id, tag_ids, task_repo) = init(pool).await;

        task_repo
            .set_tags(task_id, user_id, tag_ids.clone())
            .await
            .unwrap();
        let first_add = task_repo.get(task_id, user_id).await.unwrap().unwrap();
        task_repo.set_tags(task_id, user_id, tag_ids).await.unwrap();
        let second_add = task_repo.get(task_id, user_id).await.unwrap().unwrap();

        assert_eq!(first_add.updated_at, second_add.updated_at);
    }

    #[test]
    async fn full_replacement(pool: PgPool) {
        let (user_id, task_id, tag_ids, task_repo) = init(pool).await;
        task_repo
            .set_tags(task_id, user_id, tag_ids[0..2].to_vec())
            .await
            .unwrap();

        let tags = task_repo
            .set_tags(task_id, user_id, tag_ids.clone())
            .await
            .unwrap();
        assert_eq!(tags.len(), tag_ids.len());
        for tag in tags {
            assert!(tag_ids.contains(&tag.id));
        }

        let tags = task_repo.list_tags(task_id, user_id).await.unwrap();
        assert_eq!(tags.len(), tag_ids.len());
        for tag in tags {
            assert!(tag_ids.contains(&tag.id));
        }
    }
}

mod existence {
    use super::*;
    use sqlx::test;

    #[test]
    async fn task_soft_deleted(pool: PgPool) {
        let (user_id, task_id, tag_ids, task_repo) = init(pool).await;
        soft_delete_task(&task_repo, task_id, user_id).await;

        let res = task_repo.set_tags(task_id, user_id, tag_ids).await;
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
        let (user_id, _, tag_ids, task_repo) = init(pool).await;

        let res = task_repo.set_tags(task_id, user_id, tag_ids).await;
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
        let (user_id, _, tag_ids, task_repo) = init(pool).await;

        let res = task_repo
            .set_tags(TaskID::new_random(), user_id, tag_ids)
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
    async fn tags_not_owned(pool: PgPool) {
        let (_, _, tag_ids, _) = init(pool.clone()).await; // other tags
        let (user_id, task_id, _, task_repo) = init(pool).await;

        let res = task_repo.set_tags(task_id, user_id, tag_ids).await;
        assert!(res.is_err());
        if let Err(err) = res {
            assert!(matches!(
                err,
                Error::Constraint(ConstraintViolation::NotFound(Resource::Tag))
            ))
        }
    }

    #[test]
    async fn tags_not_exist(pool: PgPool) {
        let (user_id, task_id, _, task_repo) = init(pool).await;

        let res = task_repo
            .set_tags(task_id, user_id, vec![TagID::new_random()])
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
