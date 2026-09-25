use std::collections::HashMap;

use axum::http::StatusCode;
use serde::Deserialize;

use crate::{
    task::types::Task,
    types::extract::{ErrorResponse, Response},
};

use super::{Client, init_test_setup};

async fn init() -> Client {
    init_test_setup().await
}

#[derive(Deserialize)]
struct ListResponse {
    count: usize,
    tasks: Vec<Task>,
}

mod success {
    use super::*;
    use tokio::test;

    #[test]
    async fn success() {
        let client = init().await;

        let query = HashMap::new();
        let res = client.list(true, query).await;
        assert_eq!(res.status(), StatusCode::OK)
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
        assert_eq!(body.data.count, body.data.tasks.len());
    }

    #[test]
    async fn start_date() {
        let client = init().await;

        let mut query = HashMap::new();
        query.insert("start".to_string(), "2025-01-01".to_string());
        let body = client
            .list(true, query)
            .await
            .json::<Response<ListResponse>>()
            .await
            .unwrap();
        assert_eq!(body.status, StatusCode::OK);
    }

    #[test]
    async fn start_datetime() {
        let client = init().await;

        let mut query = HashMap::new();
        query.insert("start".to_string(), "2025-01-01T12:00:00Z".to_string());
        let body = client
            .list(true, query)
            .await
            .json::<Response<ListResponse>>()
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
    async fn invalid_sort_by_value() {
        // test deserialization
        let client = init().await;

        let mut query = HashMap::new();
        query.insert("sort".to_string(), "invalid".to_string());
        let res = client.list(true, query).await;
        assert_eq!(res.status(), StatusCode::BAD_REQUEST);

        let body = res.json::<ErrorResponse>().await.unwrap();
        assert_eq!(body.status, StatusCode::BAD_REQUEST);
    }

    #[test]
    async fn invalid_order_value() {
        // test deserialization
        let client = init().await;

        let mut query = HashMap::new();
        query.insert("order".to_string(), "invalid".to_string());
        let res = client.list(true, query).await;
        assert_eq!(res.status(), StatusCode::BAD_REQUEST);

        let body = res.json::<ErrorResponse>().await.unwrap();
        assert_eq!(body.status, StatusCode::BAD_REQUEST);
    }

    #[test]
    async fn invalid_start() {
        // test deserialization
        let client = init().await;

        let mut query = HashMap::new();
        query.insert("start".to_string(), "invalid".to_string());
        let res = client.list(true, query).await;
        assert_eq!(res.status(), StatusCode::BAD_REQUEST);

        let body = res.json::<ErrorResponse>().await.unwrap();
        assert_eq!(body.status, StatusCode::BAD_REQUEST);
    }

    #[test]
    async fn invalid_deadline() {
        // test deserialization
        let client = init().await;

        let mut query = HashMap::new();
        query.insert("deadline".to_string(), "invalid".to_string());
        let res = client.list(true, query).await;
        assert_eq!(res.status(), StatusCode::BAD_REQUEST);

        let body = res.json::<ErrorResponse>().await.unwrap();
        assert_eq!(body.status, StatusCode::BAD_REQUEST);
    }

    #[test]
    async fn invalid_completed() {
        // test deserialization
        let client = init().await;

        let mut query = HashMap::new();
        query.insert("completed".to_string(), "invalid".to_string());
        let res = client.list(true, query).await;
        assert_eq!(res.status(), StatusCode::BAD_REQUEST);

        let body = res.json::<ErrorResponse>().await.unwrap();
        assert_eq!(body.status, StatusCode::BAD_REQUEST);
    }

    #[test]
    async fn invalid_deleted() {
        // test deserialization
        let client = init().await;

        let mut query = HashMap::new();
        query.insert("deleted".to_string(), "invalid".to_string());
        let res = client.list(true, query).await;
        assert_eq!(res.status(), StatusCode::BAD_REQUEST);

        let body = res.json::<ErrorResponse>().await.unwrap();
        assert_eq!(body.status, StatusCode::BAD_REQUEST);
    }
}
