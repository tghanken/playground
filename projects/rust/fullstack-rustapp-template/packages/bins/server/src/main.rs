use server::server::{HttpServer, Server};

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

fn main() {
    // Sentry can't be initialized in an async main
    let _guard = server::tracing_setup();

    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async_main());
}

#[tracing::instrument]
async fn async_main() {
    let http_server = HttpServer::new();
    http_server.start("0.0.0.0:3031").await.unwrap();
}
