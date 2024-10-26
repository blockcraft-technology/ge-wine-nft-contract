use cosmwasm_std::{to_binary, Addr, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint128};
use crate::error::ContractError;
use crate::msg::{ExecuteMsg, QueryMsg};
use crate::state::{OWNER};
use crate::helpers::{check_is_owner, update_balance, decrease_balance, validate_address};

pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
) -> StdResult<Response> {
    OWNER.save(deps.storage, &info.sender)?;
    Ok(Response::new().add_attribute("action", "instantiate"))
}

pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Mint { token_id, owner, amount } => mint(deps, info, token_id, owner, amount),
        ExecuteMsg::Transfer { to, token_id, amount } => transfer(deps, info, to, token_id, amount),
    }
}

pub fn mint(
    deps: DepsMut,
    info: MessageInfo,
    token_id: String,
    owner: String,
    amount: Uint128,
) -> Result<Response, ContractError> {
    let stored_owner = OWNER.load(deps.storage)?;
    check_is_owner(deps.as_ref(), &info.sender, &stored_owner)?;

    let owner_addr = validate_address(deps.as_ref(), owner)?;
    update_balance(deps, owner_addr, token_id.clone(), amount)?;

    Ok(Response::new().add_attribute("action", "mint").add_attribute("token_id", token_id))
}

pub fn transfer(
    mut deps: DepsMut,
    info: MessageInfo,
    to: String,
    token_id: String,
    amount: Uint128,
) -> Result<Response, ContractError> {
    let from_addr = info.sender.clone();
    decrease_balance(deps.branch(), from_addr.clone(), token_id.clone(), amount)?;

    let to_addr = validate_address(deps.as_ref(), to)?;
    update_balance(deps, to_addr, token_id.clone(), amount)?;

    Ok(Response::new().add_attribute("action", "transfer").add_attribute("token_id", token_id))
}
