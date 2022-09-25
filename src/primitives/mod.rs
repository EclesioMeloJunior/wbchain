#[derive(Debug)]
pub struct Block {
    pub header: Header,
}

#[derive(Debug)]
pub struct Header {
    pub number: u32,
    pub sig: Sig<u64, u64, u64>,
    pub parent_hash: sha2::digest::Output<sha2::Sha256>,
}

#[derive(Debug)]
pub struct Sig<D, P, S> {
    pub vrf_fst: D,
    pub vrf_snd: P,
    pub vrf_sig: S,
}
