use namada_core::types::address::Address;
use namada_proof_of_stake::token::storage_key::{
    balance_key, minted_balance_key, minter_key,
};
pub use namada_token::*;

use crate::{log_string, Ctx, StorageRead, StorageWrite, TxResult};

#[allow(clippy::too_many_arguments)]
/// A token transfer that can be used in a transaction.
pub fn transfer(
    ctx: &mut Ctx,
    src: &Address,
    dest: &Address,
    token: &Address,
    amount: DenominatedAmount,
) -> TxResult {
    let amount = denom_to_amount(amount, token, ctx)?;
    if amount != Amount::default() && src != dest {
        let src_key = balance_key(token, src);
        let dest_key = balance_key(token, dest);
        let src_bal: Option<Amount> = ctx.read(&src_key)?;
        let mut src_bal = src_bal.unwrap_or_else(|| {
            log_string(format!("src {} has no balance", src_key));
            unreachable!()
        });
        src_bal.spend(&amount);
        let mut dest_bal: Amount = ctx.read(&dest_key)?.unwrap_or_default();
        dest_bal.receive(&amount);
        ctx.write(&src_key, src_bal)?;
        ctx.write(&dest_key, dest_bal)?;
    }
    Ok(())
}

/// An undenominated token transfer that can be used in a transaction.
pub fn undenominated_transfer(
    ctx: &mut Ctx,
    src: &Address,
    dest: &Address,
    token: &Address,
    amount: Amount,
) -> TxResult {
    if amount != Amount::default() && src != dest {
        let src_key = balance_key(token, src);
        let dest_key = balance_key(token, dest);
        let src_bal: Option<Amount> = ctx.read(&src_key)?;
        let mut src_bal = src_bal.unwrap_or_else(|| {
            log_string(format!("src {} has no balance", src_key));
            unreachable!()
        });
        src_bal.spend(&amount);
        let mut dest_bal: Amount = ctx.read(&dest_key)?.unwrap_or_default();
        dest_bal.receive(&amount);
        ctx.write(&src_key, src_bal)?;
        ctx.write(&dest_key, dest_bal)?;
    }
    Ok(())
}

/// Mint that can be used in a transaction.
pub fn mint(
    ctx: &mut Ctx,
    minter: &Address,
    target: &Address,
    token: &Address,
    amount: Amount,
) -> TxResult {
    let target_key = balance_key(token, target);
    let mut target_bal: Amount = ctx.read(&target_key)?.unwrap_or_default();
    target_bal.receive(&amount);

    let minted_key = minted_balance_key(token);
    let mut minted_bal: Amount = ctx.read(&minted_key)?.unwrap_or_default();
    minted_bal.receive(&amount);

    ctx.write(&target_key, target_bal)?;
    ctx.write(&minted_key, minted_bal)?;

    let minter_key = minter_key(token);
    ctx.write(&minter_key, minter)?;

    Ok(())
}

/// Burn that can be used in a transaction.
pub fn burn(
    ctx: &mut Ctx,
    target: &Address,
    token: &Address,
    amount: Amount,
) -> TxResult {
    let target_key = balance_key(token, target);
    let mut target_bal: Amount = ctx.read(&target_key)?.unwrap_or_default();
    target_bal.spend(&amount);

    // burn the minted amount
    let minted_key = minted_balance_key(token);
    let mut minted_bal: Amount = ctx.read(&minted_key)?.unwrap_or_default();
    minted_bal.spend(&amount);

    ctx.write(&target_key, target_bal)?;
    ctx.write(&minted_key, minted_bal)?;

    Ok(())
}
