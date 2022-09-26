use serde::Deserialize;
use std::default::Default;

#[derive(Deserialize, Debug)]
pub struct GenesisConsensus {
    c: u32,
    epoch_length: u32,
    randomness: String,
}

impl Default for GenesisConsensus {
    fn default() -> Self {
        GenesisConsensus {
            c: 0,
            epoch_length: 10,
            randomness: "00000000000000000000000000000000".to_string(),
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct ProductionAuthorities(Vec<String>);
impl Default for ProductionAuthorities {
    fn default() -> Self {
        ProductionAuthorities(vec![])
    }
}
