#![cfg(test)]

use std::vec;

use cosmwasm_std::{coin, coins, testing::mock_env, Addr, Decimal, Uint128};
use cw_multi_test::{BankSudo, Contract, ContractWrapper, Executor};
use sg_multi_test::StargazeApp;
use sg_name::NFT;
use sg_name_minter::CollectionResponse;
use sg_sites_layout::{Button, LayoutItem, NFTDisplayType, SparkDonationType, TextBoxType};
use sg_std::StargazeMsgWrapper;

use crate::{
    msg::{ExecuteMsg, InstantiateMsg},
    state::Row,
};

pub fn contract_name_marketplace() -> Box<dyn Contract<StargazeMsgWrapper>> {
    let contract = ContractWrapper::new(
        name_marketplace::execute::execute,
        name_marketplace::execute::instantiate,
        name_marketplace::query::query,
    )
    .with_sudo(name_marketplace::sudo::sudo);
    Box::new(contract)
}

pub fn contract_name_minter() -> Box<dyn Contract<StargazeMsgWrapper>> {
    let contract = ContractWrapper::new(
        name_minter::contract::execute,
        name_minter::contract::instantiate,
        name_minter::query::query,
    )
    .with_sudo(name_minter::sudo::sudo)
    .with_reply(name_minter::contract::reply);
    Box::new(contract)
}

pub fn contract_sg721_name() -> Box<dyn Contract<StargazeMsgWrapper>> {
    let contract = ContractWrapper::new(
        sg721_name::entry::execute,
        sg721_name::entry::instantiate,
        sg721_name::entry::query,
    )
    .with_sudo(sg721_name::sudo::sudo);
    Box::new(contract)
}

pub fn contract_sg_sites() -> Box<dyn Contract<StargazeMsgWrapper>> {
    let contract = ContractWrapper::new(
        crate::entry::execute,
        crate::entry::instantiate,
        crate::entry::query,
    )
    .with_sudo(crate::sudo::sudo);
    Box::new(contract)
}

const ADMIN: &str = "stars1admin";
const FEE_ACCOUNT: &str = "stars1feeacc";
// const USER: &str = "user";

// const MARKET: &str = "contract0";
const MINTER: &str = "contract1";
const COLLECTION: &str = "contract2";
const SITES: &str = "contract3";

// Initial contract setup
fn setup_contract() -> StargazeApp {
    let admin = Addr::unchecked(ADMIN);
    // let user = Addr::unchecked(USER);

    let mut router = StargazeApp::default();

    // Mint 100 ustars to the admin
    router
        .sudo(cw_multi_test::SudoMsg::Bank(BankSudo::Mint {
            to_address: ADMIN.to_string(),
            amount: coins(100, "ustars"), // 100 ustars
        }))
        .unwrap();

    // set up name_marketplace contract
    let name_marketplace_id = router.store_code(contract_name_marketplace());
    let name_marketplace_msg = name_marketplace::msg::InstantiateMsg {
        trading_fee_bps: 25u64,
        min_price: Uint128::from(50u128), // 50 ustars
        ask_interval: 0u64,
    };
    let name_marketplace_addr = router
        .instantiate_contract(
            name_marketplace_id,
            admin.clone(),
            &name_marketplace_msg,
            &[],
            "NAME_MARKETPLACE",
            None,
        )
        .unwrap();

    let sg721_name_id = router.store_code(contract_sg721_name());

    // set up name_minter contract
    let name_minter_id = router.store_code(contract_name_minter());
    let name_minter_msg = name_minter::msg::InstantiateMsg {
        admin: Some(admin.to_string()),
        verifier: None,
        collection_code_id: sg721_name_id,
        marketplace_addr: name_marketplace_addr.to_string(),
        min_name_length: 3u32,
        max_name_length: 63u32,
        base_price: Uint128::from(50u128), // 50 ustars
        fair_burn_bps: 1000u64,            // 10%
        whitelists: vec![],
    };
    let name_minter_addr = router
        .instantiate_contract(
            name_minter_id,
            admin.clone(),
            &name_minter_msg,
            &[],
            "NAME_MINTER",
            None,
        )
        .unwrap();

    // Update name_marketplace with minter address
    let msg = name_marketplace::msg::SudoMsg::UpdateNameMinter {
        minter: name_minter_addr.to_string(),
    };
    let res = router.wasm_sudo(name_marketplace_addr.clone(), &msg);
    assert!(res.is_ok());

    // Fetch sg721_name contract address from minter
    let collection: CollectionResponse = router
        .wrap()
        .query_wasm_smart(
            &name_minter_addr,
            &name_minter::msg::QueryMsg::Collection {},
        )
        .unwrap();

    let sg721_name_addr = Addr::unchecked(collection.collection);

    // Update name_marketplace with sg721_name address
    let msg = name_marketplace::msg::SudoMsg::UpdateNameCollection {
        collection: sg721_name_addr.to_string(),
    };
    let res = router.wasm_sudo(name_marketplace_addr.clone(), &msg);
    assert!(res.is_ok());

    // set up sg_sites contract
    let sg_sites_id = router.store_code(contract_sg_sites());
    router
        .instantiate_contract(
            sg_sites_id,
            admin.clone(),
            &InstantiateMsg {
                site_creation_fee: Uint128::from(50u128), // 50 ustars
                fee_account: FEE_ACCOUNT.to_string(),
                fair_burn_percent: Decimal::percent(10u64),
                sg721_name_contract_addr: sg721_name_addr.to_string(),
            },
            &[],
            "SG_SITES",
            None,
        )
        .unwrap();

    // Set minting to start
    let msg = name_minter::msg::ExecuteMsg::UpdateConfig {
        config: sg_name_minter::Config {
            public_mint_start_time: mock_env().block.time,
        },
    };
    let res = router.execute_contract(admin.clone(), name_minter_addr.clone(), &msg, &[]);
    assert!(res.is_ok());

    // Give approval to marketplace
    let approve_all_msg = sg721_name::ExecuteMsg::ApproveAll {
        operator: name_marketplace_addr.to_string(),
        expires: None,
    };
    let res = router.execute_contract(
        admin.clone(),
        sg721_name_addr.clone(),
        &approve_all_msg,
        &[],
    );
    assert!(res.is_ok());

    router
}

