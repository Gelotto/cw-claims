use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Timestamp, Uint128};

use crate::token::Token;

#[cw_serde]
pub struct Config {}

#[cw_serde]
pub struct ClaimRecord {
    pub updated_at: Timestamp,
    pub name: String,
    pub token: Token,
    pub amount: Uint128,
}
