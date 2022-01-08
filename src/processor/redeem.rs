use borsh::BorshSerialize;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

use crate::{ferror, state::*, utils::*};

pub fn process_redeem(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let signer_info = next_account_info(account_info_iter)?;
    let proposal_info = next_account_info(account_info_iter)?;
    let authority_info = next_account_info(account_info_iter)?;
    let gov_store_info = next_account_info(account_info_iter)?;
    let gov_return_info = next_account_info(account_info_iter)?;
    let spl_token_info = next_account_info(account_info_iter)?;

    // check assert
    assert_signer(&signer_info)?;
    assert_owned_by(&proposal_info, &program_id)?;
    let auth_bump = assert_proposal_authority(&program_id, &proposal_info, &authority_info)?;
    assert_owned_by(&gov_store_info, &spl_token::id())?;
    assert_owned_by(&gov_return_info, &spl_token::id())?;
    assert_eq_pubkey(&spl_token_info, &spl_token::id())?;

    // check proposal
    let mut proposal_data = ProposalData::from_account_info(&proposal_info)?;
    if proposal_data.is_initialized() == false
        || proposal_data.is_ended() == false
        || proposal_data.gov_store != *gov_store_info.key
    {
        return ferror!("invalid proposal redeem state");
    }

    let redeem_amount;

    if proposal_data.creator == *signer_info.key {
        if proposal_data.is_done == true {
            return ferror!("creator has been redeemed");
        }
        redeem_amount = proposal_data.stake;

        proposal_data.is_done = true;
        proposal_data.serialize(&mut &mut proposal_info.data.borrow_mut()[..])?;
    } else {
        let vote_info = next_account_info(account_info_iter)?;
        let _ = assert_vote_data(&program_id, &proposal_info, &signer_info, &vote_info)?;

        let mut vote_data = VoteData::from_account_info(&vote_info)?;
        if vote_data.is_done == true {
            return ferror!("voter has been redeemed");
        }
        redeem_amount = vote_data.amount;

        vote_data.is_done = true;
        vote_data.serialize(&mut &mut vote_info.data.borrow_mut()[..])?;
    }

    spl_token_transfer(TokenTransferParams {
        source: gov_store_info.clone(),
        destination: gov_return_info.clone(),
        amount: redeem_amount,
        authority: authority_info.clone(),
        authority_signer_seeds: &[
            crate::PREFIX.as_bytes(),
            program_id.as_ref(),
            proposal_info.key.as_ref(),
            "authority".as_bytes(),
            &[auth_bump],
        ],
        token_program: spl_token_info.clone(),
    })?;

    Ok(())
}
