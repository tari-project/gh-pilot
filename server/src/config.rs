pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub rule_set_path: String,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 8330,
            rule_set_path: "rules.yaml".into(),
        }
    }
}

impl ServerConfig {
    pub fn new(host: &str, port: u16) -> Self {
        Self {
            host: host.to_string(),
            port,
            ..Default::default()
        }
    }
}