#[test]
fn proper_initialization() {
    setup_contract();
}

#[test]
pub fn try_create_site() {
    let mut router = setup_contract();
    let admin = Addr::unchecked(ADMIN);

    // Try to create a site without having a name
    let create_site_msg = ExecuteMsg::CreateSite {
        name: String::from("admin"),
    };

    let err = router.execute_contract(
        admin.clone(),
        Addr::unchecked(SITES),
        &create_site_msg,
        &[coin(50, "ustars")],
    );

    assert!(err.is_err());

    // Mint a name to the admin
    let mint_msg = name_minter::msg::ExecuteMsg::MintAndList {
        name: String::from("admin"),
    };

    let res = router.execute_contract(
        admin.clone(),
        Addr::unchecked(MINTER),
        &mint_msg,
        &[coin(50, "ustars")],
    );

    assert!(res.is_ok());

    // Associate the name to the admin address
    let associate_msg = sg_name::SgNameExecuteMsg::AssociateAddress {
        name: String::from("admin"),
        address: Some(admin.to_string()),
    };

    let res = router.execute_contract(
        admin.clone(),
        Addr::unchecked(COLLECTION),
        &associate_msg,
        &[],
    );

    assert!(res.is_ok());

    // Try to create a site without sending enough fees
    let create_site_msg = ExecuteMsg::CreateSite {
        name: String::from("Admin"),
    };

    let err = router.execute_contract(
        admin.clone(),
        Addr::unchecked(SITES),
        &create_site_msg,
        &[coin(25, "ustars")],
    );

    assert!(err.is_err());

    // Create a site while owning a name & paying enough fees
    let create_site_msg = ExecuteMsg::CreateSite {
        name: String::from("admin"),
    };

    let res = router.execute_contract(
        admin.clone(),
        Addr::unchecked(SITES),
        &create_site_msg,
        &[coin(50, "ustars")],
    );

    assert!(res.is_ok());
}

