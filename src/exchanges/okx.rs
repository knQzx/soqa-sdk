use crate::models::OrderBookL1;
use crate::error::SoqaError;
use tokio_tungstenite::{connect_async, tungstenite::Message};
use serde_json::Value;
use std::time::SystemTime;
use futures_util::stream::StreamExt;
use futures_util::SinkExt;

pub struct OkxClient {
    config: crate::config::Config,
}

impl OkxClient {
    pub fn new(config: crate::config::Config) -> Self {
        OkxClient { config }
    }

    fn convert_symbol(&self, symbol: &str) -> String {
        match symbol {
            "BTCUSD" => "BTC-USD".to_string(),
            "BTCUSDT" => "BTC-USDT".to_string(),
            "ETHUSD" => "ETH-USD".to_string(),
            "ETHUSDT" => "ETH-USDT".to_string(),
            _ => symbol.to_string(),
        }
    }

    pub async fn subscribe_l1(&self, callback: impl Fn(OrderBookL1) + Send + 'static) -> Result<(), SoqaError> {
        let url = "wss://ws.okx.com:8443/ws/v5/public";
        let (ws_stream, _) = connect_async(url).await?;
        let (mut write, mut read) = ws_stream.split();

        let symbol = self.convert_symbol(&self.config.symbol);
        let original_symbol = self.config.symbol.clone();
        println!("Subscribing to OKX with symbol: {}", symbol);

        let subscribe_msg = format!(
            r#"{{"op":"subscribe","args":[{{"channel":"books","instId":"{}"}}]}}"#,
            symbol
        );
        println!("Sending subscription message: {}", subscribe_msg);
        write.send(Message::Text(subscribe_msg)).await?;

        tokio::spawn(async move {
            while let Some(msg) = read.next().await {
                match msg {
                    Ok(Message::Text(text)) => {
                        println!("Received message: {}", text);
                        if let Ok(data) = serde_json::from_str::<Value>(&text) {
                            // Handle subscription confirmation
                            if data.get("event").and_then(|e| e.as_str()) == Some("subscribe") {
                                println!("Successfully subscribed to OKX WebSocket");
                                continue;
                            }

                            // Handle error messages
                            if data.get("event").and_then(|e| e.as_str()) == Some("error") {
                                if let Some(error_msg) = data.get("msg").and_then(|e| e.as_str()) {
                                    eprintln!("OKX WebSocket error: {}", error_msg);
                                }
                                continue;
                            }

                            // Handle orderbook data
                            if let Some(data) = data.get("data").and_then(|d| d.as_array()) {
                                if let Some(book) = data.get(0) {
                                    if let Some(bids) = book.get("bids").and_then(|b| b.as_array()) {
                                        if let Some(asks) = book.get("asks").and_then(|a| a.as_array()) {
                                            if !bids.is_empty() && !asks.is_empty() {
                                                let order_book = OrderBookL1 {
                                                    exchange: "okx".to_string(),
                                                    symbol: original_symbol.clone(),
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
                    Ok(Message::Close(_)) => {
                        println!("OKX WebSocket connection closed");
                        break;
                    }
                    Err(e) => {
                        eprintln!("OKX WebSocket error: {}", e);
                        break;
                    }
                    _ => {}
                }
            }
        });

        Ok(())
    }
}