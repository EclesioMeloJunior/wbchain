use sha2::digest::Output;
use sha2::digest::OutputSizeUser;
use sha2::Digest;
#[derive(Debug, Copy, Clone)]
pub struct BlockNumber(pub u32);

pub const ZERO: BlockNumber = BlockNumber(0);

#[derive(Debug, Copy, Clone)]
pub struct Block {
    pub header: Header,
}

#[derive(Debug, Copy, Clone)]
pub struct Header {
    pub number: BlockNumber,
    pub sig: Sig<u32, u32, u32>,
    pub parent_hash: sha2::digest::Output<sha2::Sha256>,
}

impl Header {
    pub fn hash<T: OutputSizeUser + Digest>(&self) -> Output<T> {
        let mut hasher = T::new();

        let fst = self.sig.vrf_fst.to_le_bytes();
        let snd = self.sig.vrf_snd.to_le_bytes();
        let sig = self.sig.vrf_sig.to_le_bytes();

        let sig_values = &[fst, snd, sig].concat();

        hasher.update(&self.parent_hash);
        hasher.update(&self.number.0.to_le_bytes());
        hasher.update(sig_values);

        hasher.finalize()
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Sig<D, P, S> {
    pub vrf_fst: D,
    pub vrf_snd: P,
    pub vrf_sig: S,
}
