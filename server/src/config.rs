use std::env;

use log::error;

const DEFAULT_GH_PILOT_HOST: &str = "127.0.0.1";
const DEFAULT_GH_PILOT_PORT: u16 = 8330;
const DEFAULT_GH_PILOT_RULESET_PATH: &str = "rules.yaml";

pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub rule_set_path: String,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host: DEFAULT_GH_PILOT_HOST.to_string(),
            port: DEFAULT_GH_PILOT_PORT,
            rule_set_path: DEFAULT_GH_PILOT_RULESET_PATH.to_string(),
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

    pub fn from_env_or_default() -> Self {
        let host = env::var("GH_PILOT_HOST")
            .ok()
            .unwrap_or_else(|| DEFAULT_GH_PILOT_HOST.into());
        let port = env::var("GH_PILOT_PORT")
            .map(|s| {
                s.parse::<u16>().unwrap_or_else(|e| {
                    error!(
                        "{s} is not a valid port for GH_PILOT_PORT. {e} Using the default, {DEFAULT_GH_PILOT_PORT}, \
                         instead."
                    );
                    DEFAULT_GH_PILOT_PORT
                })
            })
            .ok()
            .unwrap_or(DEFAULT_GH_PILOT_PORT);
        let rule_set_path = env::var("GH_PILOT_RULESET_PATH")
            .ok()
            .unwrap_or_else(|| DEFAULT_GH_PILOT_RULESET_PATH.into());
        Self {
            host,
            port,
            rule_set_path,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::config::{ServerConfig, DEFAULT_GH_PILOT_HOST, DEFAULT_GH_PILOT_PORT, DEFAULT_GH_PILOT_RULESET_PATH};

    fn clear_env() {
        std::env::remove_var("GH_PILOT_HOST");
        std::env::remove_var("GH_PILOT_PORT");
        std::env::remove_var("GH_PILOT_RULESET_PATH");
    }

    fn default_config() {
        let config = ServerConfig::default();
        assert_eq!(config.host, DEFAULT_GH_PILOT_HOST);
        assert_eq!(config.port, DEFAULT_GH_PILOT_PORT);
        assert_eq!(config.rule_set_path, DEFAULT_GH_PILOT_RULESET_PATH);
    }

    fn from_env_to_default() {
        let config = ServerConfig::from_env_or_default();
        assert_eq!(config.host, DEFAULT_GH_PILOT_HOST);
        assert_eq!(config.port, DEFAULT_GH_PILOT_PORT);
        assert_eq!(config.rule_set_path, DEFAULT_GH_PILOT_RULESET_PATH);
    }

    fn host_from_env() {
        std::env::set_var("GH_PILOT_HOST", "foo");
        let config = ServerConfig::from_env_or_default();
        assert_eq!(config.host, "foo");
        assert_eq!(config.port, DEFAULT_GH_PILOT_PORT);
        assert_eq!(config.rule_set_path, DEFAULT_GH_PILOT_RULESET_PATH);
    }

    fn port_from_env() {
        std::env::set_var("GH_PILOT_PORT", "1234");
        let config = ServerConfig::from_env_or_default();
        assert_eq!(config.host, DEFAULT_GH_PILOT_HOST);
        assert_eq!(config.port, 1234);
        assert_eq!(config.rule_set_path, DEFAULT_GH_PILOT_RULESET_PATH);
    }

    fn invalid_port_from_env() {
        std::env::set_var("GH_PILOT_PORT", "-1");
        let config = ServerConfig::from_env_or_default();
        assert_eq!(config.host, DEFAULT_GH_PILOT_HOST);
        assert_eq!(config.port, DEFAULT_GH_PILOT_PORT);
        assert_eq!(config.rule_set_path, DEFAULT_GH_PILOT_RULESET_PATH);
    }

    fn rule_set_from_env() {
        std::env::set_var("GH_PILOT_RULESET_PATH", "my_house_my_rules.yaml");
        let config = ServerConfig::from_env_or_default();
        assert_eq!(config.host, DEFAULT_GH_PILOT_HOST);
        assert_eq!(config.port, DEFAULT_GH_PILOT_PORT);
        assert_eq!(config.rule_set_path, "my_house_my_rules.yaml");
    }

    // Calls the individual test functions in sequence. If we don't do this, tests running the parallel threads
    // result in flaky tests since the environment is global.
    #[test]
    fn envar_configurations() {
        clear_env();
        default_config();
        from_env_to_default();
        host_from_env();
        clear_env();
        port_from_env();
        invalid_port_from_env();
        clear_env();
        rule_set_from_env();
    }
}
