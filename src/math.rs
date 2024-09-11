use cosmwasm_std::{OverflowError, OverflowOperation, StdError, Uint128, Uint256, Uint64};

use crate::error::ContractError;

pub fn mul_u256<A: Into<Uint256>, B: Into<Uint256>>(
    a: A,
    b: B,
) -> Result<Uint256, ContractError> {
    let a: Uint256 = a.into();
    let b: Uint256 = b.into();
    a.checked_mul(b).map_err(|e| ContractError::Std(StdError::overflow(e)))
}

pub fn add_u256<A: Into<Uint256>, B: Into<Uint256>>(
    a: A,
    b: B,
) -> Result<Uint256, ContractError> {
    let a: Uint256 = a.into();
    let b: Uint256 = b.into();
    a.checked_add(b).map_err(|e| ContractError::Std(StdError::overflow(e)))
}

pub fn sub_u256<A: Into<Uint256>, B: Into<Uint256>>(
    a: A,
    b: B,
) -> Result<Uint256, ContractError> {
    let a: Uint256 = a.into();
    let b: Uint256 = b.into();
    a.checked_sub(b).map_err(|e| ContractError::Std(StdError::overflow(e)))
}

pub fn div_u256<A: Into<Uint256>, B: Into<Uint256>>(
    numerator: A,
    denominator: B,
) -> Result<Uint256, ContractError> {
    let a: Uint256 = numerator.into();
    let b: Uint256 = denominator.into();
    a.checked_div(b)
        .map_err(|e| ContractError::Std(StdError::divide_by_zero(e)))
}

pub fn add_u128<A: Into<Uint128>, B: Into<Uint128>>(
    a: A,
    b: B,
) -> Result<Uint128, ContractError> {
    let a: Uint128 = a.into();
    let b: Uint128 = b.into();
    a.checked_add(b).map_err(|e| ContractError::Std(StdError::overflow(e)))
}

pub fn sub_u128<A: Into<Uint128>, B: Into<Uint128>>(
    a: A,
    b: B,
) -> Result<Uint128, ContractError> {
    let a: Uint128 = a.into();
    let b: Uint128 = b.into();
    a.checked_sub(b).map_err(|e| ContractError::Std(StdError::overflow(e)))
}

pub fn mul_u128<A: Into<Uint128>, B: Into<Uint128>>(
    a: A,
    b: B,
) -> Result<Uint128, ContractError> {
    let a: Uint128 = a.into();
    let b: Uint128 = b.into();
    a.checked_mul(b).map_err(|e| ContractError::Std(StdError::overflow(e)))
}

pub fn div_u128<A: Into<Uint128>, B: Into<Uint128>>(
    numerator: A,
    denominator: B,
) -> Result<Uint128, ContractError> {
    let a: Uint128 = numerator.into();
    let b: Uint128 = denominator.into();
    a.checked_div(b)
        .map_err(|e| ContractError::Std(StdError::divide_by_zero(e)))
}

pub fn mul_ratio_u128<A: Into<Uint128>, B: Into<Uint128>, C: Into<Uint128>>(
    base: A,
    numerator: B,
    denominator: C,
) -> Result<Uint128, ContractError> {
    let a: Uint128 = base.into();
    let b: Uint128 = numerator.into();
    let c: Uint128 = denominator.into();
    a.checked_multiply_ratio(b, c)
        .map_err(|e| ContractError::Std(StdError::generic_err(e.to_string())))
}

pub fn mul_pct_u128<A: Into<Uint128>, B: Into<Uint128>>(
    base: A,
    numerator: B,
) -> Result<Uint128, ContractError> {
    let a: Uint128 = base.into();
    let b: Uint128 = numerator.into();
    mul_ratio_u128(a, b, 1_000_000u128)
}

pub fn add_u64<A: Into<Uint64>, B: Into<Uint64>>(
    a: A,
    b: B,
) -> Result<Uint64, ContractError> {
    let a: Uint64 = a.into();
    let b: Uint64 = b.into();
    a.checked_add(b).map_err(|e| ContractError::Std(StdError::overflow(e)))
}

pub fn add_u32(
    a: u32,
    b: u32,
) -> Result<u32, ContractError> {
    a.checked_add(b).ok_or_else(|| {
        ContractError::Std(StdError::Overflow {
            source: OverflowError::new(OverflowOperation::Add, a, b),
        })
    })
}
