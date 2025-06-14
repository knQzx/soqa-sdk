use crate::models::{OrderBookL1};
use crate::error::SoqaError;
use tokio_tungstenite::{connect_async, tungstenite::Message};
use serde_json::Value;
use std::time::SystemTime;
use futures_util::stream::StreamExt;
use futures_util::SinkExt;

pub struct KrakenClient {
    config: crate::config::Config,
}

impl KrakenClient {
    pub fn new(config: crate::config::Config) -> Self {
        KrakenClient { config }
    }

    fn convert_symbol(&self, symbol: &str) -> String {
        match symbol {
            "BTCUSD" => "XBT/USD".to_string(),
            "BTCUSDT" => "XBT/USDT".to_string(),
            "ETHUSD" => "ETH/USD".to_string(),
            "ETHUSDT" => "ETH/USDT".to_string(),
            _ => symbol.to_string(),
        }
    }

    pub async fn subscribe_l1(&self, callback: impl Fn(OrderBookL1) + Send + 'static) -> Result<(), SoqaError> {
        let url = "wss://ws.kraken.com";
        let (ws_stream, _) = connect_async(url).await?;
        let (mut write, mut read) = ws_stream.split();

        let symbol = self.convert_symbol(&self.config.symbol);
        let original_symbol = self.config.symbol.clone();
        println!("Subscribing to Kraken with symbol: {}", symbol);
        
        let subscribe_msg = format!(
            r#"{{"event":"subscribe","pair":["{}"],"subscription":{{"name":"ticker"}}}}"#,
            symbol
        );
        println!("Sending subscription message: {}", subscribe_msg);
        write.send(Message::Text(subscribe_msg)).await?;

        tokio::spawn(async move {
            let mut last_heartbeat = SystemTime::now();
            while let Some(msg) = read.next().await {
                match msg {
                    Ok(Message::Text(text)) => {
                        if let Ok(data) = serde_json::from_str::<Value>(&text) {
                            // Handle heartbeat silently
                            if data.get("event").and_then(|e| e.as_str()) == Some("heartbeat") {
                                let now = SystemTime::now();
                                if now.duration_since(last_heartbeat).unwrap().as_secs() >= 30 {
                                    println!("Kraken connection is alive");
                                    last_heartbeat = now;
                                }
                                continue;
                            }

                            // Handle subscription confirmation
                            if data.get("event").and_then(|e| e.as_str()) == Some("subscribe") {
                                println!("Successfully subscribed to Kraken WebSocket");
                                continue;
                            }

                            // Handle error messages
                            if data.get("event").and_then(|e| e.as_str()) == Some("error") {
                                if let Some(error_msg) = data.get("errorMessage").and_then(|e| e.as_str()) {
                                    eprintln!("Kraken WebSocket error: {}", error_msg);
                                }
                                continue;
                            }

                            // Handle ticker data
                            if data.as_array().is_some() {
                                let ticker = &data[1];
                                let order_book = OrderBookL1 {
                                    exchange: "kraken".to_string(),
                                    symbol: original_symbol.clone(),
                                    bid: ticker["b"][0].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                                    bid_volume: ticker["b"][1].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                                    ask: ticker["a"][0].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                                    ask_volume: ticker["a"][1].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                                    timestamp: SystemTime::now(),
                                };
                                callback(order_book);
                            }
                        }
                    }
                    Ok(Message::Close(_)) => {
                        println!("Kraken WebSocket connection closed");
                        break;
                    }
                    Err(e) => {
                        eprintln!("Kraken WebSocket error: {}", e);
                        break;
                    }
                    _ => {}
                }
            }
        });

        Ok(())
    }
}