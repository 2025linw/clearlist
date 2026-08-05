use sqlx::PgPool;

use crate::{
    error::{
        Resource,
        repo::{ConstraintViolation, Error},
    },
    task::{
        repo::{CreateModel, PgTaskRepository, TaskRepository},
        types::{TaskID, repo::TaskState},
    },
    tests::helpers::{create_test_user, soft_delete_task},
    user::{repo::PgUserRepository, types::UserID},
};

async fn init(pool: PgPool) -> (UserID, TaskID, PgTaskRepository) {
    let user_repo = PgUserRepository::init(pool.clone());
    let task_repo = PgTaskRepository::init(pool.clone());

    let user = create_test_user(&user_repo).await;
    let task = task_repo
        .create(user.id, CreateModel::default())
        .await
        .unwrap();

    (user.id, task.id, task_repo)
}

mod success {
    use sqlx::test;

    use super::*;

    #[test]
    async fn success(pool: PgPool) {
        let (user_id, task_id, task_repo) = init(pool).await;

        let res = task_repo.delete(task_id, user_id).await;
        assert!(res.is_ok());
    }

    #[test]
    async fn verify_works(pool: PgPool) {
        let (user_id, task_id, task_repo) = init(pool).await;

        task_repo.delete(task_id, user_id).await.unwrap();
        let task_state = task_repo.get(task_id, user_id).await.unwrap();
        assert_eq!(task_state, TaskState::None);
    }
}

mod existence {
    use sqlx::test;

    use super::*;

    #[test]
    async fn soft_deleted(pool: PgPool) {
        let (user_id, task_id, task_repo) = init(pool).await;
        soft_delete_task(&task_repo, task_id, user_id).await;

        let res = task_repo.delete(task_id, user_id).await;
        assert!(res.is_ok());
    }

    #[test]
    async fn not_owned(pool: PgPool) {
        let (_, task_id, _) = init(pool.clone()).await; // other task
        let (user_id, _, task_repo) = init(pool).await;

        let res = task_repo.delete(task_id, user_id).await;
        assert!(res.is_err());
        if let Err(err) = res {
            assert!(matches!(
                err,
                Error::Constraint(ConstraintViolation::NotFound(Resource::Task))
            ));
        }
    }

    #[test]
    async fn not_exists(pool: PgPool) {
        let (user_id, _, task_repo) = init(pool).await;

        let res = task_repo.delete(TaskID::new_v4(), user_id).await;
        assert!(res.is_err());
        if let Err(err) = res {
            assert!(matches!(
                err,
                Error::Constraint(ConstraintViolation::NotFound(Resource::Task))
            ));
        }
    }
}
