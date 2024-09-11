use crate::{
    error::ContractError,
    math::{add_u128, add_u64},
    models::ClaimRecord,
    msg::UpsertMsg,
    state::{
        ADDR_2_CLAIM_ID, AMOUNT_2_CLAIM_ID, CLAIM_RECORDS, ID_COUNTER, ID_LUTAB, TIME_2_CLAIM_ID, TOKEN_2_CLAIM_ID,
    },
};
use cosmwasm_std::{attr, from_json, Response, Uint128};
use cw20::Cw20ReceiveMsg;

use super::Context;

pub fn exec_upsert_native(
    ctx: Context,
    msg: UpsertMsg,
) -> Result<Response, ContractError> {
    // Ensure that sender included the exact amount of funds in info.funds to
    // cover the sum of all individual claim amounts.
    let total_amount_required: Uint128 = msg.amounts.iter().map(|x| x.1).sum();
    if msg
        .token
        .find_in_funds(&ctx.info.funds, Some(total_amount_required))
        .is_none()
    {
        return Err(ContractError::InsufficientFunds {
            reason: "insufficient funds to cover total upserted claim amount".to_owned(),
        });
    }

    upsert(ctx, msg)
}

pub fn exec_upsert_cw20(
    ctx: Context,
    msg: Cw20ReceiveMsg,
) -> Result<Response, ContractError> {
    let Cw20ReceiveMsg {
        amount, msg: inner_msg, ..
    } = msg;

    let upsert_msg: UpsertMsg = from_json(inner_msg.as_slice())?;

    // Ensure info.sender corresponds to the expected cw20 token address
    if let Some(cw20_addr) = upsert_msg.token.get_address() {
        if ctx.info.sender != cw20_addr {
            return Err(ContractError::NotAuthorized {
                reason: "Sender addr does not match upserted token address".to_owned(),
            });
        }
    } else {
        return Err(ContractError::NotAuthorized {
            reason: "Upserted token is not a cw20".to_owned(),
        });
    }

    // Ensure total cw20 amount recieved matches total required by upserted claims
    let total_amount_required: Uint128 = upsert_msg.amounts.iter().map(|x| x.1).sum();
    if total_amount_required != amount {
        return Err(ContractError::InsufficientFunds {
            reason: "insufficient funds to cover total upserted claim amount".to_owned(),
        });
    }

    upsert(ctx, upsert_msg)
}

pub fn upsert(
    ctx: Context,
    msg: UpsertMsg,
) -> Result<Response, ContractError> {
    let Context { deps, env, .. } = ctx;
    let UpsertMsg { name, token, amounts } = msg;

    for (recipient, amount) in amounts {
        let key = (&name, &token.to_key(), &recipient);

        // Get or create claim ID
        let id = if let Some(id) = ID_LUTAB.may_load(deps.storage, key)? {
            id
        } else {
            let id = ID_COUNTER.update(deps.storage, |n| -> Result<_, ContractError> { add_u64(n, 1u64) })?;
            ID_LUTAB.save(deps.storage, key, &id)?;
            id
        };

        // Copy previous claim record, update it in storage, and return it.
        let mut prev_record: Option<ClaimRecord> = None;
        let record = CLAIM_RECORDS.update(deps.storage, id.u64(), |maybe_record| -> Result<_, ContractError> {
            if let Some(mut record) = maybe_record {
                prev_record = Some(record.clone());
                record.amount = add_u128(record.amount, amount)?;
                record.updated_at = env.block.time;
                Ok(record)
            } else {
                Ok(ClaimRecord {
                    name: name.to_owned(),
                    updated_at: env.block.time,
                    token: token.to_owned(),
                    amount,
                })
            }
        })?;

        // Update lookup tables for paginating a recipient's claims by updated
        // time and amount.
        if let Some(prev_record) = prev_record {
            TIME_2_CLAIM_ID.remove(deps.storage, (&recipient, prev_record.updated_at.nanos(), id.into()));
            TIME_2_CLAIM_ID.save(deps.storage, (&recipient, record.updated_at.nanos(), id.into()), &())?;
            AMOUNT_2_CLAIM_ID.remove(deps.storage, (&recipient, prev_record.amount.u128(), id.into()));
            AMOUNT_2_CLAIM_ID.save(deps.storage, (&recipient, record.amount.u128(), id.into()), &())?;
            TOKEN_2_CLAIM_ID.save(deps.storage, (&recipient, &token.to_key(), id.into()), &())?;
            ADDR_2_CLAIM_ID.save(deps.storage, (&recipient, id.into()), &())?;
        }
    }

    Ok(Response::new().add_attributes(vec![attr("action", "upsert"), attr("name", name)]))
}
