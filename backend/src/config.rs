use std::net::SocketAddr;

/// Runtime configuration, read from environment variables.
#[derive(Debug, Clone)]
pub struct Config {
    /// Address the HTTP server binds to. Env: `HUDDLE_ADDR`, default `0.0.0.0:8080`.
    pub addr: SocketAddr,
}

impl Config {
    pub fn from_env() -> Result<Self, String> {
        let addr = match std::env::var("HUDDLE_ADDR") {
            Ok(raw) => raw
                .parse()
                .map_err(|e| format!("invalid HUDDLE_ADDR {raw:?}: {e}"))?,
            Err(_) => SocketAddr::from(([0, 0, 0, 0], 8080)),
        };
        Ok(Self { addr })
    }
}
