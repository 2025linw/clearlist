use std::env;

pub struct Config {
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

    pub fn new_app_from_env() -> Self {
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

    pub fn new_web_from_env() -> Self {
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

    pub fn web_config(&self) -> WebConfig {
        WebConfig {
            srv_port: self.srv_port,
            api_port: self.api_port,
            auth_port: self.auth_port,
            web_port: self.web_port.unwrap(),
        }
    }

    pub fn app_config(&self) -> AppConfig {
        AppConfig {
            srv_port: self.srv_port,
            api_port: self.api_port,
            auth_port: self.auth_port,
        }
    }
}

pub struct WebConfig {
    pub srv_port: u16,

    pub api_port: u16,
    pub auth_port: u16,
    pub web_port: u16,
}

pub struct AppConfig {
    pub srv_port: u16,

    pub api_port: u16,
    pub auth_port: u16,
}
