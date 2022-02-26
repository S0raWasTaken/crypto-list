mod primitives;

use std::env::var;
use std::fs::read_to_string;

use anyhow::{anyhow, Context, Result};
use primitives::{CoinSerializer, Coin};
use reqwest::get;
use rofi::{Rofi, Width};
use yaml_rust::YamlLoader;

#[tokio::main]
async fn main() -> Result<()> {
    let config = &YamlLoader::load_from_str(
        &read_to_string(format!("{}/.config/crypto-list/config.yml", var("HOME")?))
            .context("Config file not found")?,
    )?[0];

    let comparison_currency = config["fiat-currency"]
        .as_str()
        .ok_or(anyhow!("config[\"fiat-currency\"] is a None value"))
        .context("No fiat-currency in config file?")?;

    let coins = config["coins"]
        .as_vec()
        .ok_or(anyhow!("No coins in config file?"))?
        .iter()
        .map(|c| c.as_str().unwrap())
        .collect::<Vec<_>>();

    let currencies = get(format!(
        "https://api.coingecko.com/api/v3/coins/markets?vs_currency={comparison_currency}&ids={}",
        coins.join(",")
    ))
    .await?
    .json::<Vec<CoinSerializer>>()
    .await?
    .iter()
    .map(|c| Coin(c.clone(), comparison_currency.to_owned()))
    .collect::<Vec<_>>();

    let parsed = currencies
        .iter()
        .map(|c| {
            format!(
                "{}: {} {comparison_currency}",
                c.0.symbol, c.0.current_price
            )
        })
        .collect::<Vec<_>>();

    if let Ok(element) = Rofi::new(&parsed)
        .prompt("search")
        .lines(15)
        .width(Width::Pixels(400))?
        .run_index()
    {
        Rofi::new_message(format!("{}", currencies[element]))
            .run()
            .ok();
    }

    Ok(())
}
