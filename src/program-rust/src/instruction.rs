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
    },

    /// 0. `[is_signer]`
    /// 1. `[writable]` Signer's Link account'
    DropLink {
        /// decrypted link
        decrypted_link: String,
    }
}