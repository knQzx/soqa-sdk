use clap::Parser;
use soqa_sdk::cli::Cli;
use soqa_sdk::config::Config;
use soqa_sdk::exchanges::binance::BinanceClient;
use soqa_sdk::exchanges::bybit::BybitClient;
use soqa_sdk::exchanges::kraken::KrakenClient;
use soqa_sdk::exchanges::okx::OkxClient;
use soqa_sdk::exchanges::kucoin::KuCoinClient;
use soqa_sdk::api::websocket::websocket_route;
use soqa_sdk::api::rest::rest_routes;
use warp::Filter;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.command {
        soqa_sdk::cli::Commands::Start { exchange, symbol, level } => {
            let config = Config::new(&exchange, &symbol);
            match exchange.as_str() {
                "binance" => {
                    let client = BinanceClient::new(config);
                    client.subscribe_l1(|order_book| println!("{:?}", order_book)).await.unwrap();
                }
                "bybit" => {
                    let client = BybitClient::new(config);
                    client.subscribe_l1(|order_book| println!("{:?}", order_book)).await.unwrap();
                }
                "kraken" => {
                    let client = KrakenClient::new(config);
                    client.subscribe_l1(|order_book| println!("{:?}", order_book)).await.unwrap();
                }
                "okx" => {
                    let client = OkxClient::new(config);
                    client.subscribe_l1(|order_book| println!("{:?}", order_book)).await.unwrap();
                }
                "kucoin" => {
                    let client = KuCoinClient::new(config);
                    client.subscribe_l1(|order_book| println!("{:?}", order_book)).await.unwrap();
                }
                _ => println!("Unsupported exchange: {}", exchange),
            }
            tokio::time::sleep(std::time::Duration::from_secs(10)).await;
        }
        soqa_sdk::cli::Commands::Export { exchange, symbol, output } => {
            println!("Exporting data for {} {} to {}", exchange, symbol, output);
        }
    }

    let ws_route = websocket_route();
    let rest_route = rest_routes();
    let routes = ws_route.or(rest_route);
    warp::serve(routes).run(([127, 0, 0, 1], 8081)).await;
}