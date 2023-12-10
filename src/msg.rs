use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::CustomMsg;

#[cw_serde]
pub enum ExecuteMsgExt {}

impl CustomMsg for ExecuteMsgExt {}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsgExt {}

impl CustomMsg for QueryMsgExt {}
