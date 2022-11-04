use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct Config {
    pub admin: Addr,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct Poll {
    pub creator: Addr,
    pub question: String,
    pub options: Vec<(String, u64)>,
}

// What option the voter chose.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct Ballot {
    pub option: String,
}

// A map with a String key and Poll value.
// The key will be a UUID generated clientside
pub const POLLS: Map<String, Poll> = Map::new("polls");

pub const CONFIG: Item<Config> = Item::new("config");

// Addr - Address of the voter
// String - Poll UUID this vote is for.
pub const BALLOTS: Map<(Addr, String), Ballot> = Map::new("ballots");
