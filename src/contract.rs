#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;
use cw721_base::{ExecuteMsg, InstantiateMsg, QueryMsg};

use crate::error::ContractError;
use crate::msg::{ExecuteMsgExt, QueryMsgExt};

const CONTRACT_NAME: &str = "burning-lions";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
	deps: DepsMut,
	env: Env,
	info: MessageInfo,
	msg: InstantiateMsg,
) -> Result<Response, ContractError> {
	set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
	Ok(cw721_base::entry::instantiate(deps, env, info, msg)?)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
	deps: DepsMut,
	env: Env,
	info: MessageInfo,
	msg: ExecuteMsg<Empty, ExecuteMsgExt>,
) -> Result<Response, ContractError> {
	match msg {
		ExecuteMsg::Extension { msg } => subexecute(deps, env, info, msg),
		_ => {
			let inst = crate::Contract::default();
			Ok(inst.execute(deps, env, info, msg)?)
		}
	}
}

fn subexecute(
	_deps: DepsMut,
	_env: Env,
	_info: MessageInfo,
	msg: ExecuteMsgExt,
) -> Result<Response, ContractError> {
	match msg {}
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg<QueryMsgExt>) -> StdResult<Binary> {
	match msg {
		QueryMsg::Extension { msg } => subquery(deps, env, msg),
		_ => {
			let inst = crate::Contract::default();
			inst.query(deps, env, msg)
		}
	}
}

fn subquery(_deps: Deps, _env: Env, msg: QueryMsgExt) -> StdResult<Binary> {
	match msg {}
}

#[cfg(test)]
mod tests {}
