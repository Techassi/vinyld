use serde::{Deserialize, Serialize};
use sqlx::Type;

#[derive(Serialize, Deserialize, Type, Clone)]
pub enum BuyCondition {
    New,
    Used,
}

#[derive(Serialize, Deserialize, Type, Clone)]
pub enum Condition {
    Mint,
    NearMint,
    VeryGoodPlus,
    VeryGood,
    Good,
    Poor,
    Generic,
}
