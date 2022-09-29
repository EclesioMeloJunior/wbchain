use super::{Epoch, EpochRandomness, ProductionAuthorities};
use merlin::Transcript;
use num::{bigint::Sign, rational::Ratio, BigInt};
use schnorrkel::{
    vrf::{VRFInOut, VRFProof},
    Keypair,
};

pub const VRF_INOUT_CONTEXT: &[u8] = "wbchain_context".as_bytes();
pub const VRF_FIRST_OUTPUT_LEN: u32 = 128;

type Uint128VRFOutput = [u8; 16];

const ONE: f64 = 1.0;

fn calculate_threshold(c: f64, authorities_len: u32, l_vrf: u32) -> BigInt {
    println!(
        "CALCULATING THRESHOLD: c = {}, len = {}",
        c, authorities_len
    );

    // 1 - (1 - c) ^ 1 / len
    let p = ONE - ((ONE - c).powf(ONE / authorities_len as f64));
    let p_rat = Ratio::from_float(p).unwrap();

    // 1 << 128
    let left_shit: BigInt = BigInt::new(Sign::Plus, vec![1]) << l_vrf;

    // 1 << 128 * 1 - (1 - c) ^ 1 / len
    let num = p_rat.numer() * left_shit;
    let t = num.checked_div(p_rat.denom()).unwrap();

    t
}

pub struct Babe {
    epoch_info: Epoch,
    current_threshold: f32,
    current_authorities: ProductionAuthorities,
}

impl Babe {
    pub fn new(epoch: Epoch, authorities: ProductionAuthorities, threshold: f32) -> Self {
        Babe {
            epoch_info: epoch,
            current_threshold: threshold,
            current_authorities: authorities,
        }
    }
}

fn block_production_transcript(
    current_epoch: u32,
    slot_number: u32,
    randomness: EpochRandomness,
) -> Transcript {
    let slot_le_bytes = &slot_number.to_le_bytes()[..];
    let current_epoch_le_bytes = &current_epoch.to_le_bytes()[..];

    let mut transcript = Transcript::new(b"BABE");
    transcript.append_message(b"slot_number", slot_le_bytes);
    transcript.append_message(b"epoch", current_epoch_le_bytes);
    transcript.append_message(b"randomness", randomness.0.as_bytes());

    transcript
}

#[derive(Debug, Clone)]
struct Slot(u32, u32, VRFInOut, VRFProof);

pub struct BlockProductionEngine;

impl BlockProductionEngine {
    fn define_epoch_slots(
        current_epoch: u32,
        first_slot: u32,
        transcripts: Vec<Transcript>,
        keypair: &Keypair,
    ) -> Vec<Slot> {
        let mut slots: Vec<Slot> = vec![];
        for (idx, transcript) in transcripts.into_iter().enumerate() {
            let (inout, proof, _) = keypair.vrf_sign(transcript);
            let slot_number = idx as u32 + first_slot;
            let slot = Slot(current_epoch, slot_number, inout, proof);
            slots.push(slot);
        }

        slots
    }

    fn define_epoch_transcripts(
        current_epoch: u32,
        first_slot: u32,
        epoch_len: u32,
        randomness: EpochRandomness,
    ) -> Vec<Transcript> {
        let mut transcripts: Vec<Transcript> = vec![];

        for n in first_slot..first_slot + epoch_len {
            let transcript = block_production_transcript(current_epoch, n, randomness.clone());
            transcripts.push(transcript);
        }

        transcripts
    }

    fn run_lottery() {}

    // start_from_genesis will take in consideration the randomness
    // set in the genesis.json file to define the slots leaders
    pub fn start_from_genesis(epoch: Epoch, authorities: ProductionAuthorities, keypair: &Keypair) {
        let start_at_epoch: u32 = 1;
        let first_slot = 1;

        let transcripts = BlockProductionEngine::define_epoch_transcripts(
            start_at_epoch,
            first_slot,
            epoch.epoch_length,
            epoch.randomness.clone(),
        );

        let epoch_slots = BlockProductionEngine::define_epoch_slots(
            start_at_epoch,
            first_slot,
            transcripts,
            keypair,
        );

        let threshold =
            calculate_threshold(epoch.c, authorities.0.len() as u32, VRF_FIRST_OUTPUT_LEN);

        for slot in epoch_slots {
            let transcript = block_production_transcript(slot.0, slot.1, epoch.randomness.clone());
            let vfr_pre_out = slot.2.to_preout();

            let vrf_in_out = vfr_pre_out
                .attach_input_hash(&keypair.public, transcript)
                .unwrap();

            let output_bytes = vrf_in_out.make_bytes::<Uint128VRFOutput>(VRF_INOUT_CONTEXT);

            let output_value = u128::from_le_bytes(output_bytes);
            println!(
                "{:?}\n\tsize: {}, number: {}, threshold: {}",
                output_bytes,
                output_bytes.len(),
                output_value,
                threshold
            );
        }

        //let mut babe = Babe::new(epoch, authorities, threshold);
    }
}

pub async fn run_block_production(
    epoch: Epoch,
    authorities: ProductionAuthorities,
    keypair: &Keypair,
) -> Result<(), String> {
    BlockProductionEngine::start_from_genesis(epoch, authorities, keypair);

    println!("STARTING BLOCK PRODUCTION...");
    //println!("Current threshold: {}", babe.current_threshold);
    Ok(())
}
