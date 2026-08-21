use axum::http::StatusCode;
use serde_json::json;

use crate::{
    tag::types::Tag,
    types::extract::{ErrorResponse, Response},
};

use super::{Client, init_test_setup};

async fn init() -> Client {
    init_test_setup().await
}

mod success {
    use super::*;
    use tokio::test;

    #[test]
    async fn success() {
        let client = init().await;

        let res = client
            .create(
                true,
                json!({
                    "label": "Test Tag",
                    "positionKey": "a",
                }),
            )
            .await;
        assert_eq!(res.status(), StatusCode::CREATED);
    }

    #[test]
    async fn return_body() {
        let client = init().await;
        client.create_category().await;

        let body = client
            .create(
                true,
                json!({
                    "label": "Test Tag",
                    "category": "Test Category",
                    "positionKey": "a",
                }),
            )
            .await
            .json::<Response<Tag>>()
            .await
            .unwrap();
        assert_eq!(body.status, StatusCode::CREATED);
    }

    #[test]
    async fn full_body() {
        let client = init().await;
        client.create_category().await;

        let body = client
            .create(
                true,
                json!({
                    "label": "Test Tag",
                    "category": "Test Category",
                    "positionKey": "a",
                }),
            )
            .await
            .json::<Response<Tag>>()
            .await
            .unwrap();
        assert_eq!(body.data.label, "Test Tag");
        assert_eq!(body.data.category.unwrap().name, "Test Category");
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
                    "label": "Test Tag",
                    "positionKey": "a",
                }),
            )
            .await;
        assert_eq!(res.status(), StatusCode::UNAUTHORIZED);

        let body = res.json::<ErrorResponse>().await.unwrap();
        assert_eq!(body.status, StatusCode::UNAUTHORIZED);
    }

    #[test]
    async fn invalid_label_too_long() {
        // test validation
        let client = init().await;

        let mut too_long_label = String::new();
        for _ in 0..12 {
            too_long_label.push_str("1234567890");
        }
        let res = client
            .create(
                true,
                json!({
                    "label": too_long_label,
                    "positionKey": "a",
                }),
            )
            .await;
        assert_eq!(res.status(), StatusCode::BAD_REQUEST);

        let body = res.json::<ErrorResponse>().await.unwrap();
        assert_eq!(body.status, StatusCode::BAD_REQUEST);
    }

    #[test]
    async fn invalid_category_empty() {
        // test validation
        let client = init().await;

        let res = client
            .create(
                true,
                json!({
                    "label": "Test Tag",
                    "category": "",
                    "positionKey": "a",
                }),
            )
            .await;
        assert_eq!(res.status(), StatusCode::BAD_REQUEST);

        let body = res.json::<ErrorResponse>().await.unwrap();
        assert_eq!(body.status, StatusCode::BAD_REQUEST);
    }

    #[test]
    async fn category_not_exists() {
        let client = init().await;

        let res = client
            .create(
                true,
                json!({
                    "label": "Test Tag",
                    "category": "Fake Category",
                    "positionKey": "a",
                }),
            )
            .await;
        assert_eq!(res.status(), StatusCode::NOT_FOUND);

        let body = res.json::<ErrorResponse>().await.unwrap();
        assert_eq!(body.status, StatusCode::NOT_FOUND);
    }
}
