# Konfiguration

![Rust](https://img.shields.io/badge/rust-stable-brightgreen.svg)
[![Docs.rs](https://docs.rs/konfiguration/badge.svg)](https://docs.rs/konfiguration)

Simple Json/Yaml configuration for Rust applications.

## Usage

### Quickstart

Add Konfiguration to your `Cargo.toml`:

```toml
[dependencies]
konfiguration = "0.2.2"
serde = { version = "1.0", features = ["derive"] }
```

Create your configuration file:

In Json:
```json
{
  "server_port": 8080,
  "database": {
    "url": "postgres://localhost:5432/db",
    "username": "postgres",
    "password": "${POSTGRES_PASSWORD_ENV:default}"
  }
}
```

Or in Yaml:
```yaml
server_port: 8080
database:
  url: postgres://localhost:5432/db
  username: postgres
  password: ${POSTGRES_PASSWORD_ENV:default}
```

Load the configuration file:
```rust
use serde::Deserialize;
use konfiguration::Konfiguration;

#[derive(Debug, Deserialize)]
pub struct DatabaseConfig {
    url: String,
    username: String,
    password: String,
}

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    server_port: u16,
    database: DatabaseConfig,
}

fn main() {
    let config = Konfiguration::from_file("filepath/config.json")
        .parse::<AppConfig>()
        .unwrap();
    
    println!("Server port: {}", config.server_port);
    println!("Database url: {}", config.database.url);
    println!("Database username: {}", config.database.username);
}
```

## Contribuiting

Take a look at our [contributing guide](https://github.com/FaveroFerreira/konfiguration/blob/main/CONTRIBUTING.md) if you wish to contribute.
