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

        let res = client.update(true, task_id, json!({})).await;
        assert_eq!(res.status(), StatusCode::OK);
    }

    #[test]
    async fn return_body() {
        let (client, task_id) = init(true).await;

        let body = client
            .update(true, task_id, json!({}))
            .await
            .json::<Response<Task>>()
            .await
            .unwrap();
        assert_eq!(body.status, StatusCode::OK);
    }

    #[test]
    async fn full_body() {
        let (client, task_id) = init(true).await;

        let tags = vec![
            client.create_tag().await,
            client.create_tag().await,
            client.create_tag().await,
        ];
        let body = client
            .update(
                true,
                task_id,
                json!({
                    "title": "Updated Task",
                    "notes": "Updated notes for test task",
                    "start": "2026-09-01",
                    "deadline": "2026-10-01",
                    "tags": tags,
                }),
            )
            .await
            .json::<Response<Task>>()
            .await
            .unwrap();
        assert_eq!(body.data.title, "Updated Task");
        assert_eq!(body.data.notes.unwrap(), "Updated notes for test task");
        assert_eq!(body.data.start.unwrap().to_string(), "2026-09-01");
        assert_eq!(body.data.deadline.unwrap().to_string(), "2026-10-01");
        for tag in body.data.tags {
            assert!(tags.contains(&tag.id));
        }
    }
}

mod error {
    use super::*;
    use tokio::test;

    #[test]
    async fn not_authenticated() {
        let (client, task_id) = init(true).await;

        let res = client.update(false, task_id, json!({})).await;
        assert_eq!(res.status(), StatusCode::UNAUTHORIZED);

        let body = res.json::<ErrorResponse>().await.unwrap();
        assert_eq!(body.status, StatusCode::UNAUTHORIZED);
    }

    #[test]
    async fn not_exists() {
        let (client, task_id) = init(false).await;

        let res = client.update(true, task_id, json!({})).await;
        assert_eq!(res.status(), StatusCode::NOT_FOUND);

        let body = res.json::<ErrorResponse>().await.unwrap();
        assert_eq!(body.status, StatusCode::NOT_FOUND);
    }

    #[test]
    async fn invalid_path_id() {
        // test deserialization
        let (client, _) = init(true).await;

        let res = client
            .update(true, "InvalidPath".to_string(), json!({}))
            .await;
        assert_eq!(res.status(), StatusCode::BAD_REQUEST);

        let body = res.json::<ErrorResponse>().await.unwrap();
        assert_eq!(body.status, StatusCode::BAD_REQUEST);
    }

    #[test]
    async fn invalid_title_whitespace() {
        // test validation
        let (client, task_id) = init(true).await;

        let res = client
            .update(
                true,
                task_id,
                json!({
                    "title": "Invalid\tTitle\tFor\tNote",
                }),
            )
            .await;
        assert_eq!(res.status(), StatusCode::BAD_REQUEST);

        let body = res.json::<ErrorResponse>().await.unwrap();
        assert_eq!(body.status, StatusCode::BAD_REQUEST);
    }

    #[test]
    async fn invalid_notes_whitespace() {
        // test validation
        let (client, task_id) = init(true).await;

        let res = client
            .update(
                true,
                task_id,
                json!({
                    "notes": "Invalid\rNote\rFor\rTask",
                }),
            )
            .await;
        assert_eq!(res.status(), StatusCode::BAD_REQUEST);

        let body = res.json::<ErrorResponse>().await.unwrap();
        assert_eq!(body.status, StatusCode::BAD_REQUEST);
    }

    #[test]
    async fn invalid_start_not_date() {
        // test deserialization
        let (client, task_id) = init(true).await;

        let res = client
            .update(
                true,
                task_id,
                json!({
                    "start": "Invalid",
                }),
            )
            .await;
        assert_eq!(res.status(), StatusCode::BAD_REQUEST);

        let body = res.json::<ErrorResponse>().await.unwrap();
        assert_eq!(body.status, StatusCode::BAD_REQUEST);
    }

    #[test]
    async fn invalid_deadline_not_date() {
        // test deserialization
        let (client, task_id) = init(true).await;

        let res = client
            .update(
                true,
                task_id,
                json!({
                    "deadline": "Invalid",
                }),
            )
            .await;
        assert_eq!(res.status(), StatusCode::BAD_REQUEST);

        let body = res.json::<ErrorResponse>().await.unwrap();
        assert_eq!(body.status, StatusCode::BAD_REQUEST);
    }

    #[test]
    async fn invalid_deadline_contains_time_component() {
        // test deserialization
        let (client, task_id) = init(true).await;

        let res = client
            .update(
                true,
                task_id,
                json!({
                    "deadline": "2026-10-01T12:00:00Z",
                }),
            )
            .await;
        assert_eq!(res.status(), StatusCode::BAD_REQUEST);

        let body = res.json::<ErrorResponse>().await.unwrap();
        assert_eq!(body.status, StatusCode::BAD_REQUEST);
    }

    #[test]
    async fn invalid_tags_not_uuid() {
        // test deserialization
        let (client, task_id) = init(true).await;

        let res = client
            .update(
                true,
                task_id,
                json!({
                    "tags": "Invalid,Invalid,Invalid",
                }),
            )
            .await;
        assert_eq!(res.status(), StatusCode::BAD_REQUEST);

        let body = res.json::<ErrorResponse>().await.unwrap();
        assert_eq!(body.status, StatusCode::BAD_REQUEST);
    }
}
