//Đoạn code này tạo schema JSON cho contract, giúp frontend có thể dễ dàng sử dụng.
// Xóa schema cũ, tạo mới schema cho messages (Instantiate, Execute, Query, Migrate) và state structs (Config, Poll, Ballot).
// Dùng khi muốn tích hợp contract với frontend hoặc cần kiểm tra định dạng dữ liệu.

use std::env::current_dir; // lấy thư mục hiện tại chương trình đang chạy
use std::fs::create_dir_all; // tạo thư mục(nếu chưa có)

use cosmwasm_schema::{export_schema, remove_schemas, schema_for};
//export_schema(...): Xuất schema JSON cho một struct hoặc enum.
//remove_schemas(&out_dir): Xóa các file schema cũ trước khi tạo mới.
//schema_for!(T): Lấy schema JSON cho struct/enum T.

use cw_starter::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg}; // Các message
                                                                         //ExecuteMsg: Định nghĩa các hàm thực thi (execute) của contract.
                                                                         //InstantiateMsg: Định nghĩa dữ liệu khởi tạo contract.
                                                                         //MigrateMsg: Định nghĩa dữ liệu khi nâng cấp contract.
                                                                         //QueryMsg: Định nghĩa các truy vấn (query) của contract.

use cw_starter::state::{Ballot, Config, Poll}; // Các trạng thái của contract

fn main() {
    let mut out_dir = current_dir().unwrap(); //Lấy thư mục hiện tại và thêm thư mục "schema" vào đường dẫn (out_dir.push("schema")).
    out_dir.push("schema"); //  Tạo thư mục "schema" nếu chưa tồn tại (create_dir_all(&out_dir)).
    create_dir_all(&out_dir).unwrap(); // Xóa schema cũ trước khi xuất JSON mới (remove_schemas(&out_dir)).
    remove_schemas(&out_dir).unwrap(); //

    // Xuất schema cho từng struct và enum
    export_schema(&schema_for!(InstantiateMsg), &out_dir);
    export_schema(&schema_for!(ExecuteMsg), &out_dir);
    export_schema(&schema_for!(MigrateMsg), &out_dir);
    export_schema(&schema_for!(QueryMsg), &out_dir);
    export_schema(&schema_for!(Config), &out_dir);
    export_schema(&schema_for!(Poll), &out_dir);
    export_schema(&schema_for!(Ballot), &out_dir);
}

/*
template mới
use cosmwasm_schema::write_api;

use cw_starter::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};

fn main() {
    write_api! {
        instantiate: InstantiateMsg,
        execute: ExecuteMsg,
        query: QueryMsg,
    }
}
 */
