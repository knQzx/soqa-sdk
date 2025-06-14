use crate::models::OrderBookL1;
use crate::error::SoqaError;
use tokio_tungstenite::{connect_async, tungstenite::Message};
use serde_json::Value;
use std::time::SystemTime;
use futures_util::stream::StreamExt;
use futures_util::SinkExt;
use reqwest::Client;
use tokio::time::{sleep, Duration};

pub struct KuCoinClient {
    config: crate::config::Config,
}

impl KuCoinClient {
    pub fn new(config: crate::config::Config) -> Self {
        KuCoinClient { config }
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
        // Получаем URL WebSocket-сервера от KuCoin API с использованием POST
        let client = Client::new();
        let response = client.post("https://api.kucoin.com/api/v1/bullet-public")
            .send()
            .await
            .map_err(|e| SoqaError::Http(e))?;

        if !response.status().is_success() {
            return Err(SoqaError::ConnectionError(format!(
                "Failed to get KuCoin WebSocket URL: HTTP {}",
                response.status()
            )));
        }

        let data = response.json::<Value>()
            .await
            .map_err(|e| SoqaError::Http(e))?;

        let ws_url = data["data"]["instanceServers"][0]["endpoint"]
            .as_str()
            .ok_or_else(|| SoqaError::ConnectionError("Failed to get KuCoin WebSocket URL from response".into()))?;

        let token = data["data"]["token"]
            .as_str()
            .ok_or_else(|| SoqaError::ConnectionError("Failed to get KuCoin token from response".into()))?;

        let connect_id = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_millis();

        let full_url = format!("{}?token={}&connectId={}", ws_url, token, connect_id);

        println!("Connecting to KuCoin WebSocket at: {}", full_url);
        let (ws_stream, _) = connect_async(&full_url).await?;
        let (mut write, mut read) = ws_stream.split();

        let symbol = self.convert_symbol(&self.config.symbol);
        let original_symbol = self.config.symbol.clone();
        println!("Subscribing to KuCoin with symbol: {}", symbol);

        // Подписываемся на оба канала: level1 и ticker
        let subscribe_msg = format!(
            r#"{{"id":{},"type":"subscribe","topic":"/market/ticker:{}","privateChannel":false,"response":true}}"#,
            connect_id,
            symbol
        );

        println!("Sending subscription message: {}", subscribe_msg);
        write.send(Message::Text(subscribe_msg)).await?;

        // Запускаем пинг в отдельном таске
        let mut write_ping = write;
        tokio::spawn(async move {
            loop {
                sleep(Duration::from_secs(30)).await;
                let ping_msg = format!(
                    r#"{{"id":{},"type":"ping"}}"#,
                    SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis()
                );
                if let Err(e) = write_ping.send(Message::Text(ping_msg)).await {
                    eprintln!("Failed to send ping: {}", e);
                    break;
                }
            }
        });

        tokio::spawn(async move {
            while let Some(msg) = read.next().await {
                match msg {
                    Ok(Message::Text(text)) => {
                        println!("Received raw message: {}", text);
                        if let Ok(data) = serde_json::from_str::<Value>(&text) {
                            // Подтверждение подключения
                            if data.get("type").and_then(|t| t.as_str()) == Some("welcome") {
                                println!("Successfully connected to KuCoin WebSocket");
                                continue;
                            }

                            // Подтверждение подписки
                            if data.get("type").and_then(|t| t.as_str()) == Some("ack") {
                                println!("Successfully subscribed to KuCoin WebSocket");
                                continue;
                            }

                            // Обработка данных ticker
                            if let Some(topic) = data.get("topic").and_then(|t| t.as_str()) {
                                println!("Received topic: {}", topic);
                                if topic.starts_with("/market/ticker") {
                                    if let Some(data) = data.get("data") {
                                        println!("Processing ticker data: {:?}", data);
                                        let order_book = OrderBookL1 {
                                            exchange: "kucoin".to_string(),
                                            symbol: original_symbol.clone(),
                                            bid: data["bestBid"].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                                            bid_volume: data["bestBidSize"].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                                            ask: data["bestAsk"].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                                            ask_volume: data["bestAskSize"].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                                            timestamp: SystemTime::now(),
                                        };
                                        println!("Created OrderBookL1: {:?}", order_book);
                                        callback(order_book);
                                    }
                                }
                            }

                            // Обработка pong сообщений
                            if data.get("type").and_then(|t| t.as_str()) == Some("pong") {
                                println!("Received pong");
                                continue;
                            }
                        }
                    }
                    Ok(Message::Close(_)) => {
                        println!("KuCoin WebSocket connection closed");
                        break;
                    }
                    Err(e) => {
                        eprintln!("KuCoin WebSocket error: {}", e);
                        break;
                    }
                    _ => {}
                }
            }
        });

        Ok(())
    }
}