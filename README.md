# Konfiguration

![Rust](https://img.shields.io/badge/rust-stable-brightgreen.svg)
[![Docs.rs](https://docs.rs/konfiguration/badge.svg)](https://docs.rs/konfiguration)

TOML configuration parser for Rust.

## Usage

### Quickstart

Add Konfiguration to your `Cargo.toml`:

```toml
[dependencies]
konfiguration = "1.0.1"
serde = { version = "1.0", features = ["derive"] }
```

Create your configuration file:

```toml
profile = { env = "PROFILE", default = "local" }
rust_log = "info"
server_port = { env = "PORT", default = 8080 }
cors_origin = { env = "CORS_ALLOWED_ORIGINS", default = "*" }
exponential_backoff = { env = "EXPONENTIAL_BACKOFF", default = [1, 2, 3] }

[postgres]
username = { env = "DATABASE_USERNAME", default = "root" }
password = { env = "DATABASE_PASSWORD", default = "root" }
host = { env = "DATABASE_HOST", default = "localhost" }
port = { env = "DATABASE_PORT", default = 5432 }
database = { env = "DATABASE_NAME", default = "postgres" }
min_connections = { env = "DATABASE_MIN_CONNECTIONS", default = 3 }
max_connections = { env = "DATABASE_MAX_CONNECTIONS", default = 10 }
connection_acquire_timeout_secs = { env = "DATABASE_CONNECTION_ACQUIRE_TIMEOUT_SECONDS", default = 10 }
enable_migration = { env = "DATABASE_ENABLE_MIGRATIONS", default = false }
migrations_dir = "./migrations"
```

Load the configuration file:
```rust,no_compile
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

fn main() {
    let config = Konfiguration::from_file("filepath/config.toml")
        .parse::<Config>()
        .unwrap();
    
    println!("{:?}", config);
}
```

## Contribuiting

Take a look at our [contributing guide](https://github.com/FaveroFerreira/konfiguration/blob/main/CONTRIBUTING.md) if you wish to contribute.
