use soqa_sdk::config::Config;
use soqa_sdk::exchanges::binance::BinanceClient;

#[tokio::main]
async fn main() {
    let config = Config::new("binance", "BTCUSDT");
    let client = BinanceClient::new(config);
    client
        .subscribe_l1(|order_book| {
            println!("{:?}", order_book);
        })
        .await
        .unwrap();
    tokio::time::sleep(std::time::Duration::from_secs(10)).await;
}