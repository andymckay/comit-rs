use crypto::{digest::Digest, sha2::Sha256};
use ethereum_support::{Address as EthereumAddress, ToEthereumAddress};
use hex::FromHexError;
use secp256k1_support::KeyPair;
use std::{str::FromStr, thread::sleep, time::Duration};

mod erc20_harness;
mod ether_harness;

pub use self::{
    erc20_harness::{erc20_harness, Erc20HarnessParams},
    ether_harness::{ether_harness, EtherHarnessParams},
};
use blockchain_contracts::rfc003::{secret_hash::SecretHash, timestamp::Timestamp};

pub fn new_account(secret_key: &str) -> (KeyPair, EthereumAddress) {
    let keypair = KeyPair::from_secret_key_hex(secret_key).unwrap();
    let address = keypair.public_key().to_ethereum_address();

    (keypair, address)
}

pub const SECRET: &[u8; 32] = b"hello world, you are beautiful!!";
pub const SECRET_HASH: &'static str =
    "68d627971643a6f97f27c58957826fcba853ec2077fd10ec6b93d8e61deb4cec";

#[derive(Debug)]
pub struct CustomSizeSecret(pub Vec<u8>);

impl CustomSizeSecret {
    pub fn hash(&self) -> SecretHash {
        let mut sha = Sha256::new();
        sha.input(&self.0[..]);

        let mut result: [u8; SecretHash::LENGTH] = [0; SecretHash::LENGTH];
        sha.result(&mut result);
        SecretHash::from_str(hex::encode(result).as_str()).unwrap()
    }
}

impl FromStr for CustomSizeSecret {
    type Err = FromHexError;

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        let secret = s.as_bytes().to_vec();
        Ok(CustomSizeSecret(secret))
    }
}

fn diff(first: Timestamp, second: Timestamp) -> u32 {
    u32::from(first).checked_sub(u32::from(second)).unwrap_or(0)
}

pub fn sleep_until(timestamp: Timestamp) {
    let duration = diff(timestamp, Timestamp::now());
    let buffer = 2;

    sleep(Duration::from_secs((duration + buffer).into()));
}