use cosmwasm_std::StdError; //enum chứa các lỗi chuẩn của cosmwasm
                            // pub enum StdError {
                            //     NotFound { kind: String },        // Không tìm thấy dữ liệu
                            //     Unauthorized {},                   // Người dùng không có quyền
                            //     InvalidRequest { msg: String },    // Request không hợp lệ
                            //     GenericErr { msg: String },        // Lỗi chung chung
                            //     Overflow { .. },                    // Lỗi tràn số
                            // }

use thiserror::Error; // crate giúp tạo enum lỗi tùy chỉnh

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Too many poll options")]
    TooManyOptions {},
    // Add any other custom errors you like here.
    // Look at https://docs.rs/thiserror/1.0.21/thiserror/ for details.
}
