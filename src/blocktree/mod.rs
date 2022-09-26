use sha2::{
    digest::{Output, OutputSizeUser},
    Digest,
};
use std::collections::BTreeMap;

use crate::primitives::Header;

pub struct BlockTree<T>
where
    T: OutputSizeUser + Digest,
{
    current_heigth: u32,
    tree: BTreeMap<Output<T>, Header>,
}

impl<T> BlockTree<T>
where
    T: OutputSizeUser + Digest,
{
    pub fn new() -> Self {
        BlockTree {
            current_heigth: 0,
            tree: BTreeMap::default(),
        }
    }
    pub fn current_height(self) -> u32 {
        self.current_heigth
    }

    pub fn insert(&mut self, key: Output<T>, value: Header) -> Result<(), String> {
        if self.tree.contains_key(&key) {
            return Err(format!("key {:?} already exists", key));
        }

        self.tree.insert(key, value);
        self.current_heigth = value.number.0;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::genesis::Genesis;
    use sha2::Sha256;

    use super::*;

    #[test]
    fn test_insert_at_block_tree() {
        let genesis = Genesis::new("{}".to_string());
        let genesis_block = genesis.create_genesis_block();

        let mut bt = BlockTree::<sha2::Sha256>::new();

        let genesis_block_hash = genesis_block.header.hash::<Sha256>();
        let _ = bt
            .insert(genesis_block.header.hash::<Sha256>(), genesis_block.header)
            .unwrap();

        let contains = bt.tree.contains_key(&genesis_block_hash);
        assert!(contains);

        // try to insert twice the same hash should return Error

        if let Err(err) = bt.insert(genesis_block.header.hash::<Sha256>(), genesis_block.header) {
            assert_eq!(err, format!("key {:?} already exists", genesis_block_hash))
        } else {
            assert!(false, "expected Error while inserting key twice");
        }
    }
}
