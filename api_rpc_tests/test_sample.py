import pytest
import httpx

BASE_URL = "https://fnclientbcc5fe8c4454c314eb0a00cd3882.devnet-annapurna.stratabtc.org"


# Helper function to send JSON-RPC requests
async def send_json_rpc_request(method: str, params: list = None) -> dict:
    payload = {
        "jsonrpc": "2.0",
        "method": method,
        "params": params or [],
        "id": 1,
    }
    async with httpx.AsyncClient() as client:
        response = await client.post(BASE_URL, json=payload)
        response.raise_for_status()
        return response.json()


@pytest.mark.asyncio
async def test_strata_protocol_version():
    """Test the strata_protocolVersion method"""
    result = await send_json_rpc_request("strata_protocolVersion")
    assert result["jsonrpc"] == "2.0"
    assert "result" in result
    assert isinstance(result["result"], int), "Protocol version should be an integer"


@pytest.mark.asyncio
async def test_strata_block_time():
    """Test the strata_blockTime method"""
    result = await send_json_rpc_request("strata_blockTime")
    assert result["jsonrpc"] == "2.0"
    assert "result" in result
    # print(result)
    assert isinstance(result["result"], int), "Block time should be an integer"
    assert result["result"] == 5000, "Block time should be 5000"


@pytest.mark.asyncio
async def test_strata_l1_connected():
    """Test the strata_l1connected method"""
    result = await send_json_rpc_request("strata_l1connected")
    assert result["jsonrpc"] == "2.0"
    assert "result" in result
    assert isinstance(result["result"], bool), "Connection status should be a boolean"
    assert result["result"] is True, "Connection is True"


@pytest.mark.asyncio
async def test_l1_status():
    """Test the strata_l1status connected method"""
    result = await send_json_rpc_request("strata_l1status")
    l1_status = result["result"]
    assert result["jsonrpc"] == "2.0"
    assert "result" in result
    assert l1_status["bitcoin_rpc_connected"] is True
    assert isinstance(l1_status["cur_height"], int), "Check that height is integer"
    assert isinstance(l1_status["cur_tip_blkid"], str), (
        "Check that address is string method"
    )
    assert len(l1_status["cur_tip_blkid"]) == 64, "Check that address is 64 digits"
    assert l1_status["last_published_txid"] is None, (
        "Check that last published tx id is None"
    )
    assert isinstance(l1_status["published_inscription_count"], int), (
        "Check that this value is integer"
    )
    assert isinstance(l1_status["last_update"], int), "Check that this value is integer"
    assert l1_status["network"] == "signet", "Check that network is correct"


@pytest.mark.asyncio
async def test_strata_getL1blockHash():
    result = await send_json_rpc_request("strata_getL1blockHash", [123456])
    assert "result" in result
    assert result["jsonrpc"] == "2.0"
    assert isinstance(result["result"], str), "Check that returned value is string"
    assert result["result"] == "c30288..010000"


@pytest.mark.asyncio
async def test_strata_client_status():
    """Test the strata_clientStatus method"""
    result = await send_json_rpc_request("strata_clientStatus")
    clientStatus = result["result"]
    assert result["jsonrpc"] == "2.0"
    assert "result" in result
    assert isinstance(result["result"], dict), "Client status should be a dictionary"
    assert "chain_tip" in result["result"], "Client status should contain 'chain_tip'"
    assert isinstance(clientStatus["chain_tip"], str)
    assert len(clientStatus["chain_tip"]) == 64
    assert isinstance(clientStatus["chain_tip_slot"], int)
    assert isinstance(clientStatus["finalized_blkid"], str)
    assert len(clientStatus["finalized_blkid"]) == 64
    assert isinstance(clientStatus["last_l1_block"], str)
    assert len(clientStatus["last_l1_block"]) == 64
    assert isinstance(clientStatus["buried_l1_height"], int)


@pytest.mark.asyncio
async def test_strata_getRecentBlockHeaders():
    """Test the strata_getRecentBlockHeaders method"""
    result = await send_json_rpc_request("strata_getRecentBlockHeaders", [5])
    records = result["result"][0]
    assert "result" in result
    assert len(result["result"]) == 5, (
        "Check that in result we have correct number of records"
    )
    assert isinstance(records["block_idx"], int)
    assert isinstance(records["timestamp"], int)
    assert isinstance(records["block_id"], str)
    assert len(records["block_id"]) == 64
    assert isinstance(records["prev_block"], str)
    assert len(records["prev_block"]) == 64
    assert isinstance(records["l1_segment_hash"], str)
    assert len(records["l1_segment_hash"]) == 64
    assert isinstance(records["exec_segment_hash"], str)
    assert len(records["exec_segment_hash"]) == 64
    assert isinstance(records["state_root"], str)
    assert len(records["state_root"]) == 64


