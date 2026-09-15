use std::collections::HashMap;

use axum::http::StatusCode;
use serde::Deserialize;

use crate::{
    tag::types::Tag,
    types::extract::{ErrorResponse, Response},
};

use super::{Client, init_test_setup};

async fn init() -> Client {
    init_test_setup().await
}

#[derive(Deserialize)]
struct ListResponse {
    count: usize,
    tags: Vec<Tag>,
}

mod success {
    use super::*;
    use tokio::test;

    #[test]
    async fn success() {
        let client = init().await;

        let query = HashMap::new();
        let res = client.list(true, query).await;
        assert_eq!(res.status(), StatusCode::OK,);
    }

    #[test]
    async fn return_body() {
        let client = init().await;

        let query = HashMap::new();
        let body = client
            .list(true, query)
            .await
            .json::<Response<ListResponse>>()
            .await
            .unwrap();
        assert_eq!(body.status, StatusCode::OK);
        assert_eq!(body.data.count, body.data.tags.len());
    }
}

mod error {
    use super::*;
    use tokio::test;

    #[test]
    async fn not_authenticated() {
        let client = init().await;

        let query = HashMap::new();
        let res = client.list(false, query).await;
        assert_eq!(res.status(), StatusCode::UNAUTHORIZED);

        let body = res.json::<ErrorResponse>().await.unwrap();
        assert_eq!(body.status, StatusCode::UNAUTHORIZED);
    }

    #[test]
    async fn invalid_page_not_number() {
        // test deserialization
        let client = init().await;

        let mut query = HashMap::new();
        query.insert("page".to_string(), "five".to_string());
        let res = client.list(true, query).await;
        assert_eq!(res.status(), StatusCode::BAD_REQUEST);

        let body = res.json::<ErrorResponse>().await.unwrap();
        assert_eq!(body.status, StatusCode::BAD_REQUEST);
    }

    #[test]
    async fn invalid_page_zero() {
        // test validation
        let client = init().await;

        let mut query = HashMap::new();
        query.insert("page".to_string(), "0".to_string());
        let res = client.list(true, query).await;
        assert_eq!(res.status(), StatusCode::BAD_REQUEST);

        let body = res.json::<ErrorResponse>().await.unwrap();
        assert_eq!(body.status, StatusCode::BAD_REQUEST);
    }

    #[test]
    async fn invalid_limit_not_number() {
        // test deserialization
        let client = init().await;

        let mut query = HashMap::new();
        query.insert("limit".to_string(), "five".to_string());
        let res = client.list(true, query).await;
        assert_eq!(res.status(), StatusCode::BAD_REQUEST);

        let body = res.json::<ErrorResponse>().await.unwrap();
        assert_eq!(body.status, StatusCode::BAD_REQUEST);
    }

    #[test]
    async fn invalid_limit_zero() {
        // test validation
        let client = init().await;

        let mut query = HashMap::new();
        query.insert("limit".to_string(), "0".to_string());
        let res = client.list(true, query).await;
        assert_eq!(res.status(), StatusCode::BAD_REQUEST);

        let body = res.json::<ErrorResponse>().await.unwrap();
        assert_eq!(body.status, StatusCode::BAD_REQUEST);
    }

    #[test]
    async fn invalid_category_too_long() {
        let client = init().await;

        let mut too_long_category = String::new();
        for _ in 0..12 {
            too_long_category.push_str("1234567890");
        }
        let mut query = HashMap::new();
        query.insert("category".to_string(), too_long_category);
        let res = client.list(true, query).await;
        assert_eq!(res.status(), StatusCode::BAD_REQUEST);

        let body = res.json::<ErrorResponse>().await.unwrap();
        assert_eq!(body.status, StatusCode::BAD_REQUEST);
    }
}
