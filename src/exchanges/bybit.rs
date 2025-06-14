use crate::models::{OrderBookL1};
use crate::error::SoqaError;
use tokio_tungstenite::{connect_async, tungstenite::Message};
use serde_json::Value;
use std::time::SystemTime;
use futures_util::stream::StreamExt;
use futures_util::SinkExt;

pub struct BybitClient {
    config: crate::config::Config,
}

impl BybitClient {
    pub fn new(config: crate::config::Config) -> Self {
        BybitClient { config }
    }

    pub async fn subscribe_l1(&self, callback: impl Fn(OrderBookL1) + Send + 'static) -> Result<(), SoqaError> {
        let url = "wss://stream.bybit.com/v5/public/spot";
        let (ws_stream, _) = connect_async(url).await?;
        let (mut write, mut read) = ws_stream.split();

        let symbol = self.config.symbol.clone();
        let subscribe_msg = format!(
            r#"{{"op":"subscribe","args":["orderbook.1.{}"]}}"#,
            symbol
        );
        write.send(Message::Text(subscribe_msg)).await?;

        tokio::spawn(async move {
            while let Some(msg) = read.next().await {
                match msg {
                    Ok(Message::Text(text)) => {
                        if let Ok(data) = serde_json::from_str::<Value>(&text) {
                            // Handle subscription confirmation
                            if data.get("success").and_then(|s| s.as_bool()).unwrap_or(false) {
                                println!("Successfully subscribed to Bybit WebSocket");
                                continue;
                            }

                            // Handle error messages
                            if let Some(error) = data.get("ret_msg").and_then(|e| e.as_str()) {
                                eprintln!("Bybit WebSocket error: {}", error);
                                continue;
                            }

                            // Handle orderbook data
                            if let Some(topic) = data.get("topic").and_then(|t| t.as_str()) {
                                if topic.starts_with("orderbook") {
                                    if let Some(data) = data.get("data") {
                                        if let Some(bids) = data.get("b").and_then(|b| b.as_array()) {
                                            if let Some(asks) = data.get("a").and_then(|a| a.as_array()) {
                                                if !bids.is_empty() && !asks.is_empty() {
                                                    let order_book = OrderBookL1 {
                                                        exchange: "bybit".to_string(),
                                                        symbol: symbol.clone(),
                                                        bid: bids[0][0].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                                                        bid_volume: bids[0][1].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                                                        ask: asks[0][0].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                                                        ask_volume: asks[0][1].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                                                        timestamp: SystemTime::now(),
                                                    };
                                                    callback(order_book);
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    Ok(Message::Close(_)) => {
                        println!("Bybit WebSocket connection closed");
                        break;
                    }
                    Err(e) => {
                        eprintln!("Bybit WebSocket error: {}", e);
                        break;
                    }
                    _ => {}
                }
            }
        });

        Ok(())
    }
}