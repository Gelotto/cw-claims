use std::collections::HashMap;

use crate::{
    error::ContractError,
    math::add_u128,
    models::ClaimRecord,
    msg::ClaimMsg,
    state::{ADDR_2_CLAIM_ID, AMOUNT_2_CLAIM_ID, CLAIM_RECORDS, TIME_2_CLAIM_ID, TOKEN_2_CLAIM_ID},
    token::Token,
};
use cosmwasm_std::{attr, Addr, Order, Response, Storage, SubMsg, Uint128};

use super::Context;

pub const DEFAULT_LIMIT: usize = 100;
pub const DEFAULT_SUBMSG_LIMIT: usize = 30;

pub fn exec_claim(
    ctx: Context,
    msg: ClaimMsg,
) -> Result<Response, ContractError> {
    let Context { deps, info, .. } = ctx;

    // Collect together IDs of claim records to process
    let ids = if let Some(ids) = msg.ids {
        if ids.len() > DEFAULT_LIMIT {
            return Err(ContractError::ValidationError {
                reason: format!("cannot claim more than {} records per tx", DEFAULT_LIMIT),
            });
        }

        ids
    } else {
        ADDR_2_CLAIM_ID
            .keys(deps.storage, None, None, Order::Ascending)
            .map(|r| r.unwrap().1.into())
            .take(DEFAULT_LIMIT)
            .collect()
    };

    // Storage to agg total amounts for each token type being claimed
    let mut totals: HashMap<String, (Token, Uint128)> = HashMap::with_capacity(8);

    // Try to prevent overly large transactions that would run out of gas and
    // confuse the end-user.
    if totals.len() > DEFAULT_SUBMSG_LIMIT {
        return Err(ContractError::ValidationError {
            reason: format!("cannot claim more than {} token types at per tx", DEFAULT_SUBMSG_LIMIT),
        });
    }

    // Aggregate total amounts fo each token being claimed
    for id in ids {
        if let Some(claim) = process_claim(deps.storage, &info.sender, id.into())? {
            let key = claim.token.to_key();
            if let Some(val) = totals.get_mut(&key) {
                val.1 = add_u128(val.1, claim.amount)?;
            } else {
                totals.insert(key, (claim.token, claim.amount));
            }
        }
    }

    // Craete transfer submsgs for claimed token types
    let mut transfer_submsgs: Vec<SubMsg> = Vec::with_capacity(totals.len());
    for (token, amount) in totals.values() {
        transfer_submsgs.push(token.transfer(&info.sender, *amount)?);
    }

    Ok(Response::new()
        .add_attributes(vec![attr("action", "claim"), attr("claimant", info.sender)])
        .add_submessages(transfer_submsgs))
}

/// Return claim record whilst deleting all references to it from storage.
fn process_claim(
    store: &mut dyn Storage,
    recipient: &Addr,
    id: u64,
) -> Result<Option<ClaimRecord>, ContractError> {
    Ok(if let Some(claim) = CLAIM_RECORDS.may_load(store, id)? {
        CLAIM_RECORDS.remove(store, id);
        AMOUNT_2_CLAIM_ID.remove(store, (recipient, claim.amount.u128(), id));
        TIME_2_CLAIM_ID.remove(store, (recipient, claim.updated_at.nanos(), id));
        TOKEN_2_CLAIM_ID.remove(store, (recipient, &claim.token.to_key(), id));
        ADDR_2_CLAIM_ID.remove(store, (recipient, id));
        Some(claim)
    } else {
        None
    })
}
