use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Decimal, Uint128};
use cw_storage_plus::{Item, Map};
use sg_name::NFT;
use sg_sites_layout::LayoutItem;

#[cw_serde]
pub struct SudoParams {
    /// Site creation fee in ustars
    pub site_creation_fee: Uint128,
    /// Account to receive creation fees
    pub fee_account: Addr,
    /// Fair Burn fee (rest goes to fee account)
    pub fair_burn_percent: Decimal,
    /// Contract address for `sg721_name`
    pub sg721_name_contract_addr: Addr,
}

pub const SUDO_PARAMS: Item<SudoParams> = Item::new("params");

#[cw_serde]
pub struct Site {
    pub profile_picture: Option<NFT>,
    pub profile_banner: Option<NFT>,

    pub name: String,
    pub bio: Option<String>,

    pub layout: Vec<Row>,
    // pub preferred_chain: String,
}

#[cw_serde]
pub struct Row {
    pub row: u64,
    pub columns: u64,
    pub items: Vec<LayoutItem>,
}

pub const SITES: Map<&Addr, Site> = Map::new("sites");
