/*kiểu template mới
use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {}

*/

use crate::state::{Ballot, Poll};
/// Định nghĩa các Message type của smart contract
use schemars::JsonSchema; // tự động tạo JSON schema
use serde::{Deserialize, Serialize};

//
//Define các custom struct để gọi query
// Query messages are handled differently when returning from a query you don't return via a Response you must define a custom struct which can then be encoded to Binary
//không cần thiêt define struct khi chỉ cần query một giá trị đơn giản (bool, u64, ..)
//Cần define struct khi query cần trả về nhiều giá trị, oject phức tạp
//chẳng hạn cần trả về question + options
// pub struct PollResponse {
//     pub question: String,
//     pub options: Vec<(String, u64)>,
// }

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct AllPollsResponse {
    pub polls: Vec<Poll>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct PollResponse {
    pub poll: Option<Poll>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct VoteResponse {
    pub vote: Option<Ballot>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")] // Chuyển đổi tất cả tên field của struct/enum thành snake_case khi serialize/deserialze

// Dữ liệu init contract khi deploy lần đầu
pub struct InstantiateMsg {
    pub admin: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]

// Dữ liệu khi gọi tham thay đổi trạng thái
// định nghĩa các hành động có thể thực hiện trên contract (các hàm public)
pub enum ExecuteMsg {
    //CustomMsg {val: String};
    CreatePoll {
        poll_id: String,
        question: String,
        options: Vec<String>,
    },
    Vote {
        poll_id: String,
        vote: String,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]

// Dữ liệu khi truy vấn trạng thái -> view (các thông số public)
pub enum QueryMsg {
    // muốn thực hiện query chỉ xem
    // cần trả dữ liệu ở dạng binary
    AllPoll {},
    Poll { poll_id: String },
    Vote { poll_id: String, address: String },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]

// Dữ liệu khi nâng cấp contract
pub enum MigrateMsg {}
