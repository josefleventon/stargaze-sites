use cosmwasm_schema::write_api;

use sg721::InstantiateMsg;
use sg_sites::msg::{ExecuteMsg, QueryMsg};

fn main() {
    write_api! {
        instantiate: InstantiateMsg,
        execute: ExecuteMsg,
        query: QueryMsg,
    }
}
