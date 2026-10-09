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

        let res = client.get(true, tag_id).await;
        assert_eq!(res.status(), StatusCode::OK);
    }

    #[test]
    async fn return_body() {
        let (client, tag_id) = init(true).await;

        let body = client
            .get(true, tag_id)
            .await
            .json::<Response<Tag>>()
            .await
            .unwrap();
        assert_eq!(body.status, StatusCode::OK);
    }
}

mod error {
    use super::*;
    use tokio::test;

    #[test]
    async fn not_authenticated() {
        let (client, tag_id) = init(true).await;

        let res = client.get(false, tag_id).await;
        assert_eq!(res.status(), StatusCode::UNAUTHORIZED);

        let body = res.json::<ErrorResponse>().await.unwrap();
        assert_eq!(body.status, StatusCode::UNAUTHORIZED);
    }

    #[test]
    async fn not_exists() {
        let (client, tag_id) = init(false).await;

        let res = client.get(true, tag_id).await;
        assert_eq!(res.status(), StatusCode::NOT_FOUND);

        let body = res.json::<ErrorResponse>().await.unwrap();
        assert_eq!(body.status, StatusCode::NOT_FOUND);
    }

    #[test]
    async fn invalid_path_id() {
        // test deserialization
        let (client, _) = init(true).await;

        let res = client.get(true, "InvalidPath".to_string()).await;
        assert_eq!(res.status(), StatusCode::BAD_REQUEST);

        let body = res.json::<ErrorResponse>().await.unwrap();
        assert_eq!(body.status, StatusCode::BAD_REQUEST);
    }
}
