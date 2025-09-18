use dotenvy::dotenv;

#[derive(Clone)]
pub struct Config {
    pub port: u16,
    pub host: String,
    pub db_url: String,
    pub model_path: String,
    pub workers: usize,
}

impl Config {
    pub fn new() -> Self {
        dotenv().ok();
        let port = std::env::var("PORT").unwrap_or("8080".to_string());
        let host = std::env::var("HOST").unwrap_or("0.0.0.0".to_string());
        let db_url = std::env::var("DB_URL")
            .unwrap_or("postgres://postgres:password@localhost:5432/postgres".to_string());
        let model_path =
            std::env::var("MODEL_PATH").unwrap_or("data/model.safetensors".to_string());
        let workers = std::env::var("WORKERS")
            .unwrap_or("16".to_string())
            .parse::<usize>()
            .unwrap();
        Self {
            port: port.parse().unwrap(),
            host,
            db_url,
            model_path,
            workers,
        }
    }
}
