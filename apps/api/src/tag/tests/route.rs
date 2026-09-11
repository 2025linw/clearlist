mod create;
mod delete;
mod get;
mod list;
mod update;

use std::{collections::HashMap, net::SocketAddr};

use axum::{Router, http::StatusCode, routing::post};
use reqwest::header::COOKIE;
use serde_json::json;

use crate::{
    category::{
        route::create_handler as category_create_handler,
        types::{Category, CategoryID},
    },
    tag::route::create_router,
    tests::server::{
        TEST_COOKIE_KEY, auth::Client as AuthClient, get_test_user_info, start_test_servers,
    },
    types::extract::Response,
    user::route::provision_handler,
};

async fn init_test_setup() -> Client {
    let server_ips = start_test_servers(|| {
        Router::new()
            .route("/internal/users/provision", post(provision_handler))
            .route("/category", post(category_create_handler))
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

    pub async fn create_category(&self) -> CategoryID {
        let url = format!("{}/category", self.url);

        let body = reqwest::Client::new()
            .post(url)
            .header(COOKIE, format!("{}={}", TEST_COOKIE_KEY, self.token))
            .json(&json!({
                "name": "Test Category",
                "positionKey": "a",
            }))
            .send()
            .await
            .unwrap()
            .json::<Response<Category>>()
            .await
            .unwrap();

        body.data.id
    }

    pub async fn list(&self, auth: bool, query: HashMap<String, String>) -> reqwest::Response {
        let mut url = self.url.to_string();
        let query = query
            .into_iter()
            .map(|(key, value)| format!("{key}={value}"))
            .collect::<Vec<String>>()
            .join("&");
        if !query.is_empty() {
            url.push_str(&format!("?{query}"));
        }

        let mut client = reqwest::Client::new().get(url);
        if auth {
            client = client.header(COOKIE, format!("{}={}", TEST_COOKIE_KEY, self.token));
        }

        client.send().await.unwrap()
    }

    pub async fn create(&self, auth: bool, body: serde_json::Value) -> reqwest::Response {
        let mut client = reqwest::Client::new().post(&self.url);
        if auth {
            client = client.header(COOKIE, format!("{}={}", TEST_COOKIE_KEY, self.token));
        }

        client.json(&body).send().await.unwrap()
    }

    pub async fn get(&self, auth: bool, tag_id: String) -> reqwest::Response {
        let mut client = reqwest::Client::new().get(format!("{}/{}", self.url, tag_id));
        if auth {
            client = client.header(COOKIE, format!("{}={}", TEST_COOKIE_KEY, self.token));
        }

        client.send().await.unwrap()
    }

    pub async fn update(
        &self,
        auth: bool,
        tag_id: String,
        body: serde_json::Value,
    ) -> reqwest::Response {
        let mut client = reqwest::Client::new().patch(format!("{}/{}", self.url, tag_id));
        if auth {
            client = client.header(COOKIE, format!("{}={}", TEST_COOKIE_KEY, self.token));
        }

        client.json(&body).send().await.unwrap()
    }

    pub async fn delete(&self, auth: bool, tag_id: String) -> reqwest::Response {
        let mut client = reqwest::Client::new().delete(format!("{}/{}", self.url, tag_id));
        if auth {
            client = client.header(COOKIE, format!("{}={}", TEST_COOKIE_KEY, self.token));
        }

        client.send().await.unwrap()
    }
}
