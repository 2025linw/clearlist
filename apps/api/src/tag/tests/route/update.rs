use axum::http::StatusCode;
use serde_json::json;

use crate::{
    tag::types::{Tag, TagID},
    types::extract::{ErrorResponse, Response},
};

use super::{Client, init_test_setup};

async fn init(create: bool) -> (Client, String) {
    let client = init_test_setup().await;

    // create tag
    let mut tag_id = TagID::new_random();
    if create {
        let body = client
            .create(
                true,
                json!({
                    "label": "Test Tag",
                    "positionKey": "a",
                }),
            )
            .await
            .json::<Response<Tag>>()
            .await
            .unwrap();
        tag_id = body.data.id;
    }

    (client, tag_id.to_string())
}

mod success {
    use super::*;
    use tokio::test;

    #[test]
    async fn success() {
        let (client, tag_id) = init(true).await;

        let res = client.update(true, tag_id, json!({})).await;
        assert_eq!(res.status(), StatusCode::OK);
    }

    #[test]
    async fn return_body() {
        let (client, tag_id) = init(true).await;

        let body = client
            .update(true, tag_id, json!({}))
            .await
            .json::<Response<Tag>>()
            .await
            .unwrap();
        assert_eq!(body.status, StatusCode::OK);
    }

    #[test]
    async fn full_body() {
        let (client, tag_id) = init(true).await;
        client.create_category().await;

        let body = client
            .update(
                true,
                tag_id,
                json!({
                    "label": "Updated Tag",
                    "category": "Test Category",
                }),
            )
            .await
            .json::<Response<Tag>>()
            .await
            .unwrap();
        assert_eq!(body.data.label, "Updated Tag");
        assert_eq!(body.data.category.unwrap().name, "Test Category");
    }
}

mod error {
    use super::*;
    use tokio::test;

    #[test]
    async fn not_authenticated() {
        let (client, tag_id) = init(true).await;

        let res = client.update(false, tag_id, json!({})).await;
        assert_eq!(res.status(), StatusCode::UNAUTHORIZED);

        let body = res.json::<ErrorResponse>().await.unwrap();
        assert_eq!(body.status, StatusCode::UNAUTHORIZED);
    }

    #[test]
    async fn not_exists() {
        let (client, tag_id) = init(false).await;

        let res = client.update(true, tag_id, json!({})).await;
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
    async fn invalid_label_too_long() {
        // test validation
        let (client, tag_id) = init(true).await;

        let mut too_long_label = String::new();
        for _ in 0..12 {
            too_long_label.push_str("1234567890");
        }
        let res = client
            .update(
                true,
                tag_id,
                json!({
                    "label": too_long_label,
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
        let (client, tag_id) = init(true).await;

        let res = client
            .update(
                true,
                tag_id,
                json!({
                    "category": "",
                }),
            )
            .await;
        assert_eq!(res.status(), StatusCode::BAD_REQUEST);

        let body = res.json::<ErrorResponse>().await.unwrap();
        assert_eq!(body.status, StatusCode::BAD_REQUEST);
    }

    #[test]
    async fn category_not_exists() {
        let (client, tag_id) = init(true).await;

        let res = client
            .update(
                true,
                tag_id,
                json!({
                    "category": "Fake Category",
                }),
            )
            .await;
        assert_eq!(res.status(), StatusCode::NOT_FOUND);

        let body = res.json::<ErrorResponse>().await.unwrap();
        assert_eq!(body.status, StatusCode::NOT_FOUND);
    }
}
