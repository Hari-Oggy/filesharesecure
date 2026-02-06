#[derive(Debug, Clone)]

pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub jwt_maxage: i64,
    pub port: u16,
}

impl Config {
    pub fn init() -> Config {
        dotenv::dotenv().ok();
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
        let jwt_maxage = std::env::var("JWT_MAXAGE")
            .unwrap_or_else(|_| "60".to_string())
            .parse::<i64>()
            .unwrap();
        let port = std::env::var("PORT")
            .unwrap_or_else(|_| "8080".to_string())
            .parse::<u16>()
            .unwrap();

        Config {
            database_url,
            jwt_secret,
            jwt_maxage,
            port,
        }
    }
}
