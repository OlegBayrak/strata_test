import * as request from "supertest"; // Import supertest for HTTP requests

const BASE_URL = "https://fnclientbcc5fe8c4454c314eb0a00cd3882.devnet-annapurna.stratabtc.org";

describe("RPC Endpoints", () => {
    describe("/rpc/strata_protocolVersion GET", () => {
        it("returns HTTP 200 and the protocol version", () => {
            return request(BASE_URL)
                .post("/")
                .send({
                    jsonrpc: "2.0",
                    method: "strata_protocolVersion",
                    params: [],
                    id: 1,
                })
                .expect(200)
                .expect((res) => {
                    expect(res.body).toStrictEqual({
                        jsonrpc: "2.0",
                        result: 1, // Replace with the actual expected value
                        id: 1,
                    });
                });
        });
    });

    describe("/rpc/strata_blockTime GET", () => {
        it("returns HTTP 200 and the block time", () => {
            return request(BASE_URL)
                .post("/")
                .send({
                    jsonrpc: "2.0",
                    method: "strata_blockTime",
                    params: [],
                    id: 1,
                })
                .expect(200)
                .expect((res) => {
                    expect(res.body).toStrictEqual({
                        jsonrpc: "2.0",
                        result: 5000,
                        id: 1,
                    });
                });
        });
    });

    describe("/rpc/strata_l1connected GET", () => {
        it("returns HTTP 200 and L1 connection status", () => {
            return request(BASE_URL)
                .post("/")
                .send({
                    jsonrpc: "2.0",
                    method: "strata_l1connected",
                    params: [],
                    id: 1,
                })
                .expect(200)
                .expect((res) => {
                    expect(res.body).toStrictEqual({
                        jsonrpc: "2.0",
                        result: true, // Replace with the actual expected value
                        id: 1,
                    });
                });
        });
    });

    describe("/rpc/strata_getL1blockHash GET", () => {
        it("returns HTTP 200 and the bitcoin block hash for a given height", () => {
            const blockHeight = 100;
            return request(BASE_URL)
                .post("/")
                .send({
                    jsonrpc: "2.0",
                    method: "strata_getL1blockHash",
                    params: [blockHeight],
                    id: 1,
                })
                .expect(200)
                .expect((res) => {
                    expect(res.body).toStrictEqual({
                        jsonrpc: "2.0",
                        result: "665082..010000", // Replace with the actual expected value
                        id: 1,
                    });
                });
        });
    });

    describe("/rpc/strata_clientStatus GET", () => {
        it("returns HTTP 200 and the client status", () => {
            return request(BASE_URL)
                .post("/")
                .send({
                    jsonrpc: "2.0",
                    method: "strata_clientStatus",
                    params: [],
                    id: 1,
                })
                .expect(200)
                .expect((res) => {
                    expect(res.body).toStrictEqual({
                        jsonrpc: "2.0",
                        result: {
                            chain_tip: "115fff3ec23e126c7df8ccb598f71bb3d2c5e284ae189326440e63d8d47e3e2f",
                            chain_tip_slot: 1623487,
                            finalized_blkid: "3d2eb64a66afa64081a2c27c8cc6c58c0a4cf4243a6fd1d8ddcd276e8b1a2ad8",
                            last_l1_block: "9b231f63881911eaf9cd493f9974e3628f01c21c67c07566ecf8f3c3dd010000",
                            buried_l1_height: 257807,
                        },
                        id: 1,
                    });
                });
        });
    });

    describe("/rpc/strata_getRecentBlockHeaders GET", () => {
        it("returns HTTP 200 and recent block headers", () => {
            const count = 5;
            return request(BASE_URL)
                .post("/")
                .send({
                    jsonrpc: "2.0",
                    method: "strata_getRecentBlockHeaders",
                    params: [count],
                    id: 1,
                })
                .expect(200)
                .expect((res) => {
                    expect(res.body).toStrictEqual({
                        jsonrpc: "2.0",
                        result: [
                            {
                                block_idx: 100,
                                timestamp: 1670000000000,
                                block_id: "665082..010000",
                                prev_block: "000def",
                                l1_segment_hash: "111xyz",
                                exec_segment_hash: "222xyz",
                                state_root: "333xyz",
                            },
                        ],
                        id: 1,
                    });
                });
        });
    });
});
