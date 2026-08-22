use axum::http::StatusCode;
use serde_json::json;

use crate::{
    task::types::{Task, TaskID},
    types::extract::{ErrorResponse, Response},
};

use super::{Client, init_test_setup};

async fn init(create: bool) -> (Client, String) {
    let client = init_test_setup().await;

    // create task
    let mut task_id = TaskID::new_random();
    if create {
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

    (client, task_id.to_string())
}

mod success {
    use super::*;
    use tokio::test;

    #[test]
    async fn success() {
        let (client, task_id) = init(true).await;

        let res = client.reopen(true, task_id).await;
        assert_eq!(res.status(), StatusCode::NO_CONTENT);
    }
}

mod error {
    use super::*;
    use tokio::test;

    #[test]
    async fn not_authenticated() {
        let (client, task_id) = init(true).await;

        let res = client.reopen(false, task_id).await;
        assert_eq!(res.status(), StatusCode::UNAUTHORIZED);

        let body = res.json::<ErrorResponse>().await.unwrap();
        assert_eq!(body.status, StatusCode::UNAUTHORIZED);
    }

    #[test]
    async fn not_exists() {
        let (client, task_id) = init(false).await;

        let res = client.reopen(true, task_id).await;
        assert_eq!(res.status(), StatusCode::NOT_FOUND);

        let body = res.json::<ErrorResponse>().await.unwrap();
        assert_eq!(body.status, StatusCode::NOT_FOUND);
    }

    #[test]
    async fn invalid_path_id() {
        // test deserialization
        let (client, _) = init(true).await;

        let res = client.reopen(true, "InvalidPath".to_string()).await;
        assert_eq!(res.status(), StatusCode::BAD_REQUEST);

        let body = res.json::<ErrorResponse>().await.unwrap();
        assert_eq!(body.status, StatusCode::BAD_REQUEST);
    }
}
