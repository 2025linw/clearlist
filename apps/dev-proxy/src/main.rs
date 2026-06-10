use std::{env, net::SocketAddr};

use axum::Router;
use axum_reverse_proxy::ReverseProxy;
use axum_server::tls_rustls::RustlsConfig;

struct Config {
    srv_port: u16,

    api_port: u16,
    auth_port: u16,
    web_port: Option<u16>,
}

impl Config {
    pub fn new_app(srv_port: u16, api_port: u16, auth_port: u16) -> Self {
        assert_ne!(
            srv_port, 8081,
            "APP_PROXY_PORT must not be 8081 as it will collide with Expo default port"
        );

        Self {
            srv_port,

            api_port,
            auth_port,
            web_port: None,
        }
    }

    pub fn from_env_app() -> Self {
        let srv_port = env::var("APP_PROXY_PORT")
            .expect("APP_PROXY_PORT should be set in environment variables!")
            .parse::<u16>()
            .expect("APP_PROXY_PORT should be a valid u16");

        let api_port = env::var("API_PORT")
            .expect("API_PORT should be set in environment variables!")
            .parse::<u16>()
            .expect("API_PORT should be a valid u16");
        let auth_port = env::var("AUTH_PORT")
            .expect("AUTH_PORT should be set in environment variables!")
            .parse::<u16>()
            .expect("AUTH_PORT should be a valid u16");

        Self::new_app(srv_port, api_port, auth_port)
    }

    pub fn new_web(srv_port: u16, api_port: u16, auth_port: u16, web_port: u16) -> Self {
        Self {
            srv_port,
            api_port,
            auth_port,
            web_port: Some(web_port),
        }
    }

    pub fn from_env_web() -> Self {
        let srv_port = env::var("WEB_PROXY_PORT")
            .expect("WEB_PROXY_PORT should be set in environment variables!")
            .parse::<u16>()
            .expect("WEB_PROXY_PORT should be a valid u16");

        let api_port = env::var("API_PORT")
            .expect("API_PORT should be set in environment variables!")
            .parse::<u16>()
            .expect("API_PORT should be a valid u16");
        let auth_port = env::var("AUTH_PORT")
            .expect("AUTH_PORT should be set in environment variables!")
            .parse::<u16>()
            .expect("AUTH_PORT should be a valid u16");
        let web_port = env::var("WEBAPP_PORT")
            .expect("WEBAPP_PORT should be set in environment variables!")
            .parse::<u16>()
            .expect("WEBAPP_PORT should be a valid u16");

        Self::new_web(srv_port, api_port, auth_port, web_port)
    }
}

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
        let Config {
            srv_port,
            api_port,
            auth_port,
            web_port,
        } = Config::from_env_web();
        web_port.expect("WEBAPP_PORT should have been checked in Config::web_from_env");
        let web_port = web_port.unwrap();

        port = srv_port;

        let api_proxy = ReverseProxy::new("/", &format!("localhost:{api_port}"));
        let auth_proxy = ReverseProxy::new("/", &format!("localhost:{auth_port}"));
        let app_proxy = ReverseProxy::new("/", &format!("localhost:{web_port}"));

        Router::new()
            .route_service("/api/{*path}", api_proxy)
            .route_service("/api/auth/{*path}", auth_proxy)
            .fallback_service(app_proxy)
    } else {
        let Config {
            srv_port,
            api_port,
            auth_port,
            ..
        } = Config::from_env_app();

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
