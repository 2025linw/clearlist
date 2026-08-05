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
        repo::{CreateModel, PgTaskRepository, TaskRepository},
        types::TaskID,
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
                    label: format!("Tag {}", n),
                    ..Default::default()
                },
            )
            .await
            .unwrap();

        tags.push(tag);
    }

    let tag_ids = task_repo
        .set_tags(task.id, user.id, tags.iter().map(|tag| tag.id).collect())
        .await
        .unwrap()
        .iter()
        .map(|tag| tag.id)
        .collect();

    assert_eq!(
        task_repo.list_tags(task.id, user.id).await.unwrap().len(),
        5
    );

    (user.id, task.id, tag_ids, task_repo)
}

mod success {
    use sqlx::test;

    use super::*;

    #[test]
    async fn success(pool: PgPool) {
        let (user_id, task_id, _, task_repo) = init(pool).await;

        let res = task_repo.list_tags(task_id, user_id).await;
        assert!(res.is_ok());
    }

    #[test]
    async fn verify_output(pool: PgPool) {
        let (user_id, task_id, tag_ids, task_repo) = init(pool).await;

        let tags = task_repo.list_tags(task_id, user_id).await.unwrap();
        for tag in tags {
            assert!(tag_ids.contains(&tag.id));
        }
    }
}

mod existence {
    use sqlx::test;

    use super::*;

    #[test]
    async fn task_soft_deleted(pool: PgPool) {
        let (user_id, task_id, _, task_repo) = init(pool).await;
        soft_delete_task(&task_repo, task_id, user_id).await;

        let res = task_repo.list_tags(task_id, user_id).await;
        assert!(res.is_err());
        if let Err(err) = res {
            assert!(matches!(
                err,
                Error::Constraint(ConstraintViolation::Deleted(Resource::Task))
            ));
        }
    }

    #[test]
    async fn task_not_owned(pool: PgPool) {
        let (_, task_id, _, _) = init(pool.clone()).await; // other task
        let (user_id, _, _, task_repo) = init(pool).await;

        let res = task_repo.list_tags(task_id, user_id).await;
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
        let (user_id, _, _, task_repo) = init(pool).await;

        let res = task_repo.list_tags(TaskID::new_v4(), user_id).await;
        assert!(res.is_err());
        if let Err(err) = res {
            assert!(matches!(
                err,
                Error::Constraint(ConstraintViolation::NotFound(Resource::Task))
            ));
        }
    }
}