@pytest.mark.asyncio
async def test_strata_getHeadersAtIdx():
    result = await send_json_rpc_request("strata_getHeadersAtIdx", [123456])
    records = result["result"][0]
    assert "result" in result
    assert isinstance(records["block_idx"], int)
    assert isinstance(records["timestamp"], int)
    assert isinstance(records["block_id"], str)
    assert len(records["block_id"]) == 64
    assert isinstance(records["prev_block"], str)
    assert len(records["prev_block"]) == 64
    assert isinstance(records["l1_segment_hash"], str)
    assert len(records["l1_segment_hash"]) == 64
    assert isinstance(records["exec_segment_hash"], str)
    assert len(records["exec_segment_hash"]) == 64
    assert isinstance(records["state_root"], str)
    assert len(records["state_root"]) == 64


@pytest.mark.asyncio
async def strata_getHeaderById():
    result = await send_json_rpc_request(
        "strata_getHeaderById",
        ["35a82bd5babbbd73876262c8669834dd2ef1161c18eb1c4b738a7902cc264087"],
    )
    records = result["result"][0]
    assert "result" in result
    assert isinstance(records["block_idx"], int)
    assert isinstance(records["timestamp"], int)
    assert isinstance(records["block_id"], str)
    assert len(records["block_id"]) == 64
    assert isinstance(records["prev_block"], str)
    assert len(records["prev_block"]) == 64
    assert isinstance(records["l1_segment_hash"], str)
    assert len(records["l1_segment_hash"]) == 64
    assert isinstance(records["exec_segment_hash"], str)
    assert len(records["exec_segment_hash"]) == 64
    assert isinstance(records["state_root"], str)
    assert len(records["state_root"]) == 64


@pytest.mark.asyncio
async def test_strata_getExecUpdateById():
    result = await send_json_rpc_request(
        "strata_getExecUpdateById",
        ["35a82bd5babbbd73876262c8669834dd2ef1161c18eb1c4b738a7902cc264087"],
    )
    records = result["result"]
    assert "result" in result
    assert isinstance(records["entries_root"], str)
    assert len(records["entries_root"]) == 64
    assert isinstance(records["extra_payload"], str)
    assert len(records["extra_payload"]) == 64
    assert isinstance(records["new_state"], str)
    assert len(records["new_state"]) == 64
    assert isinstance(records["withdrawals"], list)
    assert isinstance(records["da_blobs"], list)


@pytest.mark.asyncio
async def test_strata_getCLBlockWitness():
    result = await send_json_rpc_request("strata_getCLBlockWitness", [123456])
    assert "result" in result
    assert len(result["result"]) > 10


@pytest.mark.asyncio
async def test_strata_getCurrentDeposits():
    result = await send_json_rpc_request("strata_getCurrentDeposits", [])
    assert "result" in result
    assert len(result["result"]) > 10


@pytest.mark.asyncio
async def test_strata_getCurrentDepositById():
    result = await send_json_rpc_request("strata_getCurrentDepositById", [62])
    deposit = result["result"]
    assert "result" in result
    assert isinstance(deposit["deposit_idx"], int)
    assert deposit["deposit_idx"] > 0
    assert isinstance(deposit["output"], str)
    assert len(deposit["output"]) == 66
    assert isinstance(deposit["notary_operators"], list)
    assert isinstance(deposit["amt"], int)
    assert isinstance(deposit["pending_update_txs"], list)
    assert isinstance(deposit["state"], str)
    assert deposit["state"] == "accepted"


@pytest.mark.asyncio
async def test_strata_syncStatus():
    result = await send_json_rpc_request("strata_syncStatus", [])
    status = result["result"]
    assert "result" in result
    assert isinstance(status["tip_height"], int)
    assert status["tip_height"] > 0
    assert isinstance(status["tip_block_id"], str)
    assert len(status["tip_block_id"]) == 66
    assert isinstance(status["finalized_block_id"], str)
    assert len(status["finalized_block_id"]) == 66


@pytest.mark.asyncio
async def test_strata_getRawBundles():
    result = await send_json_rpc_request("strata_syncStatus", [])
    status = result["result"]
    assert "result" in result
    assert isinstance(status["tip_height"], int)
    assert status["tip_height"] > 0
    assert isinstance(status["tip_block_id"], str)
    assert len(status["tip_block_id"]) == 66
    assert isinstance(status["finalized_block_id"], str)
    assert len(status["finalized_block_id"]) == 66


@pytest.mark.asyncio
async def test_strata_getRawBundleById():
    result = await send_json_rpc_request(
        "strata_getRawBundleById",
        ["0x0043d059a0a3635a71cf7531f62c0ead1939efe1573afc5aec4068f2741b5bf2"],
    )
    assert "result" in result
    assert isinstance(result["result"], str), "'result' should be a string"
    assert len(result["result"]) > 0, "'result' should not be empty"


@pytest.mark.asyncio
async def test_strata_getBridgeMsgsByScope():
    result = await send_json_rpc_request("strata_getBridgeMsgsByScope", ["00"])
    assert "result" in result
    assert isinstance(result["result"], list)


@pytest.mark.asyncio
async def test_strata_submitBridgeMsg():
    result = await send_json_rpc_request(
        "strata_submitBridgeMsg", ["7465737420627269646765206d657373616765"]
    )
