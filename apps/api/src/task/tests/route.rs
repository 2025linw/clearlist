mod create;
mod delete;
mod get;
mod list;
mod restore;
mod update;

mod complete;
mod reopen;

mod add_tag;
mod list_tags;
mod remove_tag;
mod set_tags;

use std::{collections::HashMap, net::SocketAddr};

use axum::{Router, http::StatusCode, routing::post};
use reqwest::header::COOKIE;
use serde_json::json;

use crate::{
    tag::{
        route::create_handler as tag_create_handler,
        types::{Tag, TagID},
    },
    task::route::create_router,
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
            .route("/tag", post(tag_create_handler))
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

    pub async fn create_tag(&self) -> TagID {
        let url = format!("{}/tag", self.url);

        let body = reqwest::Client::new()
            .post(url)
            .header(COOKIE, format!("{}={}", TEST_COOKIE_KEY, self.token))
            .json(&json!({
                "label": "Test Task",
                "positionKey": "a",
            }))
            .send()
            .await
            .unwrap()
            .json::<Response<Tag>>()
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

    pub async fn get(&self, auth: bool, task_id: String) -> reqwest::Response {
        let mut client = reqwest::Client::new().get(format!("{}/{task_id}", self.url));
        if auth {
            client = client.header(COOKIE, format!("{}={}", TEST_COOKIE_KEY, self.token));
        }

        client.send().await.unwrap()
    }

    pub async fn update(
        &self,
        auth: bool,
        task_id: String,
        body: serde_json::Value,
    ) -> reqwest::Response {
        let mut client = reqwest::Client::new().patch(format!("{}/{task_id}", self.url));
        if auth {
            client = client.header(COOKIE, format!("{}={}", TEST_COOKIE_KEY, self.token));
        }

        client.json(&body).send().await.unwrap()
    }

    pub async fn delete(&self, auth: bool, task_id: String) -> reqwest::Response {
        let mut client = reqwest::Client::new().delete(format!("{}/{task_id}", self.url));
        if auth {
            client = client.header(COOKIE, format!("{}={}", TEST_COOKIE_KEY, self.token));
        }

        client.send().await.unwrap()
    }

    pub async fn restore(&self, auth: bool, task_id: String) -> reqwest::Response {
        let mut client = reqwest::Client::new().post(format!("{}/{task_id}/restore", self.url));
        if auth {
            client = client.header(COOKIE, format!("{}={}", TEST_COOKIE_KEY, self.token));
        }

        client.send().await.unwrap()
    }

    pub async fn complete(&self, auth: bool, task_id: String) -> reqwest::Response {
        let mut client = reqwest::Client::new().post(format!("{}/{task_id}/complete", self.url));
        if auth {
            client = client.header(COOKIE, format!("{}={}", TEST_COOKIE_KEY, self.token));
        }

        client.send().await.unwrap()
    }

    pub async fn reopen(&self, auth: bool, task_id: String) -> reqwest::Response {
        let mut client = reqwest::Client::new().post(format!("{}/{task_id}/reopen", self.url));
        if auth {
            client = client.header(COOKIE, format!("{}={}", TEST_COOKIE_KEY, self.token));
        }

        client.send().await.unwrap()
    }

    pub async fn list_tags(&self, auth: bool, task_id: String) -> reqwest::Response {
        let mut client = reqwest::Client::new().get(format!("{}/{task_id}/tags", self.url));
        if auth {
            client = client.header(COOKIE, format!("{}={}", TEST_COOKIE_KEY, self.token));
        }

        client.send().await.unwrap()
    }

    pub async fn add_tag(&self, auth: bool, task_id: String, tag_id: String) -> reqwest::Response {
        let mut client =
            reqwest::Client::new().patch(format!("{}/{task_id}/tags/{tag_id}", self.url));
        if auth {
            client = client.header(COOKIE, format!("{}={}", TEST_COOKIE_KEY, self.token));
        }

        client.send().await.unwrap()
    }

    pub async fn remove_tag(
        &self,
        auth: bool,
        task_id: String,
        tag_id: String,
    ) -> reqwest::Response {
        let mut client =
            reqwest::Client::new().delete(format!("{}/{task_id}/tags/{tag_id}", self.url));
        if auth {
            client = client.header(COOKIE, format!("{}={}", TEST_COOKIE_KEY, self.token));
        }

        client.send().await.unwrap()
    }

    pub async fn set_tags(
        &self,
        auth: bool,
        task_id: String,
        tag_ids: Vec<String>,
    ) -> reqwest::Response {
        let mut client = reqwest::Client::new().put(format!("{}/{task_id}/tags", self.url));
        if auth {
            client = client.header(COOKIE, format!("{}={}", TEST_COOKIE_KEY, self.token));
        }

        client.json(&json!(tag_ids)).send().await.unwrap()
    }
}
