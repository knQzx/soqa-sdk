use serde::{Deserialize, Serialize};
use std::time::SystemTime;

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderBookL1 {
    pub exchange: String,
    pub symbol: String,
    pub bid: f64,
    pub bid_volume: f64,
    pub ask: f64,
    pub ask_volume: f64,
    pub timestamp: SystemTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderBookL2 {
    pub exchange: String,
    pub symbol: String,
    pub bids: Vec<(f64, f64)>,
    pub asks: Vec<(f64, f64)>,
    pub timestamp: SystemTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Trade {
    pub exchange: String,
    pub symbol: String,
    pub price: f64,
    pub volume: f64,
    pub side: String,
    pub timestamp: SystemTime,
}