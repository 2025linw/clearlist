use axum::http::StatusCode;
use serde_json::json;

use crate::{
    types::extract::{ErrorResponse, Response},
    user::types::User,
};

use super::{Client, init_test_setup};

async fn init() -> Client {
    init_test_setup().await
}

mod success {
    use crate::types::field::CompletedTaskRetention;

    use super::*;
    use chrono_tz::Tz;
    use tokio::test;

    #[test]
    async fn success() {
        let client = init().await;

        let res = client.update(true, json!({})).await;
        assert_eq!(res.status(), StatusCode::OK);
    }

    #[test]
    async fn return_body() {
        let client = init().await;

        let body = client
            .update(true, json!({}))
            .await
            .json::<Response<User>>()
            .await
            .unwrap();
        assert_eq!(body.status, StatusCode::OK);
    }

    #[test]
    async fn full_body() {
        let client = init().await;

        let res = client
            .update(
                true,
                json!({
                    "displayName": "Updated Name",
                    "preferredTimezone": "America/Chicago",
                    "completedTaskRetention": "hour",
                }),
            )
            .await;
        assert_eq!(res.status(), StatusCode::OK);

        let body = client
            .get(true)
            .await
            .json::<Response<User>>()
            .await
            .unwrap();
        assert_eq!(body.data.display_name, "Updated Name");
        assert_eq!(body.data.preferred_timezone.unwrap(), Tz::America__Chicago);
        assert_eq!(
            body.data.completed_task_retention.unwrap(),
            CompletedTaskRetention::EveryHour
        )
    }
}

mod error {
    use super::*;
    use tokio::test;

    #[test]
    async fn not_authenticated() {
        let client = init().await;

        let res = client.update(false, json!({})).await;
        assert_eq!(res.status(), StatusCode::UNAUTHORIZED);

        let body = res.json::<ErrorResponse>().await.unwrap();
        assert_eq!(body.status, StatusCode::UNAUTHORIZED);
    }

    #[test]
    async fn invalid_display_name() {
        let client = init().await;

        let res = client
            .update(
                true,
                json!({
                    "displayName": "New\nLines\nIn\nMiddle",
                }),
            )
            .await;
        assert_eq!(res.status(), StatusCode::BAD_REQUEST);

        let body = res.json::<ErrorResponse>().await.unwrap();
        assert_eq!(body.status, StatusCode::BAD_REQUEST);
    }

    #[test]
    async fn invalid_preferred_timezone() {
        let client = init().await;

        let res = client
            .update(
                true,
                json!({
                    "preferredTimezone": "Invalid",
                }),
            )
            .await;
        assert_eq!(res.status(), StatusCode::BAD_REQUEST);

        let body = res.json::<ErrorResponse>().await.unwrap();
        assert_eq!(body.status, StatusCode::BAD_REQUEST);
    }

    #[test]
    async fn invalid_completed_task_retention() {
        let client = init().await;

        let res = client
            .update(
                true,
                json!({
                    "completedTaskRetention": "hours",
                }),
            )
            .await;
        assert_eq!(res.status(), StatusCode::BAD_REQUEST);

        let body = res.json::<ErrorResponse>().await.unwrap();
        assert_eq!(body.status, StatusCode::BAD_REQUEST);
    }
}
