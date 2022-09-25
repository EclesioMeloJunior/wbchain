use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct GenesisConsensus {
    c: u32,
    epoch_length: u32,
    randomness: String,
}

#[derive(Deserialize, Debug)]
pub struct ProductionAuthorities(Vec<String>);
