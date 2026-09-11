use axum::http::StatusCode;
use serde_json::json;

use crate::{
    task::types::Task,
    types::extract::{ErrorResponse, Response},
};

use super::{Client, init_test_setup};

async fn init() -> Client {
    init_test_setup().await
}

mod success {
    use crate::types::field::Start;

    use super::*;
    use chrono::NaiveDate;
    use tokio::test;

    #[test]
    async fn success() {
        let client = init().await;

        let res = client
            .create(
                true,
                json!({
                    "title": "Test Task",
                    "positionKey": "a",
                }),
            )
            .await;
        assert_eq!(res.status(), StatusCode::CREATED);
    }

    #[test]
    async fn return_body() {
        let client = init().await;

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
        assert_eq!(body.status, StatusCode::CREATED);
    }

    #[test]
    async fn full_body() {
        let client = init().await;

        let tags = vec![
            client.create_tag().await,
            client.create_tag().await,
            client.create_tag().await,
        ];
        let body = client
            .create(
                true,
                json!({
                    "title": "Test Task",
                    "notes": "Notes for test task",
                    "start": "2026-09-01",
                    "deadline": "2026-10-01",
                    "tags": tags,
                    "positionKey": "a",
                }),
            )
            .await
            .json::<Response<Task>>()
            .await
            .unwrap();
        assert_eq!(body.data.title, "Test Task");
        assert_eq!(body.data.notes.unwrap(), "Notes for test task");
        assert_eq!(
            body.data.start.unwrap(),
            Start::Date(NaiveDate::from_ymd_opt(2026, 9, 1).unwrap())
        );
        assert_eq!(
            body.data.deadline.unwrap(),
            NaiveDate::from_ymd_opt(2026, 10, 1).unwrap()
        );
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
        let client = init().await;

        let res = client
            .create(
                false,
                json!({
                    "title": "Test Task",
                    "positionKey": "a",
                }),
            )
            .await;
        assert_eq!(res.status(), StatusCode::UNAUTHORIZED);

        let body = res.json::<ErrorResponse>().await.unwrap();
        assert_eq!(body.status, StatusCode::UNAUTHORIZED);
    }

    #[test]
    async fn invalid_title_whitespace() {
        // test validation
        let client = init().await;

        let res = client
            .create(
                true,
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
        let client = init().await;

        let res = client
            .create(
                true,
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
        let client = init().await;

        let res = client
            .create(
                true,
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
        let client = init().await;

        let res = client
            .create(
                true,
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
        let client = init().await;

        let res = client
            .create(
                true,
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
        let client = init().await;

        let res = client
            .create(
                true,
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
