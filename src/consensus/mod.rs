pub mod block_production;

use serde::Deserialize;
use std::default::Default;

#[derive(Deserialize, Debug)]
pub struct EpochRandomness(pub String);

impl Default for EpochRandomness {
    fn default() -> Self {
        EpochRandomness("00000000000000000000000000000000".to_string())
    }
}

#[derive(Deserialize, Debug)]
pub struct Epoch {
    pub c: u32,
    pub epoch_length: u32,
    pub randomness: EpochRandomness,
}

impl Default for Epoch {
    fn default() -> Self {
        Epoch {
            c: 0,
            epoch_length: 10,
            randomness: Default::default(),
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct ProductionAuthorities(pub Vec<String>);

impl Default for ProductionAuthorities {
    fn default() -> Self {
        ProductionAuthorities(vec![])
    }
}
