use cosmwasm_std::StdError;
use cw_controllers::AdminError;
use cw_utils::PaymentError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0}")]
    Payment(#[from] PaymentError),

    #[error("{0}")]
    Admin(#[from] AdminError),

    #[error("NotNFTOwner: {contract_address} #{token_id}")]
    NotNFTOwner {
        contract_address: String,
        token_id: String,
    },

    #[error("NameNotFound")]
    NameNotFound {},

    #[error("SiteNotFound")]
    SiteNotFound {},

    /// Bio is max 160 chars
    #[error("BioTooLong")]
    BioTooLong {},

    /// Name is max 64 chars
    #[error("NameTooLong")]
    NameTooLong {},

    #[error("InvalidColumns")]
    InvalidColumns {},

    #[error("SiteAlreadyExists")]
    SiteAlreadyExists {},

    #[error("Unauthorized: Not contract creator or admin")]
    UnauthorizedCreatorOrAdmin {},
}
