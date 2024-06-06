use alpen_vertex_primitives::prelude::*;

/// Succinct commitment to relevant EL block data.
// This ended up being the same as the EL payload types in the state crate,
// should we consolidate?
#[derive(Clone, Debug)]
pub struct ExecPayloadData {
    /// Encoded EL payload, minus any operations we push to it.
    pub el_payload: Vec<u8>,

    /// CL operations pushed into the EL, such as deposits from L1.  This
    /// corresponds to the "withdrawals" field in the `ExecutionPayloadVX`
    /// type(s), but is seperated here because we control it ourselves.
    pub ops: Vec<Op>,
}

impl ExecPayloadData {
    pub fn new(el_payload: Vec<u8>, ops: Vec<Op>) -> Self {
        Self { el_payload, ops }
    }

    /// Creates a new instance with some specific payload no ops.
    pub fn new_simple(el_payload: Vec<u8>) -> Self {
        Self::new(el_payload, Vec::new())
    }
}

/// L1 withdrawal data.
#[derive(Clone, Debug)]
pub struct WithdrawData {
    /// Amount in L1 native asset.  For Bitcoin this is sats.
    amt: u64,

    /// Schnorr pubkey for the taproot output we're going to generate.
    dest_addr: Buf64,
}

/// Environment state from the CL that we pass into the EL for the payload we're
/// producing.  Maybe this should also have L1 headers or something?
#[derive(Clone, Debug)]
pub struct PayloadEnv {
    /// Timestamp we're attesting this block was created on.
    pub timestamp: u64,

    /// State root of the previous CL block.
    pub prev_global_state_root: Buf32,

    /// Safe L1 block we're exposing into the EL that's not likely to reorg.
    pub safe_l1_block: Buf32,

    /// Operations we're pushing into the EL for processing.
    pub el_ops: Vec<Op>,
}

/// Operation the CL pushes into the EL to perform as part of the block it's
/// producing.
#[derive(Clone, Debug)]
pub enum Op {
    /// Deposit some amount.
    Deposit(ELDepositData),
}

#[derive(Clone, Debug)]
pub struct ELDepositData {
    /// Amount in L1 native asset.  For Bitcoin this is sats.
    pub amt: u64,

    /// Dest addr encoded in a portable format, assumed to be valid but must be
    /// checked by EL before committing to building block.
    pub dest_addr: Vec<u8>,
}
