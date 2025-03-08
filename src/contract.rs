// viết contract trong cosmwasm giống như kiểu Proxy pattern

#[cfg(not(feature = "library"))] // chỉ dịch file này khi nó được chay như một thư viện
// =>  Mục đích: Chỉ import entry_point khi contract đang chạy độc lập, không phải là một thư viện.
// #[cfg(...)] là attribute kiểm soát biên dịch (conditional compilation) trong Rust.
// not(feature = "library") nghĩa là chỉ chạy đoạn code này nếu không bật feature "library".
use cosmwasm_std::{entry_point, Addr};
// Định nghĩa điểm vào
// entry_point là macro giúp xác định các hàm chính của contract khi deploy

use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Order, Response, StdResult,
};
// các struct / enum của cosmwasm_std
// ✅ Binary → Dữ liệu nhị phân (thường dùng cho query). là wrapper của Vec<u8>
// ✅ Deps & DepsMut → Truy cập storage & dependencies. Đọc ghi dữ liệu vào storage
// ✅ Env → Lấy thông tin blockchain (block height, contract address).
// ✅ MessageInfo → Lấy thông tin người gửi transaction & token đính kèm.
// ✅ Response → Trả về kết quả sau khi thực thi, thay đối state contract. Chứa log,message,event => ghi lại event sau khi transaction thực hiện -> EVENT
// ✅ StdResult → Kết quả thực thi (Ok(T) hoặc Err(StdError)). = Result<T, StdError>
//to_binary là một hàm hỗ trợ (helper function) giúp mã hóa các kiểu dữ liệu (Struct, String, Vec, ...) thành Binary

// pub struct DepsMut<'a> {
//     pub api: &'a dyn Api,         // Gọi API (ví dụ: xác thực địa chỉ)
//     pub storage: &'a mut dyn Storage, // Truy cập storage (có thể ghi)
//     pub querier: &'a dyn Querier,  // Truy vấn dữ liệu từ blockchain
// }

use cw2::set_contract_version;
//Hàm hỗ trợ lưu trữ contract version
// => dễ dàng theo dõi và nâng cấp
//set_contract_version(storage, contract_name, contract_version)

//Bonus: &mut dyn Storage là một trait object đại diện cho bộ nhớ lưu trữ (storage) của smart contract.
// pub trait Storage {
//     fn get(&self, key: &[u8]) -> Option<Vec<u8>>;
//     fn set(&mut self, key: &[u8], value: &[u8]);
//     fn remove(&mut self, key: &[u8]);
// }

use crate::error::ContractError;
//crate -> tham chiếu đến gốc của project
// error tự định nghĩa

use crate::msg::{
    AllPollsResponse, ExecuteMsg, InstantiateMsg, MigrateMsg, PollResponse, QueryMsg, VoteResponse,
};

use crate::state::{Ballot, Config, Poll, BALLOTS, CONFIG, POLLS};

const CONTRACT_NAME: &str = "crates.io:cw-starter";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)] // là entry_point nếu file được chạy không như một lib
                                                   // => xác định gốc file, file contract chính sẽ thực thi
                                                   // Dòng #[cfg_attr(not(feature = "library"), entry_point)] có tác dụng như sau:

// #[cfg_attr(...)]: Đây là một attribute điều kiện (conditional attribute).
// not(feature = "library"):
// Nếu feature "library" không được bật, Rust sẽ áp dụng entry_point cho hàm.
// Nếu "library" được bật, Rust bỏ qua entry_point.
// entry_point: Đây là macro của cosmwasm_std, dùng để xác định các hàm entry point (như instantiate, execute, query, migrate). Nó giúp contract chạy trên blockchain Cosmos.
// 👉 Mục đích:
// Khi contract chạy trên blockchain, cần entry_point để Cosmos-SDK nhận diện hàm.
// Khi contract được import như một thư viện (ví dụ trong unit test), entry_point không cần thiết.

