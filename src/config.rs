#[derive(Debug)]
pub struct Config {
    pub exchange: String,
    pub symbol: String,
    pub api_key: Option<String>,
    pub api_secret: Option<String>,
}

impl Config {
    pub fn new(exchange: &str, symbol: &str) -> Self {
        Config {
            exchange: exchange.to_string(),
            symbol: symbol.to_string(),
            api_key: None,
            api_secret: None,
        }
    }
}