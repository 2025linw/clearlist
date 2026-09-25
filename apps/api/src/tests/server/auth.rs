mod mock;

use std::net::SocketAddr;

use reqwest::{Error, Response};
use tokio::net::TcpListener;

pub async fn start_server(api_ip: SocketAddr) -> SocketAddr {
    let app_state = mock::MockAuthAppState::init(api_ip);
    let app = mock::build_mock_server().with_state(app_state);

    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    tokio::spawn(async move {
        axum::serve(listener, app)
            .await
            .expect("mock auth server failed to start");
    });

    addr
}

pub struct Client {
    url: String,
}

impl Client {
    pub fn init(ip: SocketAddr) -> Self {
        Self {
            url: format!("http://{ip}/api/auth"),
        }
    }

    pub async fn register(&self, json: serde_json::Value) -> Result<Response, Error> {
        reqwest::Client::new()
            .post(format!("{}/register", self.url))
            .json(&json)
            .send()
            .await
    }
}
