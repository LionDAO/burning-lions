use cosmwasm_schema::write_api;

use burning_lions::msg::{ExecuteMsgExt, QueryMsgExt};
use cosmwasm_std::Empty;
use cw721_base::{ExecuteMsg, InstantiateMsg, QueryMsg};

fn main() {
    write_api! {
        instantiate: InstantiateMsg,
        execute: ExecuteMsg<Empty, ExecuteMsgExt>,
        query: QueryMsg<QueryMsgExt>,
    }
}
