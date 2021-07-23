use borsh::{BorshDeserialize, BorshSerialize};

/// Define the type of state stored in accounts
#[derive(Clone, BorshDeserialize, BorshSerialize, PartialEq, Debug)]
pub struct GreetingAccount {
    /// number of greetings
    pub counter: u32,
}

/// Instruction definition
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
pub enum ProgramInstruction {
    /// GreetingAccount
    ///
    ///   0. `[w]` The greeting account
    GreetingAccount(GreetingAccount)
}
