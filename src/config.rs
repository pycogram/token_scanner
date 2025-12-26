use std::env;
use once_cell::sync::Lazy; // for global lazy static

// Load once at runtime
pub static DEV_WALLET: Lazy<String> = Lazy::new(|| {
    env::var("DEV_WALLET").expect("DEV_WALLET must be set in .env")
});

pub static DEV_PRIVATE_KEY: Lazy<String> = Lazy::new(|| {
    env::var("DEV_PRIVATE_KEY").expect("DEV_PRIVATE_KEY must be set in .env")
});

pub static TELEGRAM_BOT_TOKEN: Lazy<String> = Lazy::new(|| {
    env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN must be set in .env")
});

pub static TELEGRAM_CHAT_ID: Lazy<i64> = Lazy::new(|| {
    env::var("TELEGRAM_CHAT_ID")
        .expect("TELEGRAM_CHAT_ID must be set")
        .parse()
        .expect("TELEGRAM_CHAT_ID must be a number")
});

pub const TOKEN_THRESHOLD: u64 = 100_000;
pub const CYCLE_HOURS: i64 = 1;
pub const WINNER_PERCENTAGE: f64 = 0.10;
pub const MAX_PREV_WINNER_RATIO: f64 = 0.3;
pub const FEE_SPLIT_DEV: f64 = 0.10;
pub const FEE_SPLIT_WINNERS: f64 = 0.90;
pub const WINNERS_JSON_PATH: &str = "./winners.json";
