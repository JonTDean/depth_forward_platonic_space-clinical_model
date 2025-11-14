use std::net::{IpAddr, SocketAddr};

use axum::Router;
use log::{info, warn};
use thiserror::Error;
use tokio::net::TcpListener;

/// Runtime configuration for the HTTP server.
#[derive(Debug, Clone)]
pub struct ApiServerConfig {
    pub host: String,
    pub port: u16,
}

impl Default for ApiServerConfig {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 8080,
        }
    }
}

impl ApiServerConfig {
    fn socket_addr(&self) -> Result<SocketAddr, ServerError> {
        let ip: IpAddr = self
            .host
            .parse()
            .map_err(|source| ServerError::InvalidHost {
                host: self.host.clone(),
                source,
            })?;
        Ok(SocketAddr::new(ip, self.port))
    }
}

#[derive(Debug, Error)]
pub enum ServerError {
    #[error("invalid bind host '{host}': {source}")]
    InvalidHost {
        host: String,
        #[source]
        source: std::net::AddrParseError,
    },
    #[error("failed to bind server at {addr}: {source}")]
    Bind {
        addr: SocketAddr,
        #[source]
        source: std::io::Error,
    },
    #[error("server error: {0}")]
    Serve(#[source] std::io::Error),
}

/// Start the HTTP server using the provided configuration.
///
/// The function builds a router (currently empty) and blocks until Ctrl+C or
/// another shutdown signal is received.
pub async fn run(config: ApiServerConfig) -> Result<(), ServerError> {
    let addr = config.socket_addr()?;
    info!(target: "dfps_api", "starting web backend on {addr}");
    let listener = TcpListener::bind(addr)
        .await
        .map_err(|source| ServerError::Bind { addr, source })?;

    let router = build_router();

    axum::serve(listener, router.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .map_err(ServerError::Serve)?;

    info!(target: "dfps_api", "server stopped");
    Ok(())
}

fn build_router() -> Router {
    Router::new()
}

fn shutdown_signal() -> impl std::future::Future<Output = ()> {
    async {
        match tokio::signal::ctrl_c().await {
            Ok(()) => info!(target: "dfps_api", "received shutdown signal"),
            Err(err) => warn!(target: "dfps_api", "failed waiting for ctrl_c: {err}"),
        }
    }
}
