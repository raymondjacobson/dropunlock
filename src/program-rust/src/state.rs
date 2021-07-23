use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize, Debug, PartialEq)]
pub struct LinkDrop {
  pub encrypted_link: String,
  pub decrypted_link: Option<String>,
  pub public_key: Option<String>,
  // TODO: Add target sol/waudio balance
  // pub target_balance: ??,
  // TODO: Add current sol/waudio balance
  // pub current_balance: ??,
}
