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

        let res = client.provision(true).await;
        assert_eq!(res.status(), StatusCode::CREATED);
    }

    #[test]
    async fn return_body() {
        let client = init().await;

        let body = client
            .provision(true)
            .await
            .json::<Response<User>>()
            .await
            .unwrap();
        assert_eq!(body.status, StatusCode::CREATED);
    }
}

mod error {
    use super::*;
    use tokio::test;

    #[test]
    async fn not_authenticated() {
        let client = init().await;

        let res = client.provision(false).await;
        assert_eq!(res.status(), StatusCode::UNAUTHORIZED);

        let body = res.json::<ErrorResponse>().await.unwrap();
        assert_eq!(body.status, StatusCode::UNAUTHORIZED);
    }

    #[test]
    async fn exists() {
        let client = init().await;
        client.provision(true).await;

        let res = client.provision(true).await;
        assert_eq!(res.status(), StatusCode::CONFLICT);

        let body = res.json::<ErrorResponse>().await.unwrap();
        assert_eq!(body.status, StatusCode::CONFLICT);
    }
}
