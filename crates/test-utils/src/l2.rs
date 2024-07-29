use alpen_express_primitives::{
    block_credential,
    buf::{Buf32, Buf64},
    params::{Params, RollupParams, RunParams},
};
use alpen_express_state::{
    block::{L2Block, L2BlockBody},
    client_state::ClientState,
    header::{L2BlockHeader, L2Header, SignedL2BlockHeader},
    id::L2BlockId,
};

use crate::ArbitraryGenerator;

pub fn gen_block(parent: Option<&SignedL2BlockHeader>) -> L2Block {
    let arb = ArbitraryGenerator::new();
    let header: L2BlockHeader = arb.generate();
    let body: L2BlockBody = arb.generate();

    let block_idx = match parent {
        Some(p) => p.blockidx() + 1,
        None => 0,
    };

    let prev_block = match parent {
        Some(p) => p.get_blockid(),
        None => L2BlockId::default(),
    };

    let header = L2BlockHeader::new(
        block_idx,
        header.timestamp(),
        prev_block,
        &body,
        *header.state_root(),
    );
    let empty_sig = Buf64::zero();
    let signed_header = SignedL2BlockHeader::new(header, empty_sig);
    L2Block::new(signed_header, body)
}

pub fn gen_l2_chain(parent: Option<SignedL2BlockHeader>, blocks_num: usize) -> Vec<L2Block> {
    let mut blocks = Vec::new();
    let mut parent = match parent {
        Some(p) => p,
        None => {
            let p = gen_block(None);
            blocks.push(p.clone());
            p.header().clone()
        }
    };

    for _ in 0..blocks_num {
        let block = gen_block(Some(&parent));
        blocks.push(block.clone());
        parent = block.header().clone()
    }

    blocks
}
pub fn gen_params() -> Params {
    Params {
        rollup: RollupParams {
            block_time: 1000,
            cred_rule: block_credential::CredRule::Unchecked,
            horizon_l1_height: 3,
            genesis_l1_height: 5,
            evm_genesis_block_hash: Buf32(
                "0x37ad61cff1367467a98cf7c54c4ac99e989f1fbb1bc1e646235e90c065c565ba"
                    .parse()
                    .unwrap(),
            ),
            evm_genesis_block_state_root: Buf32(
                "0x351714af72d74259f45cd7eab0b04527cd40e74836a45abcae50f92d919d988f"
                    .parse()
                    .unwrap(),
            ),
            l1_reorg_safe_depth: 5,
        },
        run: RunParams {
            l1_follow_distance: 3,
        },
    }
}

pub fn gen_client_state(params: Option<&Params>) -> ClientState {
    let params = match params {
        Some(p) => p,
        None => &gen_params(),
    };
    ClientState::from_genesis_params(
        params.rollup.genesis_l1_height,
        params.rollup.genesis_l1_height,
    )
}
