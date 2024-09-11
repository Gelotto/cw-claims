use cosmwasm_std::{Addr, Response, Uint64};
use cw_storage_plus::{Item, Map};

use crate::{error::ContractError, execute::Context, models::ClaimRecord, msg::InstantiateMsg};

pub const ID_COUNTER: Item<Uint64> = Item::new("id_counter");
pub const ID_LUTAB: Map<(&String, &String, &Addr), Uint64> = Map::new("id_lutab");
pub const CLAIM_RECORDS: Map<u64, ClaimRecord> = Map::new("claim_records");
pub const ADDR_2_CLAIM_ID: Map<(&Addr, u64), ()> = Map::new("addr_2_claim_id");
pub const TIME_2_CLAIM_ID: Map<(&Addr, u64, u64), ()> = Map::new("time_2_claim_id");
pub const AMOUNT_2_CLAIM_ID: Map<(&Addr, u128, u64), ()> = Map::new("amount_2_claim_id");
pub const TOKEN_2_CLAIM_ID: Map<(&Addr, &String, u64), ()> = Map::new("token_2_claim_id");

/// Top-level initialization of contract state
pub fn init(
    _ctx: Context,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    Ok(Response::new().add_attribute("action", "instantiate"))
}
