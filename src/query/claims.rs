use std::marker::PhantomData;

use cosmwasm_std::{Addr, Order, Uint64};
use cw_storage_plus::Bound;

use crate::{
    error::ContractError,
    models::ClaimRecord,
    msg::{ClaimsQueryMsg, ClaimsResponse, OrderKey, OrderKeyCursor},
    state::{AMOUNT_2_CLAIM_ID, CLAIM_RECORDS, TIME_2_CLAIM_ID, TOKEN_2_CLAIM_ID},
};

use super::ReadonlyContext;

pub const DEFAULT_LIMIT: usize = 100;

pub fn query_claims(
    ctx: ReadonlyContext,
    msg: ClaimsQueryMsg,
) -> Result<ClaimsResponse, ContractError> {
    let ClaimsQueryMsg {
        address,
        order_by,
        cursor,
    } = msg;

    let (claims, next_cursor) = match order_by {
        OrderKey::UpdatedAt => {
            let start_from = if let Some(OrderKeyCursor::UpdatedAt((n, id))) = cursor {
                Some((n.u64(), id.into()))
            } else {
                None
            };
            claims_by_time(ctx, address, start_from)
        },
        OrderKey::Amount => {
            let start_from = if let Some(OrderKeyCursor::Amount((n, id))) = cursor {
                Some((n.into(), id.into()))
            } else {
                None
            };
            claims_by_amount(ctx, address, start_from)
        },
        OrderKey::Token => {
            let start_from = if let Some(OrderKeyCursor::Token(x)) = cursor {
                Some(x)
            } else {
                None
            };
            claims_by_token(ctx, address, start_from)
        },
    }?;

    Ok(ClaimsResponse {
        claims,
        cursor: next_cursor,
    })
}

fn claims_by_time(
    ctx: ReadonlyContext,
    address: Addr,
    start_from: Option<(u64, u64)>,
) -> Result<(Vec<ClaimRecord>, Option<OrderKeyCursor>), ContractError> {
    let ReadonlyContext { deps, .. } = ctx;
    let max_bound = if let Some((n, id)) = start_from {
        Some(Bound::Exclusive(((&address, n, id), PhantomData)))
    } else {
        Some(Bound::Inclusive(((&address, 0, 0), PhantomData)))
    };

    let mut records: Vec<ClaimRecord> = Vec::with_capacity(8);
    let mut next_cursor: Option<OrderKeyCursor> = None;

    for result in TIME_2_CLAIM_ID.range(deps.storage, None, max_bound, Order::Descending) {
        let ((_, t, id), _) = result?;
        records.push(CLAIM_RECORDS.load(deps.storage, id)?);
        next_cursor = Some(OrderKeyCursor::Amount((t.into(), id.into())));
    }

    if records.len() < DEFAULT_LIMIT {
        next_cursor = None;
    };

    Ok((records, next_cursor))
}

fn claims_by_amount(
    ctx: ReadonlyContext,
    address: Addr,
    start_from: Option<(u128, u64)>,
) -> Result<(Vec<ClaimRecord>, Option<OrderKeyCursor>), ContractError> {
    let ReadonlyContext { deps, .. } = ctx;
    let max_bound = if let Some((n, id)) = start_from {
        Some(Bound::Exclusive(((&address, n, id), PhantomData)))
    } else {
        Some(Bound::Inclusive(((&address, 0, 0), PhantomData)))
    };

    let mut records: Vec<ClaimRecord> = Vec::with_capacity(8);
    let mut next_cursor: Option<OrderKeyCursor> = None;

    for result in AMOUNT_2_CLAIM_ID.range(deps.storage, None, max_bound, Order::Descending) {
        let ((_, amount, id), _) = result?;
        records.push(CLAIM_RECORDS.load(deps.storage, id)?);
        next_cursor = Some(OrderKeyCursor::Amount((amount.into(), id.into())));
    }

    if records.len() < DEFAULT_LIMIT {
        next_cursor = None;
    };

    Ok((records, next_cursor))
}

fn claims_by_token(
    ctx: ReadonlyContext,
    address: Addr,
    start_from: Option<(String, Uint64)>,
) -> Result<(Vec<ClaimRecord>, Option<OrderKeyCursor>), ContractError> {
    let ReadonlyContext { deps, .. } = ctx;
    let mut boxed_str: Box<String> = Box::new("".to_owned());
    let max_bound = if let Some((token_key, id)) = start_from {
        *boxed_str = token_key;
        Some(Bound::Exclusive(((&address, &*boxed_str, id.u64()), PhantomData)))
    } else {
        Some(Bound::Inclusive(((&address, &*boxed_str, 0u64), PhantomData)))
    };

    let mut records: Vec<ClaimRecord> = Vec::with_capacity(8);
    let mut next_cursor: Option<OrderKeyCursor> = None;

    for result in TOKEN_2_CLAIM_ID.range(deps.storage, None, max_bound, Order::Descending) {
        let ((_, token_key, id), _) = result?;
        records.push(CLAIM_RECORDS.load(deps.storage, id)?);
        next_cursor = Some(OrderKeyCursor::Token((token_key, id.into())));
    }

    if records.len() < DEFAULT_LIMIT {
        next_cursor = None;
    };

    Ok((records, next_cursor))
}
