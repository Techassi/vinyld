use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum BuyCondition {
    New,
    Used,
}

#[derive(Serialize, Deserialize)]
pub enum Condition {
    Mint,
    NearMint,
    VeryGoodPlus,
    VeryGood,
    Good,
    Poor,
    Generic,
}
