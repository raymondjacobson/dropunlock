import * as borsh from 'borsh';
import BN from 'bn.js'
import {
  Keypair,
  Connection,
  PublicKey,
  LAMPORTS_PER_SOL,
  SystemProgram,
  TransactionInstruction,
  Transaction,
  sendAndConfirmTransaction,
} from '@solana/web3.js';

// Rust code in state for reference
// use borsh::{BorshDeserialize, BorshSerialize};

// #[derive(BorshSerialize, BorshDeserialize, Debug, PartialEq)]
// pub struct LinkDrop {
//   pub encrypted_link: String,
//   pub private_key: Option<String>,
//   pub public_key: Option<String>,
//   // TODO: Add target sol/waudio balance
//   // pub target_balance: ??,
//   // TODO: Add current sol/waudio balance
//   // pub current_balance: ??,
// }

/**
 * The state of a greeting account managed by the hello world program
 */
 class LinkDrop {
  encrypted_link: string
  public_key: string
  private_key: string
  bounty: BN
  balance: BN

  constructor(fields: { encryptedLink: string, publicKey: string } | undefined = undefined) {
    if (fields) {
      this.encryptedLink = fields.encryptedLink;
      this.publicKey = fields.publicKey;
      this.private_key = ''
      this.bounty = new BN()
      this.balance = new BN()
    }
  }
}

/**
 * Borsh schema definition for greeting accounts
 */
const DropLinkSchema = new Map([
  [LinkDrop, 
    {
      kind: 'struct', 
      fields: [
        ['encrypted_link', 'string'],
        ['public_key', 'string'],
        ['private_key', { kind: 'option', type: 'string' }],
        ['bounty', 'u64'],
        ['balance', 'u64']
      ]
    }
  ],
]);

/**
 * The expected size of each drop link account.
 */
const DROP_LINK_SIZE = borsh.serialize(
  DropLinkSchema,
  new LinkDrop(),
).length;

export async function checkAccount(
  seed: string, 
  programId: string,
  payer: Keypair,
  connection: Connection
): Promise<void> {

  // Derive the address (public key) of a greeting account from the program so that it's easy to find later.
  const dropLinkPK = await PublicKey.createWithSeed(
    payer.publicKey,
    seed,
    programId
  );

  // Check if the greeting account has already been created
  const dropLinkAccount = await connection.getAccountInfo(dropLinkPK);
  if (dropLinkAccount === null) {
    console.log(
      'Creating drop link account',
      dropLinkPK.toBase58(),
      'to say hello to',
    );
    const lamports = await connection.getMinimumBalanceForRentExemption(
      DROP_LINK_SIZE,
    );

    const transaction = new Transaction().add(
      SystemProgram.createAccountWithSeed({
        fromPubkey: payer.publicKey,
        basePubkey: dropLinkPK,
        seed: seed,
        newAccountPubkey: dropLinkPK,
        lamports,
        space: DROP_LINK_SIZE,
        programId,
      }),
    );
    await sendAndConfirmTransaction(connection, transaction, [payer]);
  } else {
    console.log('Drop link account already exists')
  }
}