pub fn try_update_info() {
    let mut router = setup_contract();
    let admin = Addr::unchecked(ADMIN);
    // Mint a name to the admin
    let mint_msg = name_minter::msg::ExecuteMsg::MintAndList {
        name: String::from("admin"),
    };

    let res = router.execute_contract(
        admin.clone(),
        Addr::unchecked(MINTER),
        &mint_msg,
        &[coin(50, "ustars")],
    );

    assert!(res.is_ok());

    // Associate the name to the admin address
    let associate_msg = sg_name::SgNameExecuteMsg::AssociateAddress {
        name: String::from("admin"),
        address: Some(admin.to_string()),
    };

    let res = router.execute_contract(
        admin.clone(),
        Addr::unchecked(COLLECTION),
        &associate_msg,
        &[],
    );

    assert!(res.is_ok());

    // Create a site while owning a name & paying enough fees
    let create_site_msg = ExecuteMsg::CreateSite {
        name: String::from("Josef"),
    };

    let res = router.execute_contract(
        admin.clone(),
        Addr::unchecked(SITES),
        &create_site_msg,
        &[coin(50, "ustars")],
    );

    assert!(res.is_ok());

    // Cannot test NFTs because of lack of sg721 contract
    // Not doing it cause it would be a pain to set up
    let update_info_msg = ExecuteMsg::UpdateInfo {
        profile_picture: None,
        profile_banner: None,
        name: String::from("Jeff"),
        bio: Some(String::from("My name Jeff")),
    };

    let res = router.execute_contract(admin.clone(), Addr::unchecked(SITES), &update_info_msg, &[]);

    assert!(res.is_ok());
}

#[test]
pub fn try_update_layout() {
    let mut router = setup_contract();
    let admin = Addr::unchecked(ADMIN);
    // Mint a name to the admin
    let mint_msg = name_minter::msg::ExecuteMsg::MintAndList {
        name: String::from("admin"),
    };

    let res = router.execute_contract(
        admin.clone(),
        Addr::unchecked(MINTER),
        &mint_msg,
        &[coin(50, "ustars")],
    );

    assert!(res.is_ok());

    // Associate the name to the admin address
    let associate_msg = sg_name::SgNameExecuteMsg::AssociateAddress {
        name: String::from("admin"),
        address: Some(admin.to_string()),
    };

    let res = router.execute_contract(
        admin.clone(),
        Addr::unchecked(COLLECTION),
        &associate_msg,
        &[],
    );

    assert!(res.is_ok());

    // Create a site while owning a name & paying enough fees
    let create_site_msg = ExecuteMsg::CreateSite {
        name: String::from("admin"),
    };

    let res = router.execute_contract(
        admin.clone(),
        Addr::unchecked(SITES),
        &create_site_msg,
        &[coin(50, "ustars")],
    );

    assert!(res.is_ok());

    // Compose layout
    // This layout will have 2 rows.
    // Row 1 - 2/3 NFT display, 1/3 text box
    // Row 2 - 1/1 Spark Donation component

    // Same layout as this:
    // https://twitter.com/josefleventon_/status/1587803589753413633

    let layout = vec![
        Row {
            row: 1,
            columns: 3,
            items: vec![
                LayoutItem {
                    title: String::from("My NFTs"),
                    subtitle: None,
                    col_span: 2,
                    content: sg_sites_layout::LayoutContent::NFTDisplay(NFTDisplayType {
                        nfts: vec![
                            NFT {
                                collection: Addr::unchecked("honorstarty"),
                                token_id: String::from("565"),
                            },
                            NFT {
                                collection: Addr::unchecked("starchoadz"),
                                token_id: String::from("4514"),
                            },
                            NFT {
                                collection: Addr::unchecked("badkids"),
                                token_id: String::from("5739"),
                            },
                        ],
                    }),
                },
                LayoutItem {
                    title: String::from("About me"),
                    subtitle: None,
                    col_span: 1,
                    content: sg_sites_layout::LayoutContent::TextBox(TextBoxType {
                        text: String::from(
                            "Lorem ipsum dolor sit amet, consectetur adipiscing elit...",
                        ),
                        button: Some(Button {
                            text: String::from("Shoot me an email"),
                            href: String::from("mailto:josef.leventon@gmail.com"),
                        }),
                    }),
                },
            ],
        },
        Row {
            row: 2,
            columns: 1,
            items: vec![LayoutItem {
                title: String::from("Donate to my campaign"),
                subtitle: Some(String::from(
                    "Make a donation to my charity's campaign on SparkIBC",
                )),
                col_span: 1,
                content: sg_sites_layout::LayoutContent::SparkDonation(SparkDonationType {
                    campaign: String::from("campaign0"),
                }),
            }],
        },
    ];

    let update_layout_msg = ExecuteMsg::UpdateLayout { layout };

    let res = router.execute_contract(
        admin.clone(),
        Addr::unchecked(SITES),
        &update_layout_msg,
        &[],
    );

    assert!(res.is_ok());
}