pub fn instantiate(
    _deps: DepsMut,       //truy cập contract storage , có thể thay đổi
    _env: Env, // thông tin môi trường bao gồm address, block info (chiều cao và thời gian) và một số transaction info
    _info: MessageInfo, // thông tin sender, token gửi kèm
    _msg: InstantiateMsg, // init data
) -> Result<Response, ContractError> {
    set_contract_version(_deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?; // Lưu version và tên của contract

    let admin = _msg.admin.unwrap_or(_info.sender.to_string());
    // theo InstantianMsg trong msg.rs
    // admin là string
    // _msg.admin return Options<String>
    // unwrap_or(...) : lấy giá trị bên trong Some(value) ~value ở đây là địa chỉ admin~ hoặc nếu là None -> _infor.sender
    //_infor.sender -> địa chỉ người gọi contract có kiểu Addr
    // Nếu giải _msg.admin ra None thì lấy admin là người deploy

    ///eprintln!("{}", admin);
    let validated_admin = Addr::unchecked(admin.clone());
    ///let validated_admin = _deps.api.addr_validate(&admin)?;
    // Kiểm tra và xác định địa chỉ admin
    // gọi hàm addr_validate(&admin) để kiểm tra admin có hợp lệ không
    // nếu hợp lệ trả về Addr
    // Nếu không hợp lệ, trả về lỗi StdError::GenericErr { msg: "invalid address".to_string() }.
    let config = Config {
        admin: validated_admin.clone(),
    };

    CONFIG.save(_deps.storage, &config)?;

    Ok(Response::new()
        .add_attribute("action", "instantiate")
        .add_attribute("admin", validated_admin.to_string()))
    //Trả về Reponse chứa metadata để dễ dàng theo dõi khi blockchain được khởi tạo
    // Đồng thời emit event

    //unimplemented!() // marco đánh dấu một đoạn code chưa được hoàn thành -> panic ngay lập tức khi chạy chương trình
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    // Hàm thực thi các yêu cầu
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: ExecuteMsg, // chứa lệnh cần thực thi
) -> Result<Response, ContractError> {
    match _msg {
        ExecuteMsg::CreatePoll {
            poll_id,
            question,
            options,
        } => execute_create_poll(_deps, _env, _info, poll_id, question, options),

        ExecuteMsg::Vote { poll_id, vote } => execute_vote(_deps, _env, _info, poll_id, vote),
    }
    //unimplemented!()
}

fn execute_vote(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    poll_id: String,
    vote: String,
) -> Result<Response, ContractError> {
    let poll = POLLS.may_load(deps.storage, poll_id.clone())?;

    match poll {
        Some(mut poll) => {
            // The poll exists
            BALLOTS.update(
                // update BALLOTS tại key (info.sender, poll_id)
                deps.storage,
                (info.sender, poll_id.clone()),
                |ballot| -> StdResult<Ballot> {
                    //inline function thực hiện gì tại key (info.sender,id)
                    //Nếu trả về Ok(value) thì update giá trị tại khóa thành value
                    // tham số thứ 3 là closure  nhận vào 1 para là Option<T>, với T là kiểu của giá trị của Map
                    // function F: FnOnce(Option<T>) -> StdResult<T>
                    match ballot {
                        Some(ballot) => {
                            // trường hợp đã vote rồi giờ muốn vote lại
                            // We need to revoke their old vote
                            // Find the position
                            let position_of_old_vote = poll
                                .options
                                .iter()
                                .position(|option| option.0 == ballot.option)
                                .unwrap(); // tìm vote trong poll.options mà tương đồng với cái vote đã từng chọn
                                           // Decrement by 1
                            poll.options[position_of_old_vote].1 -= 1;
                            // Update the ballot
                            Ok(Ballot {
                                option: vote.clone(),
                            }) //update vote mới
                        }
                        None => {
                            //chưa vote trươc đó
                            // Simply add the ballot
                            Ok(Ballot {
                                option: vote.clone(),
                            }) //update vote mới
                        }
                    }
                },
            )?;

            // Find the position of the new vote option and increment it by 1
            let position = poll //Tìm vote mới
                .options
                .iter()
                .position(|option| option.0 == vote); // .position : tìm phần tử đầu tiên thỏa mãn điều kiện
            if position.is_none() {
                return Err(ContractError::Unauthorized {});
            }
            let position = position.unwrap();
            poll.options[position].1 += 1;

            // Save the update
            POLLS.save(deps.storage, poll_id, &poll)?;
            Ok(Response::new())
        }
        None => Err(ContractError::Unauthorized {}), // The poll does not exist so we just error
    }
}

fn execute_create_poll(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    poll_id: String,
    question: String,
    options: Vec<String>,
) -> Result<Response, ContractError> {
    if options.len() > 10 {
        return Err(ContractError::TooManyOptions {});
    }

    let mut opts: Vec<(String, u64)> = vec![];
    for option in options {
        opts.push((option, 0));
    }

    let poll = Poll {
        creator: info.sender,
        question,
        options: opts,
    };

    POLLS.save(deps.storage, poll_id, &poll)?;

    Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    match _msg {
        QueryMsg::AllPoll {} => query_all_polls(_deps, _env),
        QueryMsg::Poll { poll_id } => query_poll(_deps, _env, poll_id),
        QueryMsg::Vote { poll_id, address } => query_vote(_deps, _env, address, poll_id),
    }
    //unimplemented!()
}

fn query_all_polls(deps: Deps, _env: Env) -> StdResult<Binary> {
    let polls = POLLS
        .range(deps.storage, None, None, Order::Ascending)
        .map(|p| Ok(p?.1))
        .collect::<StdResult<Vec<_>>>()?;

    // đoạn trên tương đương
    // let polls_iter = POLLS
    //     .range(deps.storage, None, None, Order::Ascending)
    //     .map(|p| Ok(p?.1));
    // let polls: StdResult<Vec<_>> = polls_iter.collect(); // Chỉ thực thi tại đây
    // -> chưa unwrap
    // let polls: Vec<_> = polls_iter.collect()?; // Unwrap ra Vec, nếu có lỗi sẽ trả về lỗi ngay lập tức

    // POLLS.range(...): Trả về một iterator chứa từng mục (key, value) trong storage.
    // None, None: Không có giới hạn dưới và trên → lấy toàn bộ dữ liệu.
    // Order::Ascending: Lấy dữ liệu theo thứ tự tăng dần của key.

    // .map(|p| Ok(p?.1))
    //  p là một Result<(Key, Value), StdError>. Do .range() khi dùng với storage trả về iterator chứa phần tử Result<(key, value), StdError>
    //  p?: Nếu p là Err, thoát khỏi hàm và trả lỗi.
    // .1: Lấy phần Value (bỏ qua Key)

    // .collect::<StdResult<Vec<_>>>()?
    // Mục tiêu: Chuyển iterator thành Vec<Value>, nhưng vẫn giữ lỗi nếu có. lưu các value vào Vec
    // StdResult<Vec<_>>:
    // Ok(Vec<Value>) nếu tất cả phần tử thành công.
    // Err(StdError) nếu có lỗi.
    // ?: Nếu có lỗi, dừng và trả lỗi ngay lập tức.

    to_binary(&AllPollsResponse { polls })
}

fn query_poll(deps: Deps, env: Env, poll_id: String) -> StdResult<Binary> {
    let poll = POLLS.may_load(deps.storage, poll_id)?;

    to_binary(&PollResponse { poll })
    //unimplemented!()
}

fn query_vote(deps: Deps, env: Env, address: String, poll_id: String) -> StdResult<Binary> {
    let validated_address = deps.api.addr_validate(&address)?;
    let vote = BALLOTS.may_load(deps.storage, (validated_address, poll_id))?;
    to_binary(&VoteResponse { vote })
    //unimplemented!();
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> Result<Response, ContractError> {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{attr, from_binary};
    //module attr, helper mod
    //tạo và sử dụng các thuộc tính(attributes)
    // e.g. : ("action", "instantiate")

    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    //các hàm giả lập (mock function to mock an envirionment, message info, dependencies)
    //mock_dependencies tạo ra một đối tượng giả lập cho các Deps trong môi trường CosmWasm. Nó bao gồm các phần như bộ lưu trữ (storage), API, và querier mà hợp đồng sẽ sử dụng. Điều này giúp bạn kiểm tra các hành động như ghi dữ liệu vào bộ lưu trữ mà không cần một blockchain thật.
    //mock_env tạo ra một đối tượng giả lập cho môi trường (Env) mà hợp đồng thông minh chạy trong đó. Nó bao gồm các thông tin như thời gian, địa chỉ của người gọi, và các yếu tố khác liên quan đến môi trường thực thi.
    //mock_info giúp tạo ra thông tin giả lập cho MessageInfo, bao gồm địa chỉ người gọi và các tiền tệ gửi kèm (nếu có). Đây là đối tượng chứa các thông tin về người gửi giao dịch

    use crate::contract::{execute, instantiate}; // hàm init của contract
    use crate::msg::{
        AllPollsResponse, ExecuteMsg, InstantiateMsg, PollResponse, QueryMsg, VoteResponse,
    };

    use super::query; //

    //các account giả lập
    pub const ADDR1: &str = "cosmos1g84934fjawef9w4vl4um68mjhuhlx5xstt0whp";
    pub const ADDR2: &str = "addr2";

    #[test]
    fn test_query_all_polls() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info(ADDR1, &vec![]);

        let msg = InstantiateMsg { admin: None };
        let res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg);

        let msg = ExecuteMsg::CreatePoll {
            poll_id: "some_id_1".to_string(),
            question: "jdksjkkdj".to_string(),
            options: vec![
                "Cosmos Hub".to_string(),
                "Juno".to_string(),
                "Osmosis".to_string(),
            ],
        };
        let res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

        let msg = ExecuteMsg::CreatePoll {
            poll_id: "some_id_2".to_string(),
            question: "What's your colour?".to_string(),
            options: vec!["Red".to_string(), "Green".to_string(), "Blue".to_string()],
        };
        let _res = execute(deps.as_mut(), env.clone(), info, msg).unwrap();

        let msg = QueryMsg::AllPoll {};
        let bin = query(deps.as_ref(), env, msg).unwrap();
        let res: AllPollsResponse = from_binary(&bin).unwrap();

        assert_eq!(res.polls.len(), 2);
    }

    #[test]
    fn test_query_poll() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info(ADDR1, &vec![]);
        // Instantiate the contract
        let msg = InstantiateMsg { admin: None };
        let _res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

        // Create a poll
        let msg = ExecuteMsg::CreatePoll {
            poll_id: "some_id_1".to_string(),
            question: "What's your favourite Cosmos coin?".to_string(),
            options: vec![
                "Cosmos Hub".to_string(),
                "Juno".to_string(),
                "Osmosis".to_string(),
            ],
        };
        let _res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

        // Query for the poll that exists
        let msg = QueryMsg::Poll {
            poll_id: "some_id_1".to_string(),
        };
        let bin = query(deps.as_ref(), env.clone(), msg).unwrap();
        let res: PollResponse = from_binary(&bin).unwrap();
        // Expect a poll
        assert!(res.poll.is_some());

        // Query for the poll that does not exists
        let msg = QueryMsg::Poll {
            poll_id: "some_id_not_exist".to_string(),
        };
        let bin = query(deps.as_ref(), env.clone(), msg).unwrap();
        let res: PollResponse = from_binary(&bin).unwrap();
        // Expect none
        assert!(res.poll.is_none());
    }

    // Previous code omitted
    #[test]
    fn test_query_vote() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info(ADDR1, &vec![]);
        // Instantiate the contract
        let msg = InstantiateMsg { admin: None };
        let _res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

        // Create a poll
        let msg = ExecuteMsg::CreatePoll {
            poll_id: "some_id_1".to_string(),
            question: "What's your favourite Cosmos coin?".to_string(),
            options: vec![
                "Cosmos Hub".to_string(),
                "Juno".to_string(),
                "Osmosis".to_string(),
            ],
        };
        let _res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

        // Create a vote
        let msg = ExecuteMsg::Vote {
            poll_id: "some_id_1".to_string(),
            vote: "Juno".to_string(),
        };
        let _res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

        // Query for a vote that exists
        let msg = QueryMsg::Vote {
            poll_id: "some_id_1".to_string(),
            address: ADDR1.to_string(),
        };
        let bin = query(deps.as_ref(), env.clone(), msg).unwrap();
        let res: VoteResponse = from_binary(&bin).unwrap();
        // Expect the vote to exist
        assert!(res.vote.is_some());

        // Query for a vote that does not exists
        let msg = QueryMsg::Vote {
            poll_id: "some_id_2".to_string(),
            address: ADDR2.to_string(),
        };
        let bin = query(deps.as_ref(), env, msg).unwrap();
        let res: VoteResponse = from_binary(&bin).unwrap();
        // Expect the vote to not exist
        assert!(res.vote.is_none());
    }
    // Following code omitted

    #[test]
    fn test_instantiate() {
        //eprintln!("{} ÂSSSASAS", Addr::unchecked(ADDR1));
        let mut deps = mock_dependencies();
        //Mock the dependencies

        let env = mock_env();
        //MOck the contract environment

        let info = mock_info(ADDR1, &vec![]);
        //Mock the message info

        let msg = InstantiateMsg { admin: None };
        //Tạo message khi muốn đặt admin là người gọi

        ///eprintln!("hello1");
        let res = instantiate(deps.as_mut(), env, info, msg).unwrap();
        //deps.as_mut() -> lấy mutable reference của DepMut.
        //.unwrap() -> giải nén kết quả kiểu Result -> sẽ panic nếu có lỗi

        assert_eq!(
            res.attributes,
            vec![attr("action", "instantiate"), attr("admin", ADDR1)]
        );
        //trong Response của Cosmwasm có trường attributes là Vec<Attribute>
    }

    #[test]
    fn test_instantiate_with_admin() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let msg = InstantiateMsg {
            admin: Some(ADDR2.to_string()),
        };
        let info = mock_info(ADDR1, &vec![]);

        let res = instantiate(deps.as_mut(), env, info, msg).unwrap();

        assert_eq!(
            res.attributes,
            vec![attr("action", "instantiate"), attr("admin", ADDR2)],
        );
    }

    #[test]
    fn test_execute_create_poll_valid() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info(ADDR1, &vec![]);

        let msg = InstantiateMsg { admin: None };
        let res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

        let msg = ExecuteMsg::CreatePoll {
            poll_id: "some_id".to_string(),
            question: "What's your favourite cosmos coin?".to_string(),
            options: vec![
                "Cosmos hub".to_string(),
                "Juno".to_string(),
                "Osmosis".to_string(),
            ],
        };
        let res = execute(deps.as_mut(), env, info, msg).unwrap();
    }

    #[test]
    fn test_execute_create_poll_invalid() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info(ADDR1, &vec![]);

        let msg = InstantiateMsg { admin: None };
        let res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

        let msg = ExecuteMsg::CreatePoll {
            poll_id: "some_id".to_string(),
            question: "What's your favourite number?".to_string(),
            options: vec![
                "1".to_string(),
                "2".to_string(),
                "3".to_string(),
                "4".to_string(),
                "5".to_string(),
                "6".to_string(),
                "7".to_string(),
                "8".to_string(),
                "9".to_string(),
                "10".to_string(),
                "11".to_string(),
            ],
        };

        let err = execute(deps.as_mut(), env, info, msg).unwrap_err();
        //unwrap_err(): lấy giá trị lỗi E từ Result<T,E> -> nếu kết quả trả về Ok(T) -> panic
    }

    #[test]
    fn test_execute_vote_valid() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info(ADDR1, &vec![]);

        let msg = InstantiateMsg { admin: None };
        let res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

        //tạo poll
        let msg = ExecuteMsg::CreatePoll {
            poll_id: "some_id".to_string(),
            question: "What's your favourite Cosmos coin?".to_string(),
            options: vec![
                "Cosmos Hub".to_string(),
                "Juno".to_string(),
                "Osmosis".to_string(),
            ],
        };
        let _res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

        //vote lần đầu
        let msg = ExecuteMsg::Vote {
            poll_id: "some_id".to_string(),
            vote: "Juno".to_string(),
        };
        let _res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

        //đổi vote
        let msg = ExecuteMsg::Vote {
            poll_id: "some_id".to_string(),
            vote: "Osmosis".to_string(),
        };
        let _res = execute(deps.as_mut(), env, info, msg).unwrap();
    }

    #[test]
    fn test_execute_vote_invalid() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info(ADDR1, &vec![]);

        let msg = InstantiateMsg { admin: None };
        let res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

        let msg = ExecuteMsg::Vote {
            // tạo vote nhưng poll_id chưa có
            poll_id: "some_id".to_string(),
            vote: "Juno".to_string(),
        };

        //test error
        let _err = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap_err();
        eprintln!("invalid vote {}", _err);

        //tạo poll
        let msg = ExecuteMsg::CreatePoll {
            poll_id: "some_id".to_string(),
            question: "What's your favourite Cosmos coin?".to_string(),
            options: vec![
                "Cosmos Hub".to_string(),
                "Juno".to_string(),
                "Osmosis".to_string(),
            ],
        };
        let _res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

        //tạo vote không hợp lệ
        let msg = ExecuteMsg::Vote {
            poll_id: "some_id".to_string(),
            vote: "DVPN".to_string(),
        };
        let _err = execute(deps.as_mut(), env, info, msg).unwrap_err();
    }
}

//.as_mut() trong Rust được dùng để chuyển một Option<T> hoặc Result<T, E> thành một tham chiếu mutable (&mut T) nếu có giá trị Some(T) hoặc Ok(T), còn nếu là None hoặc Err(E), nó trả về None.

/*#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
// use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};

/*
// version info for migration info
const CONTRACT_NAME: &str = "crates.io:cw-starter";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
*/

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    unimplemented!()
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    unimplemented!()
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    unimplemented!()
}

#[cfg(test)]
mod tests {}*/
