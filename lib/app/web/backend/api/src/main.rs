use dfps_api::{ApiServerConfig, init_logging, run};

#[tokio::main]
async fn main() {
    init_logging();
    if let Err(err) = run(ApiServerConfig::default()).await {
        eprintln!("dfps_api server error: {err}");
        std::process::exit(1);
    }
}
