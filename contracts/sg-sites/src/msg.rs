use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Decimal, Uint128};
use sg_name::NFT;

use crate::state::{Row, Site};

#[cw_serde]
#[cfg_attr(test, derive(Default))]
pub struct InstantiateMsg {
    pub site_creation_fee: Uint128,
    pub fee_account: String,
    pub fair_burn_bps: u64,
    pub sg721_name_contract_addr: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    /// Create a new site (requires a fee)
    CreateSite { name: String },
    /// Update site info (pfp, banner, etc)
    UpdateInfo {
        profile_picture: Option<NFT>,
        profile_banner: Option<NFT>,

        name: String,
        bio: Option<String>,
    },
    /// Update site layout (NFT display, donation component, etc)
    UpdateLayout { layout: Vec<Row> },
}

#[cw_serde]
pub enum SudoMsg {
    UpdateParams {
        site_creation_fee: Uint128,
        fee_account: String,
        fair_burn_percent: Decimal,
        sg721_name_contract_addr: String,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    /// Return a site object
    #[returns(SiteResponse)]
    Site { address: String },

    /// Query SudoParams
    #[returns(ParamsResponse)]
    Params {},
}

#[cw_serde]
pub struct ParamsResponse {
    pub site_creation_fee: Uint128,
    pub fee_account: String,
    pub fair_burn_percent: Decimal,
    pub sg721_name_contract_addr: String,
}

#[cw_serde]
pub struct SiteResponse {
    pub site: Site,
}
