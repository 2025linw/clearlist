use std::{env, net::SocketAddr};

use axum::Router;
use axum_reverse_proxy::ReverseProxy;
use axum_server::tls_rustls::RustlsConfig;

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

    let srv_port = env::var("PROXY_PORT")
        .expect("PROXY_PORT should be set in environment variables!")
        .parse::<u16>()
        .expect("PROXY_PORT should be a valid u16");

    let api_port = env::var("API_PORT")
        .expect("API_PORT should be set in environment variables!")
        .parse::<u16>()
        .expect("API_PORT should be a valid u16");
    let auth_port = env::var("AUTH_PORT")
        .expect("AUTH_PORT should be set in environment variables!")
        .parse::<u16>()
        .expect("AUTH_PORT should be a valid u16");

    // Run proxy
    let api_proxy = ReverseProxy::new("/", &format!("localhost:{api_port}"));
    let auth_proxy = ReverseProxy::new("/", &format!("localhost:{auth_port}"));
    let app_proxy = ReverseProxy::new("/", "localhost:5002");

    let config = RustlsConfig::from_pem_file("certs/todo.local.pem", "certs/todo.local-key.pem")
        .await
        .expect("certs should exist and be loaded");

    let app = if web {
        Router::new()
            .route_service("/api/{*path}", api_proxy)
            .route_service("/api/auth/{*path}", auth_proxy)
            .fallback_service(app_proxy)
    } else {
        Router::new()
            .route_service("/api/{*path}", api_proxy)
            .route_service("/api/auth/{*path}", auth_proxy)
    };

    if web {
        let addr: SocketAddr = format!("0.0.0.0:{srv_port}").parse().unwrap();

        println!("Starting proxy server for web development on port {srv_port}");
        axum_server::bind_rustls(addr, config)
            .serve(app.into_make_service())
            .await
            .unwrap_or_else(|err| {
                eprintln!("unable to start proxy server: {err}");

                std::process::exit(1);
            });
    } else {
        let listener = tokio::net::TcpListener::bind(&format!("0.0.0.0:{srv_port}"))
            .await
            .unwrap();

        println!("Starting proxy server for native app development on port {srv_port}");
        axum::serve(listener, app.into_make_service())
            .await
            .unwrap_or_else(|err| {
                eprintln!("unable to start proxy server: {err}");

                std::process::exit(1);
            });
    }
}
