use std::env;

pub struct Config {
    pub srv_port: u16,

    pub api_port: u16,
    pub auth_port: u16,
    pub app_port: u16,

    pub cert_path: String,
    pub cert_key_path: String,
}

impl Config {
    pub fn new(
        srv_port: u16,
        api_port: u16,
        auth_port: u16,
        app_port: u16,
        cert_path: String,
        cert_key_path: String,
    ) -> Self {
        assert_ne!(
            srv_port, 8081,
            "PROXY_PORT must not be 8081 as it will collide with Expo default port"
        );

        Self {
            srv_port,

            api_port,
            auth_port,
            app_port,

            cert_path,
            cert_key_path,
        }
    }

    pub fn new_app_from_env() -> Self {
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

        let app_port = env::var("APP_PORT")
            .expect("APP_PORT should be set in environment variables!")
            .parse::<u16>()
            .expect("APP_PORT should be a valid u16");

        let cert_path = std::env::var("CERT_PATH").expect("CERT_PATH must be set");
        let cert_key_path = std::env::var("CERT_KEY_PATH").expect("CERT_KEY_PATH must be set");

        Self::new(
            srv_port,
            api_port,
            auth_port,
            app_port,
            cert_path,
            cert_key_path,
        )
    }
}
