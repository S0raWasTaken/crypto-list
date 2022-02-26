use std::fmt::{Display, Formatter};

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Coin {
    pub id: String,
    pub name: String,
    pub current_price: f64,
    pub symbol: String,
    pub market_cap: f64,
    pub total_volume: f64,
    pub high_24h: f64,
    pub low_24h: f64,
    pub price_change_24h: f64,
    pub circulating_supply: f64,
    pub ath: f64,
    pub atl: f64,
}

impl Display for Coin {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let currency = std::env::var("CURRENCY").unwrap();
        write!(
            f,
            "{}:
  Current Price: {} {currency}
  ID: {}
  Symbol: {}
  Market Cap: {} {currency}
  Total Volume: {} {currency}
  High 24h: {} {currency}
  Low 24h: {} {currency}
  Price Change 24h: {} {currency}
  Circulating Supply: {} {currency}
  All Time High: {} {currency}
  All Time Low: {} {currency}",
            self.name,
            self.current_price,
            self.id,
            self.symbol,
            self.market_cap,
            self.total_volume,
            self.high_24h,
            self.low_24h,
            self.price_change_24h,
            self.circulating_supply,
            self.ath,
            self.atl
        )
    }
}
