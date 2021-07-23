use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize, Debug, PartialEq)]
pub struct LinkDrop {
  pub encrypted_link: String,
  pub private_key: Option<String>,
  pub public_key: String,
  pub bounty: u64,
  pub balance: u64,
}
