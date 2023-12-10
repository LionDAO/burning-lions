pub mod contract;
mod error;
pub mod msg;
pub mod state;

pub use crate::error::ContractError;

type Contract<'a> = cw721_base::Cw721Contract<'a, cosmwasm_std::Empty, cosmwasm_std::Empty, msg::ExecuteMsgExt, msg::QueryMsgExt>;
