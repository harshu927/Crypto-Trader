use std::collections::VecDeque;
use std::path::PathBuf;
use std::time::Duration;

use clap::Parser;
use csv::Reader;
use log::{error, info, warn};
use reqwest::Client;
use serde::Deserialize;
use teloxide::{Bot, prelude::Requester};
use teloxide::types::ChatId;
use tokio::time::{interval, sleep};

#[derive(Parser, Debug)]
#[clap(version = "1.0", author = "Your Name")]
struct Args {
    #[clap(long)]
    backtest: bool,

    #[clap(long, default_value = "historical_prices.csv")]
    historical_data: PathBuf,

    #[clap(long, default_value_t = 5)]
    sma_short: usize,

    #[clap(long, default_value_t = 20)]
    sma_long: usize,

    #[clap(long, default_value_t = 5.0)]
    stop_loss: f64,

    #[clap(long, env = "TELEGRAM_BOT_TOKEN")]
    telegram_token: Option<String>,

    #[clap(long, env = "TELEGRAM_CHAT_ID")]
    telegram_chat_id: Option<String>,
}

#[derive(Debug, Deserialize)]
struct HistoricalPrice {
    timestamp: String,
    price: f64,
}

#[derive(Debug, Deserialize)]
struct CoinData {
    market_data: MarketData,
}

#[derive(Debug, Deserialize)]
struct MarketData {
    current_price: CurrentPrice,
    last_updated: String,
}

#[derive(Debug, Deserialize)]
struct CurrentPrice {
    usd: f64,
}

struct TradingBot {
    client: Client,
    args: Args,
    prices: VecDeque<f64>,
    short_sum: f64,
    long_sum: f64,
    position: i32,
    profit: f64,
    last_buy_price: f64,
    bot: Option<Bot>,
    chat_id: Option<i64>,
}

impl TradingBot {
    async fn new(args: Args) -> Self {
        let chat_id = args
            .telegram_chat_id
            .as_ref()
            .and_then(|id| id.parse::<i64>().ok());
        let bot = match &args.telegram_token {
            Some(token) => Some(Bot::new(token)),
            None => None,
        };

        if let (Some(bot), Some(chat_id)) = (&bot, &chat_id) {
            let _ = bot
                .send_message(ChatId(*chat_id), "🚀 Trading bot started!")
                .await;
        }

        TradingBot {
            client: Client::new(),
            args,
            prices: VecDeque::with_capacity(100),
            short_sum: 0.0,
            long_sum: 0.0,
            position: 0,
            profit: 0.0,
            last_buy_price: 0.0,
            bot,
            chat_id,
        }
    }

    async fn send_alert(&self, message: &str) {
        if let (Some(bot), Some(chat_id)) = (&self.bot, &self.chat_id) {
            let _ = bot.send_message(ChatId(*chat_id), message).await;
        }
    }

    async fn process_price(&mut self, price: f64, timestamp: &str) {
        let n = self.prices.len();
        
        if n >= self.args.sma_short {
            self.short_sum -= self.prices[self.args.sma_short - 1];
        }
        self.short_sum += price;
        self.long_sum += price;
        self.prices.push_front(price);
        
        if n >= self.args.sma_long {
            if let Some(old_price) = self.prices.pop_back() {
                self.long_sum -= old_price;
            }
        }

        if n >= 5 {
            let prev_price = self.prices[1];
            let change_rate = (price - prev_price) / prev_price * 100.0;
            if change_rate.abs() > 2.0 {
                let alert = format!(
                    "{}: ALERT - Price {:.2}% ({})",
                    timestamp,
                    change_rate,
                    if change_rate > 0.0 { "↑" } else { "↓" }
                );
                warn!("{}", alert);
                self.send_alert(&alert).await;
            }
        }

        if self.prices.len() >= self.args.sma_long {
            let short_sma = self.short_sum / self.args.sma_short as f64;
            let long_sma = self.long_sum / self.args.sma_long as f64;
            let current_above = short_sma > long_sma;

            if self.position == 1 {
                let current_loss = (price - self.last_buy_price) / self.last_buy_price * 100.0;
                if current_loss <= -self.args.stop_loss {
                    let message = format!(
                        "🚨 STOP-LOSS TRIGGERED!\nSold at ${:.2}\nLoss: {:.2}%",
                        price, current_loss
                    );
                    self.execute_sell(price, &message).await;
                    return;
                }
            }

            match (self.position, current_above) {
                (0, true) => {
                    let message = format!(
                        "✅ BUY SIGNAL\nPrice: ${:.2}\nSMA{}: {:.2}\nSMA{}: {:.2}",
                        price, self.args.sma_short, short_sma, self.args.sma_long, long_sma
                    );
                    self.execute_buy(price, &message).await;
                }
                (1, false) => {
                    let message = format!(
                        "🛑 SELL SIGNAL\nPrice: ${:.2}\nSMA{}: {:.2}\nSMA{}: {:.2}",
                        price, self.args.sma_short, short_sma, self.args.sma_long, long_sma
                    );
                    self.execute_sell(price, &message).await;
                }
                _ => {}
            }
        }
    }

    async fn execute_buy(&mut self, price: f64, message: &str) {
        self.position = 1;
        self.last_buy_price = price;
        info!("{}", message);
        self.send_alert(message).await;
    }

    async fn execute_sell(&mut self, price: f64, message: &str) {
        self.profit += price - self.last_buy_price;
        self.position = 0;
        info!("{}", message);
        self.send_alert(message).await;
        let profit_msg = format!("💰 Current profit: ${:.2}", self.profit);
        info!("{}", profit_msg);
        self.send_alert(&profit_msg).await;
    }

    async fn live_trading(&mut self) {
        let mut interval = interval(Duration::from_secs(60));
        info!("Starting LIVE trading mode");

        loop {
            interval.tick().await;
            
            let mut retries = 0;
            let response = loop {
                match self.client.get("https://api.coingecko.com/api/v3/coins/bitcoin").send().await {
                    Ok(resp) => break resp,
                    Err(_) if retries < 3 => {
                        retries += 1;
                        sleep(Duration::from_secs(2u64.pow(retries))).await;
                        continue;
                    }
                    Err(e) => {
                        error!("API request failed: {}", e);
                        return;
                    }
                }
            };

            match response.json::<CoinData>().await {
                Ok(coin_data) => {
                    let price = coin_data.market_data.current_price.usd;
                    let timestamp = &coin_data.market_data.last_updated;
                    self.process_price(price, timestamp).await;
                }
                Err(e) => error!("Failed to parse API response: {}", e),
            }
        }
    }

    async fn backtest(&mut self) {
        info!("Starting BACKTEST mode");
        let mut rdr = Reader::from_path(&self.args.historical_data).unwrap();
        
        for result in rdr.deserialize() {
            match result {
                Ok(record) => {
                    let hp: HistoricalPrice = record;
                    self.process_price(hp.price, &hp.timestamp).await;
                }
                Err(e) => error!("CSV parsing error: {}", e),
            }
        }
        info!("Backtest complete. Final profit: ${:.2}", self.profit);
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let args = Args::parse();

    let mut bot = TradingBot::new(args).await;
    
    if bot.args.backtest {
        bot.backtest().await;
    } else {
        bot.live_trading().await;
    }

    Ok(())
}