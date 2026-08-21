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

use super::init_test_setup;

async fn init() -> (
    UserContext,
    TaskID,
    Vec<TagID>,
    TaskService<MockTaskRepository>,
) {
    let task_service = init_test_setup();

    let user_context = UserContext {
        id: task_service.repo.add_user().await,
        tz: task_service.repo.tz(),
    };
    let task = task_service
        .create(user_context, CreateRequest::default())
        .await
        .unwrap();
    let mut tag_ids = Vec::with_capacity(5);
    for _ in 0..5 {
        let tag_id = task_service.repo.add_tag(user_context.id).await;

        tag_ids.push(tag_id);
    }

    (user_context, task.id, tag_ids, task_service)
}

mod success {
    use super::*;
    use tokio::test;

    #[test]
    async fn success() {
        let (user_context, task_id, tag_ids, task_service) = init().await;

        let res = task_service.set_tags(task_id, user_context, tag_ids).await;
        assert!(res.is_ok());
    }

    #[test]
    async fn is_idempotent() {
        let (user_context, task_id, tag_ids, task_service) = init().await;

        task_service
            .set_tags(task_id, user_context, tag_ids.clone())
            .await
            .unwrap();
        let first_add = task_service.get(task_id, user_context).await.unwrap();
        task_service
            .set_tags(task_id, user_context, tag_ids)
            .await
            .unwrap();
        let second_add = task_service.get(task_id, user_context).await.unwrap();

        assert_eq!(first_add, second_add);
    }

    #[test]
    async fn deduplicate_tags() {
        let (user_context, task_id, _, task_service) = init().await;

        let tag_id = task_service.repo.add_tag(user_context.id).await;
        let res = task_service
            .set_tags(task_id, user_context, vec![tag_id, tag_id, tag_id])
            .await;
        assert!(res.is_ok());

        let tags = task_service.repo.get_last_multi_tag().await;
        assert_eq!(tags.len(), 1);
    }
}

mod existence {
    use super::*;
    use tokio::test;

    #[test]
    async fn task_soft_deleted() {
        let (user_context, task_id, tag_ids, task_service) = init().await;
        task_service.delete(task_id, user_context).await.unwrap();

        let res = task_service.set_tags(task_id, user_context, tag_ids).await;
        if let Err(err) = res {
            assert!(matches!(err, Error::NotFound(Resource::Task)))
        }
    }

    #[test]
    async fn task_not_owned() {
        let (_, task_id, tag_ids, task_service) = init().await;
        let user_context = UserContext {
            id: task_service.repo.add_user().await,
            tz: task_service.repo.tz(),
        };

        let res = task_service.set_tags(task_id, user_context, tag_ids).await;
        assert!(res.is_err());
        if let Err(err) = res {
            assert!(matches!(err, Error::NotFound(Resource::Task)));
        }
    }

    #[test]
    async fn task_not_exists() {
        let (user_context, _, tag_ids, task_service) = init().await;

        let res = task_service
            .set_tags(TaskID::new_random(), user_context, tag_ids)
            .await;
        assert!(res.is_err());
        if let Err(err) = res {
            assert!(matches!(err, Error::NotFound(Resource::Task)))
        }
    }

    #[test]
    async fn tag_not_owned() {
        let (user_context, task_id, _, task_service) = init().await;
        let tag_id = task_service
            .repo
            .add_tag(task_service.repo.add_user().await)
            .await;

        let res = task_service
            .set_tags(task_id, user_context, vec![tag_id])
            .await;
        assert!(res.is_err());
        if let Err(err) = res {
            assert!(matches!(err, Error::NotFound(Resource::Tag)))
        }
    }

    #[test]
    async fn tag_not_exists() {
        let (user_context, task_id, _, task_service) = init().await;

        let res = task_service
            .set_tags(task_id, user_context, vec![TagID::new_random()])
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
            .set_tags(
                TaskID::new_random(),
                user_context,
                vec![TagID::new_random()],
            )
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
            .set_tags(
                TaskID::new_random(),
                user_context,
                vec![TagID::new_random()],
            )
            .await;
        assert!(res.is_err());
        if let Err(err) = res {
            assert!(matches!(err, Error::Internal(_)));
        }
    }
}
