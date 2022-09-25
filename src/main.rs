mod consensus;
mod genesis;
mod primitives;

use crate::genesis::Genesis;
use std::fs;

fn main() {
    let gensis_file_path = "./chain_spec/genesis.json";
    let genesis_contents =
        fs::read_to_string(String::from(gensis_file_path)).expect("failed to read genesis_file");

    let genesis = Genesis::new(genesis_contents);
    let block = genesis.create_genesis_block();

    let genesis_block_hash = block.header.hash();

    println!("{:x}", genesis_block_hash);
}
