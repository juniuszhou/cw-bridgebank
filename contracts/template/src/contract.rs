#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};

use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{ITEM_NAME, MAP_NAME};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:cw-template";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    // check valid token info
    msg.validate()?;
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::SetName { name } => {
            execute_set_name(deps, env, info, name)
        }
    }
}

pub fn execute_set_name(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    name: String,
) -> Result<Response, ContractError> {
    
    // save name in single item storage
    ITEM_NAME.save(deps.storage, &name,)?;
    // update value of key name
    MAP_NAME.update(
        deps.storage,
        &name,
        |_: Option<bool>| -> StdResult<_> { Ok(true) },
    )?;

    let res = Response::new()
        .add_attribute("action", "set")
        .add_attribute("from", info.sender)
        .add_attribute("name", name)
        .add_attribute("value", "true");
    Ok(res)
}


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetName { } => to_binary(&query_name(deps)?),
    }
}

pub fn query_name(deps: Deps) -> StdResult<String> {
    let name = ITEM_NAME
        .may_load(deps.storage)?
        .unwrap_or_default();
    Ok(name)
}
