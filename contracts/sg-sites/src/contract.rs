use crate::{
    error::ContractError,
    msg::{ParamsResponse, SiteResponse},
    state::{Row, Site, SITES, SUDO_PARAMS},
};
use cosmwasm_std::{
    to_binary, Deps, DepsMut, MessageInfo, QueryRequest, StdError, StdResult, WasmQuery,
};
use cw721::OwnerOfResponse;
use cw_utils::{must_pay, nonpayable};
use sg_name::{NameResponse, NFT};
use sg_sites_common::charge_site_creation_fees;
use sg_std::{Response, NATIVE_DENOM};

// use subtle_encoding::bech32;

pub fn execute_create_site(
    deps: DepsMut,
    info: MessageInfo,
    name: String,
) -> Result<Response, ContractError> {
    let payment = must_pay(&info, NATIVE_DENOM)?;
    let params = SUDO_PARAMS.load(deps.storage)?;

    // Verify that the payment amount is correct
    if payment != params.site_creation_fee {
        return Err(ContractError::Payment(
            cw_utils::PaymentError::MissingDenom(String::from(NATIVE_DENOM)),
        ));
    }

    // Cannot create new site if it already exists
    if SITES.has(deps.storage, &info.sender) {
        return Err(ContractError::SiteAlreadyExists {});
    }

    // Verify that the user owns a name
    let query_msg = sg721_name::QueryMsg::Name {
        address: info.sender.to_string(),
    };
    let res: Result<NameResponse, StdError> =
        deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
            contract_addr: params.sg721_name_contract_addr.to_string(),
            msg: to_binary(&query_msg)?,
        }));

    if let Err(err) = res {
        return Err(ContractError::Std(err));
    }

    let mut res = Response::new().add_attribute("action", "create_site");

    // Charge fees
    charge_site_creation_fees(
        &mut res,
        params.fair_burn_percent,
        payment,
        params.fee_account,
    );

    let site = Site {
        name,
        bio: None,
        profile_picture: None,
        profile_banner: None,
        layout: vec![],
    };

    // Create new site
    SITES.save(deps.storage, &info.sender, &site)?;

    Ok(res.add_attribute("site_addr", info.sender))
}

pub fn execute_update_info(
    deps: DepsMut,
    info: MessageInfo,
    profile_picture: Option<NFT>,
    profile_banner: Option<NFT>,
    name: String,
    bio: Option<String>,
) -> Result<Response, ContractError> {
    nonpayable(&info)?;

    // Verify that site exists
    if !SITES.has(deps.storage, &info.sender) {
        return Err(ContractError::SiteNotFound {});
    };

    // Verify that the sender owns the picture
    if let Some(profile_picture) = profile_picture.clone() {
        let query_msg = sg721_base::QueryMsg::OwnerOf {
            token_id: profile_picture.token_id.clone(),
            include_expired: None,
        };
        let res: Result<OwnerOfResponse, StdError> =
            deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
                contract_addr: profile_picture.collection.to_string(),
                msg: to_binary(&query_msg)?,
            }));

        if let Err(err) = res {
            return Err(ContractError::Std(err));
        }

        if res.unwrap().owner != info.sender {
            return Err(ContractError::NotNFTOwner {
                contract_address: profile_picture.collection.to_string(),
                token_id: profile_picture.token_id,
            });
        }
    }

    // Verify that the sender owns the banner
    if let Some(profile_banner) = profile_banner.clone() {
        let query_msg = sg721_base::QueryMsg::OwnerOf {
            token_id: profile_banner.token_id.clone(),
            include_expired: None,
        };
        let res: Result<OwnerOfResponse, StdError> =
            deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
                contract_addr: profile_banner.collection.to_string(),
                msg: to_binary(&query_msg)?,
            }));

        if let Err(err) = res {
            return Err(ContractError::Std(err));
        }

        if res.unwrap().owner != info.sender {
            return Err(ContractError::NotNFTOwner {
                contract_address: profile_banner.collection.to_string(),
                token_id: profile_banner.token_id,
            });
        }
    }

    // Verify that the name is 64 chars or less
    if name.len() > 64 {
        return Err(ContractError::NameTooLong {});
    }

    // Verify that the bio is 160 chars or less
    if let Some(bio) = bio.clone() {
        if bio.len() > 160 {
            return Err(ContractError::BioTooLong {});
        }
    }

    // Update the site's info
    SITES.update(deps.storage, &info.sender, |site| match site {
        Some(site) => Ok(Site {
            profile_picture,
            profile_banner,
            name,
            bio,
            layout: site.layout,
        }),
        None => Err(ContractError::SiteNotFound {}),
    })?;

    Ok(Response::new().add_attribute("action", "update_site_info"))
}

pub fn execute_update_layout(
    deps: DepsMut,
    info: MessageInfo,
    layout: Vec<Row>,
) -> Result<Response, ContractError> {
    nonpayable(&info)?;

    // Verify that site exists
    if !SITES.has(deps.storage, &info.sender) {
        return Err(ContractError::SiteNotFound {});
    };

    // Verify that the total column span of a row is the same as its maximum width
    for row in layout.clone() {
        let row_columns = row.columns;

        let mut total_columns = 0;
        for item in row.items {
            total_columns += item.col_span;
        }

        if total_columns != row_columns {
            return Err(ContractError::InvalidColumns {});
        }
    }

    // Update the site's layout
    SITES.update(deps.storage, &info.sender, |site| match site {
        Some(site) => Ok(Site {
            profile_picture: site.profile_picture,
            profile_banner: site.profile_banner,
            name: site.name,
            bio: site.bio,
            layout,
        }),
        None => Err(ContractError::SiteNotFound {}),
    })?;

    Ok(Response::new().add_attribute("action", "update_site_layout"))
}

pub fn query_site(deps: Deps, address: String) -> StdResult<SiteResponse> {
    let api = deps.api;
    let address = api.addr_validate(&address)?;

    let site = SITES.load(deps.storage, &address)?;

    Ok(SiteResponse { site })
}

pub fn query_params(deps: Deps) -> StdResult<ParamsResponse> {
    let params = SUDO_PARAMS.load(deps.storage)?;
    Ok(ParamsResponse {
        site_creation_fee: params.site_creation_fee,
        fee_account: params.fee_account.to_string(),
        fair_burn_percent: params.fair_burn_percent,
        sg721_name_contract_addr: params.sg721_name_contract_addr.to_string(),
    })
}
