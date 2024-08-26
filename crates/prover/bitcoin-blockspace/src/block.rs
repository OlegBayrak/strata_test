use bitcoin::{
    block::Header, consensus::Encodable, hashes::Hash, Block, BlockHash, TxMerkleNode,
    WitnessCommitment, WitnessMerkleNode,
};

use crate::{
    merkle::calculate_root,
    sha256d::sha256d,
    tx::{compute_txid, compute_wtxid},
};

/// Computes the transaction merkle root.
///
/// Equivalent to [bitcoin::Block::compute_merkle_root](https://github.com/rust-bitcoin/rust-bitcoin/blob/master/bitcoin/src/blockdata/block.rs)
pub fn compute_merkle_root(block: &Block) -> Option<[u8; 32]> {
    let hashes = block.txdata.iter().map(compute_txid);
    calculate_root(hashes)
}

/// Computes the transaction witness root.
///
/// Equivalent to [bitcoin::Block::compute_witness_root](https://github.com/rust-bitcoin/rust-bitcoin/blob/master/bitcoin/src/blockdata/block.rs)
pub fn compute_witness_root(block: &Block) -> Option<[u8; 32]> {
    let hashes = block.txdata.iter().enumerate().map(|(i, t)| {
        if i == 0 {
            // Replace the first hash with zeroes.
            [0u8; 32]
        } else {
            compute_wtxid(t)
        }
    });
    calculate_root(hashes)
}

/// Checks if Merkle root of header matches Merkle root of the transaction list.
///
/// Equivalent to [bitcoin::Block::check_merkle_root](https://github.com/rust-bitcoin/rust-bitcoin/blob/master/bitcoin/src/blockdata/block.rs)
pub fn check_merkle_root(block: &Block) -> bool {
    match compute_merkle_root(block) {
        Some(merkle_root) => block.header.merkle_root == TxMerkleNode::from_byte_array(merkle_root),
        None => false,
    }
}

/// Computes the witness commitment for the block's transaction list.
///
/// Equivalent to [bitcoin::Block::cimpute_witness_commitment](https://github.com/rust-bitcoin/rust-bitcoin/blob/master/bitcoin/src/blockdata/block.rs)
pub fn compute_witness_commitment(
    witness_root: &WitnessMerkleNode,
    witness_reserved_value: &[u8],
) -> WitnessCommitment {
    let mut vec = Vec::new();
    witness_root
        .consensus_encode(&mut vec)
        .expect("engines don't error");
    vec.extend(witness_reserved_value);
    WitnessCommitment::from_byte_array(sha256d(&vec))
}

/// Returns the block hash.
///
/// Equivalent to [bitcoin::Header::block_hash](https://github.com/rust-bitcoin/rust-bitcoin/blob/master/bitcoin/src/blockdata/block.rs)
fn compute_block_hash(header: &Header) -> [u8; 32] {
    let mut vec = Vec::with_capacity(80);
    header
        .consensus_encode(&mut vec)
        .expect("engines don't error");
    sha256d(&vec)
}

/// Checks if witness commitment in coinbase matches the transaction list.
///
/// Equivalent to [bitcoin::Block::check_witness_commitment](https://github.com/rust-bitcoin/rust-bitcoin/blob/master/bitcoin/src/blockdata/block.rs)
pub fn check_witness_commitment(block: &Block) -> bool {
    const MAGIC: [u8; 6] = [0x6a, 0x24, 0xaa, 0x21, 0xa9, 0xed];
    // Witness commitment is optional if there are no transactions using SegWit in the block.
    if block
        .txdata
        .iter()
        .all(|t| t.input.iter().all(|i| i.witness.is_empty()))
    {
        return true;
    }

    if block.txdata.is_empty() {
        return false;
    }

    let coinbase = &block.txdata[0];
    if !coinbase.is_coinbase() {
        return false;
    }

    // Commitment is in the last output that starts with magic bytes.
    if let Some(pos) = coinbase
        .output
        .iter()
        .rposition(|o| o.script_pubkey.len() >= 38 && o.script_pubkey.as_bytes()[0..6] == MAGIC)
    {
        let commitment =
            WitnessCommitment::from_slice(&coinbase.output[pos].script_pubkey.as_bytes()[6..38])
                .unwrap();
        // Witness reserved value is in coinbase input witness.
        let witness_vec: Vec<_> = coinbase.input[0].witness.iter().collect();
        if witness_vec.len() == 1 && witness_vec[0].len() == 32 {
            if let Some(witness_root) = compute_witness_root(block) {
                return commitment
                    == compute_witness_commitment(
                        &WitnessMerkleNode::from_byte_array(witness_root),
                        witness_vec[0],
                    );
            }
        }
    }
    false
}

/// Checks that the proof-of-work for the block is valid, returning a bool
pub fn check_pow(block: &Block) -> bool {
    let target = block.header.target();
    let block_hash = BlockHash::from_byte_array(compute_block_hash(&block.header));
    target.is_met_by(block_hash)
}
