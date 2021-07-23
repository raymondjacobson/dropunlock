//! Program instruction processor
use crate::{
    instruction::ProgramInstruction
};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
};

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
        
    let instruction = ProgramInstruction::try_from_slice(input)?;
    let accounts_iter = &mut accounts.iter();
    match instruction {
        ProgramInstruction::GreetingAccount(instruction) => {
            msg!("Instruction: Greet");

            // Get the account to say hello to
            let greeting_account = next_account_info(accounts_iter)?;

            // Increment and store the number of times the account has been greeted
            // let mut greeting_account = GreetingAccount::try_from_slice(&account.data.borrow())?;
            greeting_account.counter += 1;
            greeting_account.serialize(&mut &mut greeting_account.data.borrow_mut()[..])?;
            
            msg!("Said hello {} time(s)!", greeting_account.counter);
        }
    }
    
	Ok(())
}
    