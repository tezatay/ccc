use clap::Parser;
use rust_decimal::Decimal;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
struct Response {
    rate: Decimal,
}

#[derive(Parser, Debug)]
struct Args {
    #[arg(requires_all = ["base", "to"])]
    amount: Option<Decimal>,

    /// Исходная валюта (например, USD)
    #[arg(short, long)]
    base: Option<String>,

    /// Целевая валюта (например, EUR)
    #[arg(short, long)]
    to: Option<String>,
}

async fn convert(amount: Decimal, base: &str, to: &str) -> Decimal {
    if base.to_lowercase() == to.to_lowercase() {
        return amount;
    }
    let url = format!("https://api.frankfurter.dev/v2/rate/{}/{}", base, to);
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
    if let (Some(amount), Some(base), Some(to)) = (args.amount, args.base, args.to) {
        let result = convert(amount, &base, &to).await;
        println!("{} {} = {} {}", amount, base, result, to);
    } else {
        Args::parse_from(["app", "--help"]);
    }
}
