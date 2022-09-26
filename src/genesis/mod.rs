use crate::consensus::genesis::{GenesisConsensus, ProductionAuthorities};
use crate::primitives::{Block, Header, Sig, ZERO};
use serde::Deserialize;
use sha2::{Digest, Sha256};

#[derive(Deserialize, Debug)]
pub struct Genesis {
    #[serde(default)]
    consensus: GenesisConsensus,

    #[serde(default)]
    production_authorities: ProductionAuthorities,
}

impl Genesis {
    pub fn new(json_file: String) -> Self {
        serde_json::from_str(json_file.as_str()).unwrap()
    }

    pub fn create_genesis_block(self) -> Block {
        let mut parent_header = Sha256::new();
        parent_header.update(b"[genesis_block_parent_header]");

        let result = parent_header.finalize();

        Block {
            header: Header {
                number: ZERO,
                sig: Sig {
                    vrf_fst: 0,
                    vrf_snd: 0,
                    vrf_sig: 0,
                },
                parent_hash: result,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn genesis_json_parsing() {
        let json = r#"
        {
            "consensus": {
                "c": 1,
                "epoch_length": 10,
                "randomness": "00000000000000000000000000000000"
            },
            "production_authorities": [
                "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
                "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty",
                "5FLSigC9HGRKVhB9FiEo4Y3koPsNmBmLJbpXg2mp1hXcS59Y"
            ]
        }
        "#;

        let genesis: Genesis = serde_json::from_str(json).unwrap();
        println!("{:?}", genesis);
    }
}
