use schemars::JsonSchema; //là một derive macro
                          // Giúp tự động tạo JSON schema từ các struct và enum
                          // Làm việc với crate serde
use serde::{Deserialize, Serialize};
// Serialize : Chuyển đổi struct/enum -> JSON
// Deserialize: chuyển đổi JSON -> struct/enum

use cosmwasm_std::Addr; // làm việc với Cosmos address
                        //Addr thực chất là wrapped String????

use cw_storage_plus::{Item, Map}; // Lưu trữ giá trị trên chain
                                  // Moi Item la mot bien trangj thai, moi Iteam chi luu 1 bien
                                  // tuong tu moi Map chi luu mot map

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)] // marco để impl trait cho struct
pub struct Config {
    // cấu trúc định nghĩa các biến state
    pub admin: Addr, // Admin address
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Poll {
    //Lưu thông tin Poll
    pub creator: Addr,
    pub question: String,
    pub options: Vec<(String, u64)>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Ballot {
    // Lưu thông tin vote của user
    pub option: String,
}

pub const CONFIG: Item<Config> = Item::new("config"); //lưu các item(các biến đơn)

pub const POLLS: Map<String, Poll> = Map::new("polls");

pub const BALLOTS: Map<(Addr, String), Ballot> = Map::new("ballots");
