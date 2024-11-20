use axum::Router;
use snafu::prelude::*;
use tokio::net::TcpListener;

use crate::middleware;

#[derive(Debug, Clone)]
pub struct HttpServer {
    router: Router,
}

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Failed to bind to address {address}"))]
    BindFailed {
        source: std::io::Error,
        address: String,
    },
}

impl Default for HttpServer {
    #[tracing::instrument]
    fn default() -> Self {
        Self::new()
    }
}

impl HttpServer {
    #[tracing::instrument]
    pub fn new() -> Self {
        // let config = todo!();
        let router = frontend_ui::get_router();
        let router = middleware::add_middlewares(router);

        Self { router }
    }
}

pub trait Server: Clone + Send + Sync + 'static {
    fn start(&self, address: &str) -> impl std::future::Future<Output = Result<(), Error>> + Send;
}

impl Server for HttpServer {
    #[tracing::instrument]
    async fn start(&self, address: &str) -> Result<(), Error> {
        tracing::info!("Starting server");
        let listener = TcpListener::bind(address)
            .await
            .context(BindFailedSnafu { address })?;
        tracing::info!("Listening on {}", address);
        axum::serve(listener, self.router.clone())
            .with_graceful_shutdown(shutdown_signal())
            .await
            .unwrap();
        Ok(())
    }
}

async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let interrupt = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::interrupt())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = interrupt => {},
        _ = terminate => {},
    }
}
