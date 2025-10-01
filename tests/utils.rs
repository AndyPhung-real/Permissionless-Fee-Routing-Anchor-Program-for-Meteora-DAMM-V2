// BEGIN tests/utils.rs
use anchor_lang::prelude::*;
use anchor_client::{Client, Cluster};
use solana_sdk::signature::{Keypair, Signer};

pub fn new_client() -> (Client, Keypair) {
    let payer = Keypair::new();
    let client = Client::new_with_options(
        Cluster::Localnet,
        Rc::new(payer.clone()),
        CommitmentConfig::processed(),
    );
    (client, payer)
}
// END tests/utils.rs
