# soqa-sdk


<p align="center">
  <b>Ultra-fast, unified market data for HFT & algo-traders — powered by Rust ⚡</b>
</p>

<p align="center">
  <a href="https://www.linkedin.com/company/soqaio"><img src="https://img.shields.io/badge/LinkedIn-SOQA-blue?logo=linkedin" alt="LinkedIn"></a>
  <img src="https://img.shields.io/badge/Rust-stable-orange?logo=rust" alt="Rust Stable">
  <img src="https://img.shields.io/badge/PRs-welcome-brightgreen.svg" alt="PRs Welcome">
</p>

---

**soqa-sdk** is an open-source Rust-based tool designed to empower algo-traders and HFT teams by aggregating and normalizing market data from multiple exchanges. Our mission is to simplify access to clean, low-latency data, enabling traders to build high-performance strategies without the hassle of inconsistent APIs.

[Follow us on LinkedIn](https://www.linkedin.com/company/soqaio)

---

## ✨ Features
- 🚀 **Ultra-low latency**: Real-time L1 orderbook data from top crypto exchanges
- 🦀 **Rust-powered**: Safe, fast, and reliable
- 🔗 **Unified API**: One interface for all supported exchanges
- 🛠️ **Easy to extend**: Add new exchanges in minutes
- 📦 **Async & modern**: Built on Tokio, reqwest, and tungstenite
- 🧩 **Open-source**: MIT licensed, PRs welcome!

## 🌍 Supported Exchanges
- **Binance**
- **Bybit**
- **OKX**
- **Kraken**
- **KuCoin**

## 🚀 Quick Start

### Requirements
- Rust (latest stable recommended)
- [Tokio](https://tokio.rs/) (async runtime)
- [reqwest](https://docs.rs/reqwest/) (HTTP client)
- [tokio-tungstenite](https://docs.rs/tokio-tungstenite/) (WebSocket)
- [serde/serde_json](https://serde.rs/) (JSON parsing)

### Build
```bash
cargo build --release
```

### Run

Example: Get ETHUSDT data from KuCoin:
```bash
cargo run --release -- start --exchange kucoin --symbol ETHUSDT
```

Parameters:
- `--exchange` — exchange name (binance, bybit, okx, kraken, kucoin)
- `--symbol` — trading pair (e.g., BTCUSDT, ETHUSDT)

## 📊 Example Output

The console prints `OrderBookL1` objects:
```rust
OrderBookL1 {
    exchange: "bybit",
    symbol: "ETHUSDT",
    bid: 2498.57,
    bid_volume: 0.38006,
    ask: 2498.58,
    ask_volume: 7.90089,
    timestamp: SystemTime { tv_sec: 1749926167, tv_nsec: 147185000 }
}
```

## 🗂️ Project Structure
```
soqa-sdk/
├── src/
│   ├── exchanges/
│   │   ├── binance.rs
│   │   ├── bybit.rs
│   │   ├── okx.rs
│   │   ├── kraken.rs
│   │   ├── kucoin.rs
│   │   └── mod.rs
│   ├── models.rs
│   ├── error.rs
│   └── main.rs
├── Cargo.toml
└── README.md
```

## 🛠️ How It Works
- Each exchange has a dedicated client that connects to its WebSocket API.
- After connecting, a subscription message is sent for the selected trading pair.
- Incoming messages are parsed, and L1 orderbook data (bid/ask) is extracted and passed to a callback.
- For KuCoin and some other exchanges, a REST API token is required before connecting to WebSocket.

## 💡 Example Usage in Code
```rust
let client = KuCoinClient::new(config);
client.subscribe_l1(|orderbook| {
    println!("{:?}", orderbook);
}).await?;
```

## 🏁 Example Commands for Each Exchange

- **Binance**
  ```bash
  cargo run --release -- start --exchange binance --symbol BTCUSDT
  ```
- **Bybit**
  ```bash
  cargo run --release -- start --exchange bybit --symbol ETHUSDT
  ```
- **OKX**
  ```bash
  cargo run --release -- start --exchange okx --symbol BTC-USDT
  ```
- **Kraken**
  ```bash
  cargo run --release -- start --exchange kraken --symbol XBT/USD
  ```
- **KuCoin**
  ```bash
  cargo run --release -- start --exchange kucoin --symbol ETHUSDT
  ```

## 🤔 FAQ
**Q:** Why am I not receiving data from KuCoin?
- Check the symbol format (e.g., ETH-USDT).
- Make sure you are subscribing to the correct topic (`/market/ticker:SYMBOL`).
- Check logs — sometimes the exchange does not send data if there is no activity.

**Q:** How do I add my own exchange?
- Implement a client structure and subscription method similar to the existing ones.
- Register the new module in `mod.rs` and add handling in the main match statement.

**Q:** What data can I get?
- Only L1 orderbook (bid/ask) for the selected trading pair.

---

<p align="center">
  <b>Contact & Support:</b><br>
  Issues and suggestions — via GitHub Issues<br>
  Pull Requests are welcome!<br>
  <a href="https://www.linkedin.com/company/soqaio">LinkedIn: SOQA</a>
</p> 