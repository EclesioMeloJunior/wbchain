mod blocktree;
mod consensus;
mod genesis;
mod primitives;

use crate::genesis::Genesis;
use std::fs;
use std::io;

fn load_genesis_file(path: &str) -> io::Result<String> {
    fs::read_to_string(String::from(path))
}

fn main() {
    let genesis_contents: String = match load_genesis_file("./chain_spec/genesis.json") {
        Err(err) => panic!("{}", err),
        Ok(contents) => contents,
    };

    let genesis = Genesis::new(genesis_contents);
    let block = genesis.create_genesis_block();

    let genesis_block_hash = block.header.hash::<sha2::Sha256>();

    println!("GENESIS BLOCK {:x}", genesis_block_hash);
    println!("STARTING BLOCK PRODUCTION...");

    
}
