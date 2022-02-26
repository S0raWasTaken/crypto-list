mod primitives;

use std::{
    env::{set_var, var},
    fs::read_to_string,
};

use anyhow::{anyhow, Context, Result};
use reqwest::get;
use rofi::{Rofi, Width};
use yaml_rust::YamlLoader;

use crate::primitives::Coin;

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

    set_var("CURRENCY", comparison_currency);

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
    .json::<Vec<Coin>>()
    .await?;

    let mut parsed = Vec::<String>::new();
    for coin in &currencies {
        parsed.push(format!(
            "{}: {} {comparison_currency}",
            coin.symbol, coin.current_price
        ));
    }

    match Rofi::new(&parsed)
        .prompt("search")
        .lines(15)
        .width(Width::Pixels(400))?
        .run_index()
    {
        Ok(element) => {
            Rofi::new_message(format!("{}", currencies[element]))
                .run()
                .ok();
        }
        _ => (),
    }

    Ok(())
}
