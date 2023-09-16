use serde::Deserialize;

use konfiguration::Konfiguration;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub profile: String,
    pub rust_log: String,
    pub cors_origin: String,
    pub server_port: u16,
    pub exponential_backoff: Vec<u16>,
    pub mail: MailConfig,
    pub postgres: PostgresConfig,
    pub redis: RedisConfig,
}

#[derive(Debug, Deserialize)]
pub struct RedisConfig {
    pub url: String,
    pub max_connections: u32,
    pub min_connections: u32,
    pub connection_acquire_timeout_secs: u64,
}

#[derive(Debug, Deserialize)]
pub struct PostgresConfig {
    pub host: String,
    pub username: String,
    pub password: String,
    pub database: String,
    pub port: u16,
    pub min_connections: u32,
    pub max_connections: u32,
    pub connection_acquire_timeout_secs: u64,
    pub enable_migration: bool,
    pub migrations_dir: String,
}

#[derive(Debug, Deserialize)]
pub struct MailConfig {
    pub from: String,
    pub templates_dir: String,
    pub smtp: SmtpConfig,
}

#[derive(Debug, Deserialize)]
pub struct SmtpConfig {
    pub host: String,
    pub username: String,
    pub password: String,
}

#[test]
fn can_parse_configs() {
    std::env::set_var("EXPONENTIAL_BACKOFF", "[3,4,5]");
    std::env::set_var("PROFILE", "prod");
    std::env::set_var("SMTP_PASSWORD", "password");
    std::env::set_var("DATABASE_PORT", "1111");
    std::env::set_var("DATABASE_USERNAME", "username");

    let config = Konfiguration::from_file("test_files/config.toml")
        .parse::<Config>()
        .unwrap();

    assert_eq!(config.profile, "prod");
    assert_eq!(config.rust_log, "info");
    assert_eq!(config.cors_origin, "*");
    assert_eq!(config.server_port, 8080);
    assert_eq!(config.exponential_backoff, vec![3, 4, 5]);
    assert_eq!(config.mail.templates_dir, "templates/**/*.html");
    assert_eq!(config.postgres.port, 1111);
    assert_eq!(config.postgres.username, "username");
    assert_eq!(config.mail.smtp.password, "password");
}

fn fails_to_parse_configs_if_env_is_not_set_and_default_does_not_exists() {}
