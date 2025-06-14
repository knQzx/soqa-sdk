use crate::models::OrderBookL1;
use crate::error::SoqaError;
use tokio_tungstenite::{connect_async, tungstenite::Message};
use serde_json::Value;
use std::time::SystemTime;
use futures_util::stream::StreamExt;

pub struct BinanceClient {
    config: crate::config::Config,
}

impl BinanceClient {
    pub fn new(config: crate::config::Config) -> Self {
        BinanceClient { config }
    }

    pub async fn subscribe_l1(&self, callback: impl Fn(OrderBookL1) + Send + 'static) -> Result<(), SoqaError> {
        let url = format!("wss://stream.binance.com:9443/ws/{}@ticker", self.config.symbol.to_lowercase());
        let (ws_stream, _) = connect_async(&url).await?;
        let (mut _write, mut read) = ws_stream.split();

        tokio::spawn(async move {
            while let Some(msg) = read.next().await {
                if let Ok(Message::Text(text)) = msg {
                    if let Ok(data) = serde_json::from_str::<Value>(&text) {
                        let order_book = OrderBookL1 {
                            exchange: "binance".to_string(),
                            symbol: data["s"].as_str().unwrap_or("").to_string(),
                            bid: data["b"].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                            bid_volume: data["B"].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                            ask: data["a"].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                            ask_volume: data["A"].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                            timestamp: SystemTime::now(),
                        };
                        callback(order_book);
                    }
                }
            }
        });

        Ok(())
    }
}