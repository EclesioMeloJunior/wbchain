use super::{Epoch, EpochRandomness, ProductionAuthorities};
use crate::Node;
use merlin::Transcript;
use num::{bigint::Sign, rational::Ratio, BigInt};
use schnorrkel::{
    vrf::{VRFInOut, VRFProof},
    Keypair, PublicKey,
};

pub const SLOT_DURATION_MILLI: u32 = 5000;
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

#[derive(Debug, Clone)]
pub struct Slot(u32, u32, VRFInOut, VRFProof);

#[derive(Debug)]
pub struct Babe {
    epoch_info: Epoch,
    current_threshold: BigInt,
    current_authorities: ProductionAuthorities,

    epoch_slots: Vec<Slot>,
    claimable_slots: Vec<Slot>,
}

impl Babe {
    pub fn new(
        epoch: Epoch,
        authorities: ProductionAuthorities,
        threshold: BigInt,
        epoch_slots: Vec<Slot>,
        claimable: Vec<Slot>,
    ) -> Self {
        Babe {
            epoch_info: epoch,
            current_threshold: threshold,
            current_authorities: authorities,
            epoch_slots: epoch_slots,
            claimable_slots: claimable,
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

fn run_lottery(
    pub_key: &PublicKey,
    randomness: EpochRandomness,
    threshold: &BigInt,
    epoch_slots: Vec<Slot>,
) -> Vec<Slot> {
    let mut claimable_slots = vec![];

    for slot in epoch_slots {
        let transcript = block_production_transcript(slot.0, slot.1, randomness.clone());
        let vfr_pre_out = slot.2.to_preout();

        let vrf_in_out = vfr_pre_out.attach_input_hash(pub_key, transcript).unwrap();

        let output_bytes = vrf_in_out.make_bytes::<Uint128VRFOutput>(VRF_INOUT_CONTEXT);
        let output_value = BigInt::from_bytes_le(Sign::Plus, &output_bytes);

        if output_value.le(&threshold) {
            claimable_slots.push(slot.clone())
        }
    }

    claimable_slots
}

#[derive(Debug)]
pub struct BlockProductionEngine {
    babe: Babe,
}

impl BlockProductionEngine {
    fn handle_current_epoch(&mut self) {}
}

impl<'a> From<&mut Node<'a>> for BlockProductionEngine {
    fn from(node: &mut Node) -> Self {
        let start_at_epoch: u32 = 1;
        let first_slot = 1;

        let keypair = &node.keypair;
        let genesis = node.genesis;

        let genesis_epoch = genesis.epoch.clone();
        let production_authorities = genesis.production_authorities.clone();

        let transcripts = define_epoch_transcripts(
            start_at_epoch,
            first_slot,
            genesis_epoch.epoch_length,
            genesis_epoch.randomness.clone(),
        );

        let epoch_slots = define_epoch_slots(start_at_epoch, first_slot, transcripts, keypair);

        let threshold = calculate_threshold(
            genesis_epoch.c,
            production_authorities.0.len() as u32,
            VRF_FIRST_OUTPUT_LEN,
        );

        let claimable_slots = run_lottery(
            &keypair.public,
            genesis_epoch.randomness,
            &threshold,
            epoch_slots.clone(),
        );

        let babe = Babe::new(
            genesis.epoch.clone(),
            genesis.production_authorities.clone(),
            threshold,
            epoch_slots,
            claimable_slots,
        );
        BlockProductionEngine { babe }
    }
}

pub async fn run_block_production<'a>(node: &mut Node<'a>) -> Result<(), String> {
    println!("STARTING BLOCK PRODUCTION...");

    let block = node.genesis.create_genesis_block();
    node.genesis_hash = Some(block.header.hash::<sha2::Sha256>());

    let mut engine = BlockProductionEngine::from(node);
    engine.handle_current_epoch();

    Ok(())
}
