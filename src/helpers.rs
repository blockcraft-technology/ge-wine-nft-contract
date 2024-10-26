use cosmwasm_std::{Addr, Deps, DepsMut, StdResult, Uint128};
use crate::state::BALANCES;
use crate::error::ContractError;

pub fn check_is_owner(deps: Deps, sender: &Addr, owner: &Addr) -> Result<(), ContractError> {
    if sender != owner {
        return Err(ContractError::Unauthorized {});
    }
    Ok(())
}

pub fn update_balance(
    deps: DepsMut,
    owner: Addr,
    token_id: String,
    amount: Uint128,
) -> StdResult<()> {
    let current_balance = BALANCES.may_load(deps.storage, (owner.clone(), token_id.clone()))?.unwrap_or_default();
    BALANCES.save(deps.storage, (owner, token_id), &(current_balance + amount))?;
    Ok(())
}

pub fn decrease_balance(
    deps: DepsMut,
    owner: Addr,
    token_id: String,
    amount: Uint128,
) -> Result<(), ContractError> {
    let current_balance = BALANCES.load(deps.storage, (owner.clone(), token_id.clone()))?;
    if current_balance < amount {
        return Err(ContractError::InsufficientBalance {});
    }
    BALANCES.save(deps.storage, (owner, token_id), &(current_balance - amount))?;
    Ok(())
}

pub fn validate_address(deps: Deps, address: String) -> StdResult<Addr> {
    deps.api.addr_validate(&address)
}
