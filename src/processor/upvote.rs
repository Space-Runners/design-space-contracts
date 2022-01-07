use borsh::BorshSerialize;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

use crate::{error::AppError, ferror, state::*, utils::*, PREFIX};

pub fn process_upvote(program_id: &Pubkey, accounts: &[AccountInfo], args: UpvoteArgs) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let signer_info = next_account_info(account_info_iter)?;
    let proposal_info = next_account_info(account_info_iter)?;
    let authority_info = next_account_info(account_info_iter)?;
    let vote_info = next_account_info(account_info_iter)?;
    let gov_store_info = next_account_info(account_info_iter)?;
    let gov_payer_info = next_account_info(account_info_iter)?;
    let spl_token_info = next_account_info(account_info_iter)?;
    let rent_info = next_account_info(account_info_iter)?;
    let system_info = next_account_info(account_info_iter)?;

    // check assert
    assert_signer(&signer_info)?;
    assert_owned_by(&proposal_info, &program_id)?;
    let auth_bump = assert_proposal_authority(&program_id, &proposal_info, &authority_info)?;
    let vote_bump = assert_vote_data(&program_id, &proposal_info, &signer_info, &vote_info)?;
    assert_owned_by(&gov_store_info, &spl_token::id())?;
    assert_owned_by(&gov_payer_info, &spl_token::id())?;
    assert_eq_pubkey(&spl_token_info, &spl_token::id())?;

    // check proposal
    let mut proposal_data = ProposalData::from_account_info(&proposal_info)?;
    if proposal_data.is_initialized() == false
        || proposal_data.is_ended()
        || proposal_data.creator == *signer_info.key
        || proposal_data.gov_store != *gov_store_info.key
    {
        return ferror!("invalid proposal upvote state");
    }
    if args.power <= 0 {
        return ferror!("invalid upvote power");
    }
    proposal_data.power = proposal_data
        .power
        .checked_add(args.power)
        .ok_or(AppError::CheckedCalculateFailed)?;

    // check voter
    if vote_info.data_is_empty() {
        create_or_allocate_account_raw(
            *program_id,
            vote_info,
            rent_info,
            system_info,
            signer_info,
            VoteData::LEN,
            &[
                PREFIX.as_bytes(),
                program_id.as_ref(),
                proposal_info.key.as_ref(),
                signer_info.key.as_ref(),
                "vote".as_bytes(),
                &[vote_bump],
            ],
        )?;
    }

    let mut vote_data = VoteData::from_account_info(&vote_info)?;
    vote_data.voter = *signer_info.key;
    vote_data.is_done = false;

    let vote_amount = proposal_data
        .power_coefficient
        .checked_mul(args.power)
        .ok_or(AppError::CheckedCalculateFailed)?;
    vote_data.amount = vote_data
        .amount
        .checked_add(vote_amount)
        .ok_or(AppError::CheckedCalculateFailed)?;

    spl_token_transfer(TokenTransferParams {
        source: gov_payer_info.clone(),
        destination: gov_store_info.clone(),
        amount: vote_amount,
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

    vote_data.serialize(&mut &mut vote_info.data.borrow_mut()[..])?;

    proposal_data.serialize(&mut &mut proposal_info.data.borrow_mut()[..])?;

    Ok(())
}
