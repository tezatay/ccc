use clap::Parser;
use rust_decimal::Decimal;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
struct Response {
    rate: Decimal,
}

#[derive(Parser, Debug)]
struct Args {
    #[arg(requires_all = ["base", "quote"])]
    amount: Option<Decimal>,

    /// Исходная валюта (например, USD)
    #[arg(short, long)]
    base: Option<String>,

    /// Целевая валюта (например, EUR)
    #[arg(short, long)]
    quote: Option<String>,
}

async fn convert(amount: Decimal, base: &str, quote: &str) -> Decimal {
    if base.to_lowercase() == quote.to_lowercase() {
        return amount;
    }
    let url = format!("https://api.frankfurter.dev/v2/rate/{}/{}", base, quote);
    let response = reqwest::get(&url)
        .await
        .expect("Unable to connect to server");
    let data = response
        .json::<Response>()
        .await
        .expect("Unable to parse json");
    amount * data.rate
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    if let (Some(amount), Some(base), Some(quote)) = (args.amount, args.base, args.quote) {
        let result = convert(amount, &base, &quote).await;
        println!("{} {} = {} {}", amount, base, result, quote);
    } else {
        Args::parse_from(["app", "--help"]);
    }
}
