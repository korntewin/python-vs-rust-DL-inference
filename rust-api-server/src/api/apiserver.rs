use crate::api::routers::feature::process_feature;
use crate::config::Config;
use crate::services::adapters::GetFeatureFromSQLAdapter;
use crate::services::model_service::ModelService;
use crate::services::feature_service::FeatureService;
use ntex::web;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppData {
    pub config: Config,
    pub feature_service: FeatureService,
}

async fn health_check() -> web::HttpResponse {
    web::HttpResponse::Ok().body("OK")
}

pub async fn run_api_server(config: Config) -> std::io::Result<()> {
    env_logger::init();
    let feature_adapter = GetFeatureFromSQLAdapter::new(config.clone()).await.unwrap();
    let model_service = ModelService::new(&config.model_path).unwrap();
    let recommendation_service = FeatureService::new(Arc::new(feature_adapter), model_service);
    let app_data = AppData {
        config: config.clone(),
        feature_service: recommendation_service,
    };
    web::HttpServer::new(move || {
        web::App::new()
            .service(web::scope("/feature").service(process_feature))
            .service(
                web::scope("/")
                    .route("/health", web::get().to(health_check))
                    .route("/healthz", web::get().to(health_check)),
            )
            .state(app_data.clone())
    })
    .bind(format!("{}:{}", config.host, config.port))?
    .workers(config.workers)
    .run()
    .await
}
