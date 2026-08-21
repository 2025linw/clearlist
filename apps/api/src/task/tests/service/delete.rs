use crate::{
    error::{Resource, service::Error},
    task::{
        service::TaskService,
        types::{TaskID, route::CreateRequest},
    },
    tests::mocks::MockTaskRepository,
    types::extract::UserContext,
    user::types::UserID,
};

use super::init_test_setup;

async fn init() -> (UserContext, TaskID, TaskService<MockTaskRepository>) {
    let task_service = init_test_setup();

    let user_context = UserContext {
        id: task_service.repo.add_user().await,
        tz: task_service.repo.tz(),
    };
    let task = task_service
        .create(
            user_context,
            CreateRequest {
                title: "Test Task".to_string(),
                ..Default::default()
            },
        )
        .await
        .unwrap();

    (user_context, task.id, task_service)
}

mod success {
    use super::*;
    use tokio::test;

    #[test]
    async fn success() {
        let (user_context, task_id, task_service) = init().await;

        let res = task_service.delete(task_id, user_context).await;
        assert!(res.is_ok());
    }

    #[test]
    async fn is_idempotent() {
        let (user_context, task_id, task_service) = init().await;

        let res = task_service.delete(task_id, user_context).await;
        assert!(res.is_ok());
        let res = task_service.delete(task_id, user_context).await;
        assert!(res.is_ok());

        let res = task_service.get(task_id, user_context).await;
        assert!(res.is_err());
        if let Err(err) = res {
            assert!(matches!(err, Error::NotFound(Resource::Task)))
        }
    }
}

mod existence {
    use super::*;
    use tokio::test;

    #[test]
    async fn soft_deleted() {
        let (user_context, task_id, task_service) = init().await;
        task_service.delete(task_id, user_context).await.unwrap();

        let res = task_service.delete(task_id, user_context).await;
        assert!(res.is_ok());
    }

    #[test]
    async fn not_owned() {
        let (_, task_id, task_service) = init().await;
        let user_context = UserContext {
            id: task_service.repo.add_user().await,
            tz: task_service.repo.tz(),
        };

        let res = task_service.delete(task_id, user_context).await;
        assert!(res.is_ok());
    }

    #[test]
    async fn not_exists() {
        let (user_context, _, task_service) = init().await;

        let res = task_service
            .delete(TaskID::new_random(), user_context)
            .await;
        assert!(res.is_ok());
    }
}

mod error {
    use super::*;
    use tokio::test;

    #[test]
    async fn repo_backend_error() {
        let task_service = TaskService::init(MockTaskRepository::new_backend_error());
        let user_context = UserContext {
            id: UserID::new_random(),
            tz: task_service.repo.tz(),
        };

        let res = task_service
            .delete(TaskID::new_random(), user_context)
            .await;
        assert!(res.is_err());
        if let Err(err) = res {
            assert!(matches!(err, Error::Internal(_)));
        }
    }

    #[test]
    async fn repo_programming_error() {
        let task_service = TaskService::init(MockTaskRepository::new_programming_error());
        let user_context = UserContext {
            id: UserID::new_random(),
            tz: task_service.repo.tz(),
        };

        let res = task_service
            .delete(TaskID::new_random(), user_context)
            .await;
        assert!(res.is_err());
        if let Err(err) = res {
            assert!(matches!(err, Error::Internal(_)));
        }
    }
}
