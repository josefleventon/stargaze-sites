use cosmwasm_std::{coin, Addr, BankMsg, Decimal, Uint128};
use sg1::fair_burn;
use sg_std::{Response, SubMsg, NATIVE_DENOM};

pub fn charge_site_creation_fees(
    res: &mut Response,
    fair_burn_percent: Decimal,
    fee: Uint128,
    fee_account: Addr,
) {
    let fair_burn_amount = fee * fair_burn_percent;
    let fee_account_amount = fee - fair_burn_amount;

    fair_burn(fair_burn_amount.u128(), None, res);

    let fee_account_send_msg = BankMsg::Send {
        to_address: fee_account.to_string(),
        amount: vec![coin(fee_account_amount.u128(), NATIVE_DENOM.to_string())],
    };

    res.messages.push(SubMsg::new(fee_account_send_msg));
}
