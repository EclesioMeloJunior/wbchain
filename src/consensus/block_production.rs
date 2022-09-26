use super::{Epoch, ProductionAuthorities};

pub const VRF_FIRST_OUTPUT_LEN: u32 = 32;

fn calculate_threshold(c: u32, authorities_len: u32, l_vrf: u32) -> u64 {
    let p = 1 - ((1 - c).pow(1 / authorities_len));
    (2u32.pow(l_vrf) * p).into()
}

pub struct Babe {
    epoch_info: Epoch,
    current_threshold: u64,
    current_authorities: ProductionAuthorities,
}

impl Babe {
    fn new(epoch: Epoch, authorities: ProductionAuthorities) -> Self {
        let threshold =
            calculate_threshold(epoch.c, authorities.0.len() as u32, VRF_FIRST_OUTPUT_LEN);
        Babe {
            epoch_info: epoch,
            current_threshold: threshold,
            current_authorities: authorities,
        }
    }
}
