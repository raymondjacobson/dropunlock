//! Program instruction processor
// use crate::{
//     instruction::DropUnlockInstruction
// };
use crate::state::{LinkDrop};

use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
};

use borsh::{BorshDeserialize, BorshSerialize};

/// Instruction definition
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
pub enum DropUnlockInstruction {
    /// 0. `[is_signer]`
    /// 1. `[writable]` Signer's Link account'
    LockLink {
        /// Encrpyted link
        encrypted_link: String,
        /// Public key of user
        public_key: String,
        /// Amount for bounty
        amount: u64
    },

    /// 0. `[is_signer]`
    /// 1. `[writable]` Signer's Link account'
    DropLink {
        /// private key to decrypt link
        private_key: String,
        /// Retrieval account
        receiver: String
    },

    /// 0. `[is_signer]`
    /// 1. `[writable]` The user's account funding the lock link account
    /// 2. `[writable]` The lock link account
    /// 3. `[]` User's token account authority
    Fund {
        amount: u64,
    }
}

/// Instruction processor
pub fn process_instruction(
	program_id: &Pubkey, // Public key of the account the hello world program was loaded into
	accounts: &[AccountInfo], // The account to say hello to
	input: &[u8], // Ignored, all dropunlock instructions are hellos
) -> ProgramResult {
	msg!("Drop unlock program entrypoint");
    

	// Iterating accounts is safer then indexing
    
    
	// The account must be owned by the program in order to modify its data
	// if account.owner != program_id {
        //     msg!("Account does not have the correct program id");
        //     return Err(ProgramError::IncorrectProgramId);
        // }
        
    let instruction = DropUnlockInstruction::try_from_slice(input)?;
    let accounts_iter = &mut accounts.iter();
    // msg!("Instruction: Greet");
    // let greeting_account = next_account_info(accounts_iter)?;

    match instruction {
        DropUnlockInstruction::LockLink  { encrypted_link, public_key, amount } => {


            // is encrypted_link the right type?
            msg!("DropUnlockInstruction {} with key {} and escrow_pubkey: {}", encrypted_link, public_key, escrow_pubkey);
            // let mut link_drop = LinkDrop::try_from_slice(&account.data.borrow())?;

            // // Get the account to say hello to
            // let greeting_account = next_account_info(accounts_iter)?;

            // // Increment and store the number of times the account has been greeted
            // // let mut greeting_account = GreetingAccount::try_from_slice(&account.data.borrow())?;
            // greeting_account.counter += 1;
            // greeting_account.serialize(&mut &mut greeting_account.data.borrow_mut()[..])?;
            
            // msg!("Said hello {} time(s)!", greeting_account.counter);
        },
        DropUnlockInstruction::DropLink { private_key, receiver } => {
            msg!("DropUnlockInstruction {} ", private_key);

            // verify private key matches
            let secp_pubkey = PublicKey::from_secret_key(&private_key);

            
        },
        DropUnlockInstruction::Fund { amount } => {
            msg!("Fund instr {} ", amount);

            // let users_token_account_info = next_account_info(accounts_iter)?;
            // let escrow_token_account_info = next_account_info(accounts_iter)?;
            // let authority_account_info = next_account_info(account_info_iter)?;
            
            // let source_data = spl_token::state::Account::unpack(&source.data.borrow())?;
            // let pair = get_address_pair(program_id, &source_data.mint, eth_address)?;
            // if *source.key != pair.derive.address {
            //     return Err(ProgramError::InvalidSeeds);
            // }

            // let authority_signature_seeds = [&source_data.mint.to_bytes()[..32], &[pair.base.seed]];
            // let signers = &[&authority_signature_seeds[..]];

            // invoke_signed(
            //     &spl_token::instruction::transfer(
            //         &spl_token::id(),
            //         users_token_account_info.key,
            //         escrow_token_account_info.key,
            //         authority_account_info.key,
            //         &[&authority.key],
            //         amount,
            //     )?,
            //     &[users_token_account_info, escrow_token_account_info, authority],
            //     signers,
            // )
        }

    }
    
	Ok(())
}
    