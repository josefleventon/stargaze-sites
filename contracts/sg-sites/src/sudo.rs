use crate::{
    msg::SudoMsg,
    state::{SudoParams, SUDO_PARAMS},
    ContractError,
};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Decimal, DepsMut, Env, Event, Uint128};
use sg_std::Response;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn sudo(deps: DepsMut, _env: Env, msg: SudoMsg) -> Result<Response, ContractError> {
    match msg {
        SudoMsg::UpdateParams {
            site_creation_fee,
            fee_account,
            fair_burn_percent,
            sg721_name_contract_addr,
        } => sudo_update_params(
            deps,
            site_creation_fee,
            fee_account,
            fair_burn_percent,
            sg721_name_contract_addr,
        ),
    }
}

pub fn sudo_update_params(
    deps: DepsMut,
    site_creation_fee: Uint128,
    fee_account: String,
    fair_burn_percent: Decimal,
    sg721_name_contract_addr: String,
) -> Result<Response, ContractError> {
    let api = deps.api;

    // Validate addresses
    let fee_account = api.addr_validate(&fee_account)?;
    let sg721_name_contract_addr = api.addr_validate(&sg721_name_contract_addr)?;

    // Save new params
    SUDO_PARAMS.save(
        deps.storage,
        &SudoParams {
            site_creation_fee,
            fee_account: fee_account.clone(),
            fair_burn_percent,
            sg721_name_contract_addr: sg721_name_contract_addr.clone(),
        },
    )?;

    let event = Event::new("update_params")
        .add_attribute("site_creation_fee", site_creation_fee.to_string())
        .add_attribute("fee_account", fee_account)
        .add_attribute("fair_burn_percent", fair_burn_percent.to_string())
        .add_attribute("sg721_name_contract_addr", sg721_name_contract_addr);
    Ok(Response::new().add_event(event))
}
