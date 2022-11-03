use cosmwasm_schema::cw_serde;
use sg_name::NFT;

#[cw_serde]
pub struct LayoutItem {
    pub title: String,
    pub subtitle: Option<String>,
    pub col_span: u64,
    pub content: LayoutContent,
}

#[cw_serde]
pub enum LayoutContent {
    NFTDisplay(NFTDisplayType),
    TextBox(TextBoxType),
    SparkDonation(SparkDonationType),
}

#[cw_serde]
pub struct NFTDisplayType {
    pub nfts: Vec<NFT>,
}

#[cw_serde]
pub struct TextBoxType {
    pub text: String,
    pub button: Option<Button>,
}

#[cw_serde]
pub struct SparkDonationType {
    pub campaign: String,
}

#[cw_serde]
pub struct Button {
    pub text: String,
    pub href: String,
}
