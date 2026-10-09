mod config;

use std::net::SocketAddr;

use axum::{
    Router,
    body::Body,
    http::{Request, header},
};
use axum_reverse_proxy::ReverseProxy;
use axum_server::tls_rustls::RustlsConfig;
use config::Config;
use tower::ServiceBuilder;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let Config {
        srv_port,
        api_port,
        auth_port,
        app_port,
        cert_path,
        cert_key_path,
    } = Config::new_app_from_env();

    let api_proxy = ReverseProxy::new("/", &format!("http://127.0.0.1:{api_port}"));
    let auth_proxy = ReverseProxy::new("/", &format!("http://127.0.0.1:{auth_port}"));

    let app_proxy = ReverseProxy::new("/", &format!("http://127.0.0.1:{app_port}"));

    let ws_compatible_app_proxy = ServiceBuilder::new()
        .map_request(|mut request: Request<Body>| {
            let is_websocket = request
                .headers()
                .get(header::UPGRADE)
                .and_then(|value| value.to_str().ok())
                .is_some_and(|value| value.eq_ignore_ascii_case("websocket"));

            if !is_websocket && request.headers().contains_key(header::ORIGIN) {
                request.headers_mut().insert(
                    header::ORIGIN,
                    header::HeaderValue::from_static("http://localhost:8081"),
                );
            }

            request
        })
        .service(app_proxy);

    let app = Router::new()
        .route_service("/api/{*path}", api_proxy)
        .route_service("/api/auth/{*path}", auth_proxy)
        .fallback_service(ws_compatible_app_proxy);

    let addr: SocketAddr = format!("0.0.0.0:{srv_port}").parse().unwrap();
    let config = RustlsConfig::from_pem_file(cert_path, cert_key_path)
        .await
        .expect("Tailscale certs should exist and be loaded");

    println!("Starting proxy server for development on port {srv_port}");
    println!("Connect on https://ts.net:{srv_port}");
    axum_server::bind_rustls(addr, config)
        .serve(app.into_make_service())
        .await
        .unwrap_or_else(|err| {
            eprintln!("unable to start proxy server: {err}");

            std::process::exit(1);
        });
}
