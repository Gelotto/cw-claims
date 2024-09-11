use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Uint128, Uint64};

use crate::{
    models::{ClaimRecord, Config},
    token::Token,
};

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
#[derive(cw_orch::ExecuteFns)]
pub enum ExecuteMsg {
    SetConfig(Config),
}

#[cw_serde]
#[derive(cw_orch::QueryFns, QueryResponses)]
pub enum QueryMsg {
    #[returns(ConfigResponse)]
    Config {},
}

#[cw_serde]
pub struct MigrateMsg {}

#[cw_serde]
pub struct ConfigResponse(pub Config);

#[cw_serde]
pub struct UpsertMsg {
    pub name: String,
    pub token: Token,
    pub amounts: Vec<(Addr, Uint128)>,
}

#[cw_serde]
pub struct ClaimMsg {
    pub ids: Option<Vec<Uint64>>,
}

#[cw_serde]
pub enum OrderKey {
    UpdatedAt,
    Amount,
    Token,
}

#[cw_serde]
pub enum OrderKeyCursor {
    UpdatedAt((Uint64, Uint64)),
    Amount((Uint128, Uint64)),
    Token((String, Uint64)),
}

#[cw_serde]
pub struct ClaimsQueryMsg {
    pub address: Addr,
    pub order_by: OrderKey,
    pub cursor: Option<OrderKeyCursor>,
}

#[cw_serde]
pub struct ClaimsResponse {
    pub claims: Vec<ClaimRecord>,
    pub cursor: Option<OrderKeyCursor>,
}
