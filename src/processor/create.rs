use borsh::BorshSerialize;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    program_option::COption,
    program_pack::Pack,
    pubkey::Pubkey,
    sysvar::{rent::Rent, Sysvar},
};
use spl_token::state::{Account, Mint};

use crate::{ferror, state::*, utils::*};

pub fn process_create(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let signer_info = next_account_info(account_info_iter)?;
    let new_proposal_info = next_account_info(account_info_iter)?; // save l, create but not init, own by program
    let authority_info = next_account_info(account_info_iter)?; // [PREFIX, program_id, new_proposal, 'authority']
    let gov_mint_info = next_account_info(account_info_iter)?;
    let gov_store_info = next_account_info(account_info_iter)?;
    let gov_payer_info = next_account_info(account_info_iter)?; // own gov token, approve for authority
    let spl_token_info = next_account_info(account_info_iter)?;
    let rent_info = next_account_info(account_info_iter)?;

    // check assert
    assert_signer(&signer_info)?;
    assert_owned_by(&new_proposal_info, &program_id)?;
    assert_rent_exempt(&Rent::from_account_info(rent_info)?, &new_proposal_info)?;
    let auth_bump = assert_proposal_authority(&program_id, &new_proposal_info, &authority_info)?;
    assert_owned_by(&gov_mint_info, &spl_token::id())?;
    assert_owned_by(&gov_store_info, &spl_token::id())?;
    assert_owned_by(&gov_payer_info, &spl_token::id())?;
    assert_eq_pubkey(&spl_token_info, &spl_token::id())?;

    let gov_mint = Mint::unpack_unchecked(&gov_mint_info.data.borrow())?;
    if gov_mint.decimals != 6 {
        return ferror!("invalid gov_mint");
    }
    // if *gov_mint_info.key.to_string() != "So11111111111111111111111111111111111111112".to_string() {
    //     return ferror!("invalid gov_mint");
    // }

    // check gov_store_info
    let gov_store: Account = assert_initialized(gov_store_info)?;
    if gov_store.amount > 0
        || gov_store.mint != *gov_mint_info.key
        || gov_store.owner != *authority_info.key
        || gov_store.delegate != COption::None
        || gov_store.close_authority != COption::None
    {
        return ferror!("invalid gov_store");
    }

    // check proposal_data
    let mut proposal_data = ProposalData::from_account_info(&new_proposal_info)?;
    if proposal_data.is_initialized() {
        return ferror!("proposal data has been initialized");
    }

    proposal_data.creator = *signer_info.key;
    proposal_data.gov_mint = *gov_mint_info.key;
    proposal_data.gov_store = *gov_store_info.key;
    proposal_data.begin_ts = now_timestamp();
    // proposal_data.duration = 7 * 24 * 60 * 60;
    proposal_data.duration = 7 * 60;
    proposal_data.power = 0;
    proposal_data.stake = 100 * 1e6 as u64;
    proposal_data.power_coefficient = 50 * 1e6 as u64;
    proposal_data.is_done = false;

    spl_token_transfer(TokenTransferParams {
        source: gov_payer_info.clone(),
        destination: gov_store_info.clone(),
        amount: proposal_data.stake,
        authority: authority_info.clone(),
        authority_signer_seeds: &[
            crate::PREFIX.as_bytes(),
            program_id.as_ref(),
            new_proposal_info.key.as_ref(),
            "authority".as_bytes(),
            &[auth_bump],
        ],
        token_program: spl_token_info.clone(),
    })?;

    proposal_data.serialize(&mut &mut new_proposal_info.data.borrow_mut()[..])?;

    Ok(())
}
