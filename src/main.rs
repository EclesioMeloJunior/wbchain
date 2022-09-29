mod blocktree;
mod consensus;
mod genesis;
mod primitives;

use crate::consensus::block_production::run_block_production;
use crate::genesis::Genesis;
use hex;
use schnorrkel::keys::MiniSecretKey;
use std::fs;
use std::io;
use tokio;

fn load_genesis_file(path: &str) -> io::Result<String> {
    fs::read_to_string(String::from(path))
}

#[tokio::main]
async fn main() {
    let genesis_contents: String = match load_genesis_file("./chain_spec/genesis.json") {
        Err(err) => panic!("{}", err),
        Ok(contents) => contents,
    };

    println!("contents {}", genesis_contents);

    let genesis = Genesis::new(genesis_contents);
    let start_epoch = genesis.epoch.clone();
    println!("{}", start_epoch.c);
    let start_production_authorities = genesis.production_authorities.clone();

    let block = genesis.create_genesis_block();

    let genesis_block_hash = block.header.hash::<sha2::Sha256>();

    println!("GENESIS BLOCK {:x}", genesis_block_hash);

    let dummy_keypair =
        hex::decode("e5be9a5092b81bca64be81d212e7f2f9eba183bb7a90954f7b76361f6edb5c0a").unwrap();
    let mini_secret_key = MiniSecretKey::from_bytes(&dummy_keypair[..]).unwrap();

    let keypair = mini_secret_key.expand_to_keypair(MiniSecretKey::ED25519_MODE);
    let pub_key: &[u8] = &keypair.public.to_bytes()[..];
    println!("Production Authority: 0x{}", hex::encode(pub_key));

    run_block_production(start_epoch, start_production_authorities, &keypair)
        .await
        .unwrap();
}
