use crate::{
    error::{Resource, service::Error},
    tag::types::TagID,
    task::{
        service::TaskService,
        types::{TaskID, route::CreateRequest},
    },
    tests::mocks::MockTaskRepository,
    types::extract::UserContext,
    user::types::UserID,
};

async fn new() -> (UserContext, TaskID, TagID, TaskService<MockTaskRepository>) {
    let task_service = TaskService::init(MockTaskRepository::new());

    let user_context = UserContext {
        id: task_service.repo.add_user().await,
        tz: task_service.repo.tz(),
    };
    let task = task_service
        .create(user_context, CreateRequest::default())
        .await
        .unwrap();
    let tag_id = task_service.repo.add_tag(user_context.id).await;

    (user_context, task.id, tag_id, task_service)
}

mod success {
    use super::*;
    use tokio::test;

    #[test]
    async fn success() {
        let (user_context, task_id, tag_id, task_service) = new().await;

        let res = task_service.add_tag(task_id, user_context, tag_id).await;
        assert!(res.is_ok());
    }

    #[test]
    async fn verify_added_tag() {
        let (user_context, task_id, tag_id, task_service) = new().await;

        task_service
            .add_tag(task_id, user_context, tag_id)
            .await
            .unwrap();

        let tag = task_service.repo.get_last_single_tag().await;
        assert_eq!(tag, tag_id);
    }

    #[test]
    async fn is_idempotent() {
        let (user_context, task_id, tag_id, task_service) = new().await;

        task_service
            .add_tag(task_id, user_context, tag_id)
            .await
            .unwrap();
        let first_add = task_service.get(task_id, user_context).await.unwrap();
        task_service
            .add_tag(task_id, user_context, tag_id)
            .await
            .unwrap();
        let second_add = task_service.get(task_id, user_context).await.unwrap();

        assert_eq!(first_add, second_add);
    }
}

mod existence {
    use super::*;
    use tokio::test;

    #[test]
    async fn task_soft_deleted() {
        let (user_context, task_id, tag_id, task_service) = new().await;
        task_service.delete(task_id, user_context).await.unwrap();

        let res = task_service.add_tag(task_id, user_context, tag_id).await;
        if let Err(err) = res {
            assert!(matches!(err, Error::NotFound(Resource::Task)))
        }
    }

    #[test]
    async fn task_not_owned() {
        let (_, task_id, tag_id, task_service) = new().await;
        let user_context = UserContext {
            id: task_service.repo.add_user().await,
            tz: task_service.repo.tz(),
        };

        let res = task_service.add_tag(task_id, user_context, tag_id).await;
        assert!(res.is_err());
        if let Err(err) = res {
            assert!(matches!(err, Error::NotFound(Resource::Task)));
        }
    }

    #[test]
    async fn task_not_exists() {
        let (user_context, _, tag_id, task_service) = new().await;

        let res = task_service
            .add_tag(TaskID::new_random(), user_context, tag_id)
            .await;
        assert!(res.is_err());
        if let Err(err) = res {
            assert!(matches!(err, Error::NotFound(Resource::Task)))
        }
    }

    #[test]
    async fn tag_not_owned() {
        let (user_context, task_id, _, task_service) = new().await;
        let tag_id = task_service
            .repo
            .add_tag(task_service.repo.add_user().await)
            .await;

        let res = task_service.add_tag(task_id, user_context, tag_id).await;
        assert!(res.is_err());
        if let Err(err) = res {
            assert!(matches!(err, Error::NotFound(Resource::Tag)))
        }
    }

    #[test]
    async fn tag_not_exists() {
        let (user_context, task_id, _, task_service) = new().await;

        let res = task_service
            .add_tag(task_id, user_context, TagID::new_random())
            .await;
        assert!(res.is_err());
        if let Err(err) = res {
            assert!(matches!(err, Error::NotFound(Resource::Tag)))
        }
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
            .add_tag(TaskID::new_random(), user_context, TagID::new_random())
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
            .add_tag(TaskID::new_random(), user_context, TagID::new_random())
            .await;
        assert!(res.is_err());
        if let Err(err) = res {
            assert!(matches!(err, Error::Internal(_)));
        }
    }
}
