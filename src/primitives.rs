use std::fmt::{Display, Formatter};

use serde::Deserialize;

#[derive(Debug, Clone)]
pub struct Coin(pub CoinSerializer, pub String);

#[derive(Deserialize, Debug, Clone)]
pub struct CoinSerializer {
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
            self.0.name,
            self.0.current_price,
            self.0.id,
            self.0.symbol,
            self.0.market_cap,
            self.0.total_volume,
            self.0.high_24h,
            self.0.low_24h,
            self.0.price_change_24h,
            self.0.circulating_supply,
            self.0.ath,
            self.0.atl,
            currency = self.1,
        )
    }
}
