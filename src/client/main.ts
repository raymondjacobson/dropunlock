/**
 * Hello world
 */

import {
  establishConnection,
  establishPayer,
  checkProgram,
  createDropLink as create,
  releaseDropLink as drop,
  reportGreetings,
} from './drop_unlock';
import {
  createDropLinkAccount
} from './create_account.ts'

async function createDropLink() {
  console.log("Let's say hello to a Solana account...");

  // Establish connection to the cluster
  await establishConnection();

  // Determine who pays for the fees
  await establishPayer();

  // Check if the program has been deployed
  await checkProgram();

  // Say hello to an account
  await create('https://audius.co/rayj');

  // Find out how many times that account has been greeted
  await reportGreetings();

  console.log('Success');
}


async function releaseDropLink() {
  console.log("Running release drop link");

  // Establish connection to the cluster
  await establishConnection();

  // Determine who pays for the fees
  await establishPayer();

  // Check if the program has been deployed
  await checkProgram();

  // Say hello to an account
  await drop('https://audius.co/rayj');

  // Find out how many times that account has been greeted
  await reportGreetings();

  console.log('Success');
}

const run = async () => {
  const args = process.argv.slice(2)
  const command = args[0]
  if (command === 'create') {
    await createDropLink()
  } else if (command === 'drop') {
    await releaseDropLink()
  }
}

run().then(
  () => process.exit(),
  err => {
    console.error(err);
    process.exit(-1);
  },
);
