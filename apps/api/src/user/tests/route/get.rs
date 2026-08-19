use axum::http::StatusCode;

use crate::{
    types::extract::{ErrorResponse, Response},
    user::types::User,
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

        let res = client.get(true).await;
        assert_eq!(
            res.status(),
            StatusCode::OK,
            "{}",
            res.text().await.unwrap()
        );
    }

    #[test]
    async fn return_body() {
        let client = init().await;

        let body = client
            .get(true)
            .await
            .json::<Response<User>>()
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

        let res = client.get(false).await;
        assert_eq!(res.status(), StatusCode::UNAUTHORIZED);

        let body = res.json::<ErrorResponse>().await.unwrap();
        assert_eq!(body.status, StatusCode::UNAUTHORIZED);
    }

    #[test]
    async fn not_exists() {
        let client = init().await;

        let res = client.get(true).await;
        assert_eq!(res.status(), StatusCode::NOT_FOUND);

        let body = res.json::<ErrorResponse>().await.unwrap();
        assert_eq!(body.status, StatusCode::NOT_FOUND);
    }
}
