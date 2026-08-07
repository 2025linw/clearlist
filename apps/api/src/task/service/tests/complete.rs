use crate::{
    error::{Resource, service::Error},
    task::{
        service::{TaskService, TaskServiceTrait},
        types::{TaskID, route::CreateRequest, service::UserContext},
    },
    tests::mocks::task::MockTaskRepository,
    user::types::UserID,
};

async fn init() -> (UserContext, TaskID, TaskService<MockTaskRepository>) {
    let task_service = TaskService::init(MockTaskRepository::new());

    let user_context = UserContext {
        id: task_service.repo.add_user().await,
        tz: task_service.repo.tz(),
    };
    let task = task_service
        .create(user_context, CreateRequest::default())
        .await
        .unwrap();

    (user_context, task.id, task_service)
}

mod success {
    use tokio::test;

    use super::*;

    #[test]
    async fn success() {
        let (user_context, task_id, task_service) = init().await;

        let res = task_service.complete(task_id, user_context).await;
        assert!(res.is_ok());
    }

    #[test]
    async fn is_idempotent() {
        let (user_context, task_id, task_service) = init().await;

        let res = task_service.complete(task_id, user_context).await;
        assert!(res.is_ok());
        let res = task_service.complete(task_id, user_context).await;
        assert!(res.is_ok());

        let res = task_service.get(task_id, user_context).await;
        assert!(res.is_ok());
    }
}

mod existence {
    use tokio::test;

    use super::*;

    #[test]
    async fn soft_deleted() {
        let (user_context, task_id, task_service) = init().await;
        task_service.delete(task_id, user_context).await.unwrap();

        let res = task_service.complete(task_id, user_context).await;
        if let Err(err) = res {
            assert!(matches!(err, Error::NotFound(Resource::Task)))
        }
    }

    #[test]
    async fn not_owned() {
        let (_, task_id, task_service) = init().await;
        let user_context = UserContext {
            id: task_service.repo.add_user().await,
            tz: task_service.repo.tz(),
        };

        let res = task_service.complete(task_id, user_context).await;
        assert!(res.is_err());
        if let Err(err) = res {
            assert!(matches!(err, Error::NotFound(Resource::Task)));
        }
    }

    #[test]
    async fn not_exists() {
        let (user_context, _, task_service) = init().await;

        let res = task_service.complete(TaskID::new_v4(), user_context).await;
        assert!(res.is_err());
        if let Err(err) = res {
            assert!(matches!(err, Error::NotFound(Resource::Task)))
        }
    }
}

mod error {
    use tokio::test;

    use super::*;

    #[test]
    async fn repo_backend_error() {
        let task_service = TaskService::init(MockTaskRepository::new_backend_error());
        let user_context = UserContext {
            id: UserID::new_v4(),
            tz: task_service.repo.tz(),
        };

        let res = task_service.complete(TaskID::new_v4(), user_context).await;
        assert!(res.is_err());
        if let Err(err) = res {
            assert!(matches!(err, Error::Internal(_)));
        }
    }

    #[test]
    async fn repo_programming_error() {
        let task_service = TaskService::init(MockTaskRepository::new_programming_error());
        let user_context = UserContext {
            id: UserID::new_v4(),
            tz: task_service.repo.tz(),
        };

        let res = task_service.complete(TaskID::new_v4(), user_context).await;
        assert!(res.is_err());
        if let Err(err) = res {
            assert!(matches!(err, Error::Unhandled(_)));
        }
    }
}
