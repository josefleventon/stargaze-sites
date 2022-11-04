pub use crate::error::ContractError;
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;

pub mod contract;
mod error;
pub mod msg;
pub mod state;
pub mod sudo;

#[cfg(test)]
pub mod integration_tests;

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:sg-sites";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

pub type ExecuteMsg = crate::msg::ExecuteMsg;
pub type QueryMsg = crate::msg::QueryMsg;

pub mod entry {
    use crate::{
        contract::{execute_create_site, query_params, query_site},
        msg::{ExecuteMsg, InstantiateMsg},
        sudo::sudo_update_params,
    };

    use super::*;
    use cosmwasm_std::{
        to_binary, Binary, Deps, DepsMut, Empty, Env, MessageInfo, StdError, StdResult,
    };
    use cw_utils::nonpayable;
    use semver::Version;
    use sg_std::Response;

    #[cfg_attr(not(feature = "library"), entry_point)]
    pub fn instantiate(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: InstantiateMsg,
    ) -> Result<Response, ContractError> {
        cw2::set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

        // no funds should be sent to this contract
        nonpayable(&info)?;

        // Update sudo params
        sudo_update_params(
            deps,
            msg.site_creation_fee,
            msg.fee_account,
            msg.fair_burn_percent,
            msg.sg721_name_contract_addr,
        )?;

        Ok(Response::new()
            .add_attribute("action", "instantiate")
            .add_attribute("sg721_sites_addr", env.contract.address.to_string())
            .add_attribute("contract_name", CONTRACT_NAME)
            .add_attribute("contract_version", CONTRACT_VERSION))
    }

    #[cfg_attr(not(feature = "library"), entry_point)]
    pub fn execute(
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        msg: ExecuteMsg,
    ) -> Result<Response, ContractError> {
        match msg {
            ExecuteMsg::CreateSite { name } => execute_create_site(deps, info, name),
            _ => Ok(Response::new()),
        }
    }

    #[cfg_attr(not(feature = "library"), entry_point)]
    pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
        match msg {
            QueryMsg::Site { address } => to_binary(&query_site(deps, address)?),
            QueryMsg::Params {} => to_binary(&query_params(deps)?),
        }
    }

    #[cfg_attr(not(feature = "library"), entry_point)]
    pub fn migrate(deps: DepsMut, _env: Env, _msg: Empty) -> Result<Response, ContractError> {
        let ver = cw2::get_contract_version(deps.storage)?;
        // ensure we are migrating from an allowed contract
        if ver.contract != CONTRACT_NAME {
            return Err(StdError::generic_err("Can only upgrade from same type").into());
        }

        // use semver
        let version = Version::parse(&ver.version).unwrap();
        let contract_version = Version::parse(CONTRACT_VERSION).unwrap();

        if version.ge(&contract_version) {
            return Err(StdError::generic_err("Cannot upgrade from a newer version").into());
        }

        // set the new version
        cw2::set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

        // do any desired state migrations...

        Ok(Response::default())
    }
}
