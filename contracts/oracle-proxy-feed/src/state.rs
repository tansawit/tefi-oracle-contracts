use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Decimal};
use cw_storage_plus::{Item, Map};
use tefi_oracle::proxy::ProxyPriceResponse;

use crate::msg::ConfigResponse;

pub const CONFIG: Item<Config> = Item::new("config");
pub const FEEDERS: Map<&Addr, Addr> = Map::new("feeders");
pub const PRICES: Map<&Addr, PriceInfo> = Map::new("prices");

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct Config {
    pub owner: Addr,
}

impl Config {
    pub fn as_res(&self) -> ConfigResponse {
        ConfigResponse {
            owner: self.owner.to_string(),
        }
    }

    /// @dev Checks if the provided addr is owner
    /// @param addr : address to check
    pub fn is_owner(&self, addr: &Addr) -> bool {
        self.owner.eq(addr)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct PriceInfo {
    pub price: Decimal,
    pub last_updated_time: u64,
}

impl PriceInfo {
    pub fn as_res(&self) -> ProxyPriceResponse {
        ProxyPriceResponse {
            rate: self.price,
            last_updated: self.last_updated_time,
        }
    }
}
