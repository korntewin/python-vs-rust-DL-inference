use rust_api_server::api::apiserver::run_api_server;
use rust_api_server::config::Config;

#[ntex::main]
async fn main() -> std::io::Result<()> {
    let app_config = Config::new();

    run_api_server(app_config).await
}
