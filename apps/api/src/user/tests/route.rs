mod get;
mod update;

use std::net::SocketAddr;

use axum::{Router, http::StatusCode, routing::post};
use reqwest::{Response, header::COOKIE};

use crate::{
    tests::server::{
        TEST_COOKIE_KEY, auth::Client as AuthClient, get_test_user_info, start_test_servers,
    },
    user::route::{create_router, provision_handler},
};

async fn init_test_setup() -> Client {
    let server_ips = start_test_servers(|| {
        Router::new()
            .route("/internal/users/provision", post(provision_handler))
            .merge(create_router())
    })
    .await;

    // register user
    let res = AuthClient::init(server_ips.auth_addr)
        .register(get_test_user_info().to_owned())
        .await
        .unwrap();
    if res.status() != StatusCode::OK {
        panic!("failed to create user in auth");
    }
    let token = res.text().await.unwrap();

    Client::init(server_ips.api_addr, token)
}

struct Client {
    url: String,

    token: String,
}

impl Client {
    pub fn init(ip: SocketAddr, token: String) -> Self {
        Self {
            url: format!("http://{ip}"),
            token,
        }
    }

    pub async fn get(&self, auth: bool) -> Response {
        let mut client = reqwest::Client::new().get(format!("{}/me", self.url));
        if auth {
            client = client.header(COOKIE, format!("{}={}", TEST_COOKIE_KEY, self.token))
        }

        client.send().await.unwrap()
    }

    pub async fn update(&self, auth: bool, body: serde_json::Value) -> Response {
        let mut client = reqwest::Client::new().patch(format!("{}/me", self.url));
        if auth {
            client = client.header(COOKIE, format!("{}={}", TEST_COOKIE_KEY, self.token))
        }

        client.json(&body).send().await.unwrap()
    }
}
