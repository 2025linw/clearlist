mod config;

use std::{env, net::SocketAddr};

use axum::Router;
use axum_reverse_proxy::ReverseProxy;
use axum_server::tls_rustls::RustlsConfig;

use config::Config;

use crate::config::{AppConfig, WebConfig};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let mut web = false;

    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        if args.len() > 2 {
            eprintln!("{} expected 0 or 1 arguments: <function>", args[0]);

            std::process::exit(1);
        }

        match args[1].as_ref() {
            "native" => (),
            "web" => web = true,
            _ => (),
        }
    }

    let port: u16;
    let app = if web {
        let WebConfig {
            srv_port,
            api_port,
            auth_port,
            web_port,
        } = Config::new_web_from_env().web_config();

        port = srv_port;

        let api_proxy = ReverseProxy::new("/", &format!("localhost:{api_port}"));
        let auth_proxy = ReverseProxy::new("/", &format!("localhost:{auth_port}"));
        let app_proxy = ReverseProxy::new("/", &format!("localhost:{web_port}"));

        Router::new()
            .route_service("/api/{*path}", api_proxy)
            .route_service("/api/auth/{*path}", auth_proxy)
            .fallback_service(app_proxy)
    } else {
        let AppConfig {
            srv_port,
            api_port,
            auth_port,
        } = Config::new_app_from_env().app_config();

        port = srv_port;

        let api_proxy = ReverseProxy::new("/", &format!("localhost:{api_port}"));
        let auth_proxy = ReverseProxy::new("/", &format!("localhost:{auth_port}"));

        Router::new()
            .route_service("/api/{*path}", api_proxy)
            .route_service("/api/auth/{*path}", auth_proxy)
    };

    if web {
        let addr: SocketAddr = format!("0.0.0.0:{port}").parse().unwrap();
        let config =
            RustlsConfig::from_pem_file("certs/todo.localhost.pem", "certs/todo.localhost-key.pem")
                .await
                .expect("certs should exist and be loaded");

        println!("Starting proxy server for web development on port {port}");
        axum_server::bind_rustls(addr, config)
            .serve(app.into_make_service())
            .await
            .unwrap_or_else(|err| {
                eprintln!("unable to start proxy server: {err}");

                std::process::exit(1);
            });
    } else {
        let listener = tokio::net::TcpListener::bind(&format!("0.0.0.0:{port}"))
            .await
            .unwrap();

        println!("Starting proxy server for native app development on port {port}");
        axum::serve(listener, app.into_make_service())
            .await
            .unwrap_or_else(|err| {
                eprintln!("unable to start proxy server: {err}");

                std::process::exit(1);
            });
    }
}
