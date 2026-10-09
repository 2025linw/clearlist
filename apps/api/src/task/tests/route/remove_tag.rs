use axum::http::StatusCode;
use serde_json::json;

use crate::{
    tag::types::TagID,
    task::types::{Task, TaskID},
    types::extract::{ErrorResponse, Response},
};

use super::{Client, init_test_setup};

async fn init(create_task: bool, create_tag: bool) -> (Client, String, String) {
    let client = init_test_setup().await;

    // create tag
    let mut tag_id = TagID::new_random();
    if create_tag {
        tag_id = client.create_tag().await;
    }

    // create task
    let mut task_id = TaskID::new_random();
    if create_task {
        let body = client
            .create(
                true,
                json!({
                    "title": "Test Task",
                    "positionKey": "a",
                }),
            )
            .await
            .json::<Response<Task>>()
            .await
            .unwrap();
        task_id = body.data.id;
    }

    (client, task_id.to_string(), tag_id.to_string())
}

mod success {
    use super::*;
    use tokio::test;

    #[test]
    async fn success() {
        let (client, task_id, tag_id) = init(true, true).await;

        let res = client.remove_tag(true, task_id, tag_id).await;
        assert_eq!(res.status(), StatusCode::NO_CONTENT);
    }

    #[test]
    async fn tag_not_exists() {
        let (client, task_id, tag_id) = init(true, false).await;

        let res = client.remove_tag(true, task_id, tag_id).await;
        assert_eq!(res.status(), StatusCode::NO_CONTENT);
    }
}

mod error {
    use super::*;
    use tokio::test;

    #[test]
    async fn not_authenticated() {
        let (client, task_id, tag_id) = init(true, true).await;

        let res = client.remove_tag(false, task_id, tag_id).await;
        assert_eq!(res.status(), StatusCode::UNAUTHORIZED);

        let body = res.json::<ErrorResponse>().await.unwrap();
        assert_eq!(body.status, StatusCode::UNAUTHORIZED);
    }

    #[test]
    async fn task_not_exists() {
        let (client, task_id, tag_id) = init(false, true).await;

        let res = client.remove_tag(true, task_id, tag_id).await;
        assert_eq!(res.status(), StatusCode::NOT_FOUND);

        let body = res.json::<ErrorResponse>().await.unwrap();
        assert_eq!(body.status, StatusCode::NOT_FOUND);
    }

    #[test]
    async fn invalid_path_id() {
        // test deserialization
        let (client, _, tag_id) = init(true, true).await;

        let res = client
            .remove_tag(true, "InvalidPath".to_string(), tag_id)
            .await;
        assert_eq!(res.status(), StatusCode::BAD_REQUEST);

        let body = res.json::<ErrorResponse>().await.unwrap();
        assert_eq!(body.status, StatusCode::BAD_REQUEST);
    }
}
