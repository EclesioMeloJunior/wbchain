mod blocktree;
mod consensus;
mod genesis;
mod primitives;

use crate::consensus::block_production::run_block_production;
use crate::genesis::Genesis;
use hex;
use schnorrkel::keys::MiniSecretKey;
use schnorrkel::Keypair;
use sha2::digest::crypto_common::Key;
use std::fs;
use std::io;
use tokio;

fn load_genesis_file(path: &str) -> io::Result<String> {
    fs::read_to_string(String::from(path))
}

fn load_keypair() -> Keypair {
    let dummy_keypair =
        hex::decode("e5be9a5092b81bca64be81d212e7f2f9eba183bb7a90954f7b76361f6edb5c0a").unwrap();
    let mini_secret_key = MiniSecretKey::from_bytes(&dummy_keypair[..]).unwrap();

    let keypair = mini_secret_key.expand_to_keypair(MiniSecretKey::ED25519_MODE);
    keypair
}

pub struct Node<'a> {
    genesis: &'a Genesis,
    keypair: Keypair,
    genesis_hash: Option<sha2::digest::Output<sha2::Sha256>>,
}

impl<'a> Node<'a> {
    fn new(genesis: &'a Genesis, keypair: Keypair) -> Self {
        Node {
            genesis,
            keypair,
            genesis_hash: None,
        }
    }
}

#[tokio::main]
async fn main() {
    let genesis_contents: String = match load_genesis_file("./chain_spec/genesis.json") {
        Err(err) => panic!("{}", err),
        Ok(contents) => contents,
    };

    let genesis = Genesis::new(genesis_contents);

    let keypair = load_keypair();
    let pub_key: &[u8] = &keypair.public.to_bytes()[..];
    println!("Production Authority: 0x{}", hex::encode(pub_key));

    let mut node = Node::new(&genesis, keypair);
    run_block_production(&mut node).await.unwrap();

    println!("GENESIS BLOCK {:x}", node.genesis_hash.unwrap());
}
