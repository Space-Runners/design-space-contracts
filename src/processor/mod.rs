use borsh::BorshDeserialize;
use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, msg, pubkey::Pubkey};

use crate::instruction::*;

pub mod create;
pub use create::*;

pub mod upvote;
pub use upvote::*;

pub mod redeem;
pub use redeem::*;

pub fn process_instruction(program_id: &Pubkey, accounts: &[AccountInfo], input: &[u8]) -> ProgramResult {
    match AppInstruction::try_from_slice(input)? {
        AppInstruction::Create => {
            msg!("Instruction: Create");
            process_create(program_id, accounts)
        }
        AppInstruction::Upvote(args) => {
            msg!("Instruction: Upvote");
            process_upvote(program_id, accounts, args)
        }
        AppInstruction::Redeem => {
            msg!("Instruction: Redeem");
            process_redeem(program_id, accounts)
        }
    }
}
