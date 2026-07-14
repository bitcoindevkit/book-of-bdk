#!/usr/bin/env bash
set -euo pipefail

CMD="${1:-help}"
PROJECT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

IMAGE="docker.io/bitcoindevkit/esplora:0.4.0"
CONTAINER_NAME="bdk-regtest"

# Taproot P2TR address (used by electrum example)
BDK_TR_ADDRESS="bcrt1pkar3gerekw8f9gef9vn9xz0qypytgacp9wa5saelpksdgct33qdqan7c89"
# Native SegWit P2WPKH address (used by esplora example, electrs HTTP doesn't index Taproot)
BDK_WPKH_ADDRESS="bcrt1qwk6p86mzqmstcsg99qlu2mhsp3766u68jktv6k"
EXPECTED_BALANCE_SATS=50000

RPC_HOST="127.0.0.1"
RPC_PORT=18443
ELECTRUM_PORT=60401
ESPLORA_PORT=3002

# Detect container runtime
if command -v docker &>/dev/null && docker info &>/dev/null 2>&1; then
    DOCKER=docker
elif command -v podman &>/dev/null; then
    DOCKER=podman
else
    echo "ERROR: neither docker nor podman found"
    exit 1
fi

cleanup_cookie() {
    rm -f /tmp/bdk-regtest.cookie
}

refresh_cookie() {
    cleanup_cookie
    $DOCKER cp "$CONTAINER_NAME":/root/.bitcoin/regtest/.cookie /tmp/bdk-regtest.cookie 2>/dev/null || true
}

rpc_ping() {
    if [ ! -f /tmp/bdk-regtest.cookie ]; then
        return 1
    fi
    COOKIE=$(cat /tmp/bdk-regtest.cookie 2>/dev/null) || return 1
    CU=$(echo "$COOKIE" | cut -d: -f1)
    CP=$(echo "$COOKIE" | cut -d: -f2)
    bitcoin-cli -regtest -rpcconnect="$RPC_HOST" -rpcport="$RPC_PORT" \
        -rpcuser="$CU" -rpcpassword="$CP" getblockcount 2>/dev/null
}

run_bitcoin_cli() {
    refresh_cookie
    if [ ! -f /tmp/bdk-regtest.cookie ]; then
        echo "ERROR: no cookie file available" >&2
        return 1
    fi
    COOKIE=$(cat /tmp/bdk-regtest.cookie)
    CU=$(echo "$COOKIE" | cut -d: -f1)
    CP=$(echo "$COOKIE" | cut -d: -f2)
    bitcoin-cli -regtest -rpcconnect="$RPC_HOST" -rpcport="$RPC_PORT" \
        -rpcuser="$CU" -rpcpassword="$CP" "$@"
}

up() {
    echo "=== Starting Regtest environment ==="
    cleanup_cookie

    $DOCKER run --detach --rm \
        --name "$CONTAINER_NAME" \
        -p "$RPC_HOST:$RPC_PORT-$((RPC_PORT+1)):$RPC_PORT-$((RPC_PORT+1))/tcp" \
        -p "$RPC_HOST:$ELECTRUM_PORT:$ELECTRUM_PORT/tcp" \
        -p "$RPC_HOST:$ESPLORA_PORT:$ESPLORA_PORT/tcp" \
        "$IMAGE"

    # Wait for Bitcoin Core RPC
    echo "Waiting for Bitcoin Core to start..."
    sleep 5
    for i in $(seq 1 120); do
        refresh_cookie
        if rpc_ping > /dev/null 2>&1; then
            echo "Bitcoin Core ready (height: $(rpc_ping))"
            break
        fi
        sleep 2
    done
    if ! rpc_ping > /dev/null 2>&1; then
        echo "ERROR: Bitcoin Core did not start within 240 seconds"
        $DOCKER logs "$CONTAINER_NAME" 2>/dev/null | tail -30
        exit 1
    fi

    # Wait for electrs Electrum server
    echo "Waiting for Electrum server..."
    for i in $(seq 1 30); do
        if nc -z "$RPC_HOST" "$ELECTRUM_PORT" 2>/dev/null; then
            echo "Electrum server ready"
            break
        fi
        sleep 2
    done
    if ! nc -z "$RPC_HOST" "$ELECTRUM_PORT" 2>/dev/null; then
        echo "ERROR: Electrum server did not start"
        $DOCKER logs "$CONTAINER_NAME" 2>/dev/null | tail -30
        exit 1
    fi

    # Wait for esplora HTTP server
    echo "Waiting for Esplora server..."
    for i in $(seq 1 60); do
        if curl -fsS "http://$RPC_HOST:$ESPLORA_PORT/blocks/tip/height" > /dev/null 2>&1; then
            echo "Esplora server ready (height: $(curl -fsS http://$RPC_HOST:$ESPLORA_PORT/blocks/tip/height))"
            break
        fi
        sleep 2
    done
    if ! curl -fsS "http://$RPC_HOST:$ESPLORA_PORT/blocks/tip/height" > /dev/null 2>&1; then
        echo "ERROR: Esplora server did not start"
        $DOCKER logs "$CONTAINER_NAME" 2>/dev/null | tail -30
        exit 1
    fi

    echo "=== Regtest environment is ready ==="
}

prepare() {
    echo "=== Preparing Regtest environment ==="

    if ! $DOCKER ps --format '{{.Names}}' | grep -q "^$CONTAINER_NAME$"; then
        echo "ERROR: container $CONTAINER_NAME is not running. Run '$0 up' first."
        exit 1
    fi

    refresh_cookie

    # Create bdk-test wallet if it doesn't exist
    WALLET_EXISTS=$(run_bitcoin_cli listwallets | grep -c "bdk-test" 2>/dev/null || true)
    if [ "$WALLET_EXISTS" -eq 0 ]; then
        echo "Creating bdk-test wallet..."
        run_bitcoin_cli createwallet bdk-test
    fi

    # Ensure enough mature blocks
    TIP_HEIGHT=$(run_bitcoin_cli getblockcount)
    if [ "$TIP_HEIGHT" -lt 101 ]; then
        MINER_ADDR=$(run_bitcoin_cli -rpcwallet=bdk-test getnewaddress)
        NEED=$((101 - TIP_HEIGHT))
        echo "Mining $NEED blocks to reach maturity..."
        run_bitcoin_cli -rpcwallet=bdk-test generatetoaddress "$NEED" "$MINER_ADDR"
    fi

    # Fund both BDK addresses
    fund_address() {
        local addr="$1"
        local amt="$2"
        local existing
        existing=$(run_bitcoin_cli -rpcwallet=bdk-test getreceivedbyaddress "$addr" 0 2>/dev/null || echo "0")
        if [ "$existing" = "0.00000000" ] || [ "$existing" = "0" ]; then
            echo "Funding $addr with $amt BTC..."
            FUNDING_TX=$(run_bitcoin_cli -rpcwallet=bdk-test sendtoaddress "$addr" "$amt")
            echo "Funding TX: $FUNDING_TX"
        else
            echo "$addr already has balance: $existing BTC"
        fi
    }

    fund_address "$BDK_TR_ADDRESS" 0.0005
    fund_address "$BDK_WPKH_ADDRESS" 0.0005

    # Mine confirmation blocks for any pending transactions
    MINER_ADDR=$(run_bitcoin_cli -rpcwallet=bdk-test getnewaddress)
    run_bitcoin_cli -rpcwallet=bdk-test generatetoaddress 1 "$MINER_ADDR"
    echo "Mined confirmation block"

    # Wait for indexers to catch up
    EXPECTED_HEIGHT=$(run_bitcoin_cli getblockcount)
    echo "Block height: $EXPECTED_HEIGHT"

    echo "Waiting for Esplora to index..."
    for i in $(seq 1 30); do
        ESPLORA_HEIGHT=$(curl -fsS "http://$RPC_HOST:$ESPLORA_PORT/blocks/tip/height" 2>/dev/null || echo "0")
        if [ "$ESPLORA_HEIGHT" = "$EXPECTED_HEIGHT" ]; then
            echo "Esplora indexed height $EXPECTED_HEIGHT"
            break
        fi
        sleep 2
    done

    # Wait a moment for electrs to finish indexing too
    sleep 5

    # Verify funding via esplora (P2WPKH address; electrs HTTP doesn't index Taproot scripthashes)
    VERIFIED=$(curl -fsS "http://$RPC_HOST:$ESPLORA_PORT/address/$BDK_WPKH_ADDRESS" 2>/dev/null | \
        python3 -c "import sys,json; d=json.load(sys.stdin); cs=d.get('chain_stats',{}); print(cs.get('funded_txo_count',0), cs.get('funded_txo_sum',0))" 2>/dev/null || echo "0 0")
    echo "Esplora P2WPKH address stats (tx_count funded_txo_sum): $VERIFIED sats"

    echo "=== Regtest environment prepared ==="
}

run_esplora() {
    echo "=== Running Esplora example ==="
    EXPECTED_RECEIVE_ADDRESS="$BDK_WPKH_ADDRESS" \
    EXPECTED_BALANCE_SATS="$EXPECTED_BALANCE_SATS" \
    ESPLORA_URL="http://$RPC_HOST:$ESPLORA_PORT" \
    cargo run --manifest-path "$PROJECT_DIR/examples/rust/Cargo.toml" -p esplora
    echo "=== Esplora example passed ==="
}

run_electrum() {
    echo "=== Running Electrum example ==="
    EXPECTED_RECEIVE_ADDRESS="$BDK_TR_ADDRESS" \
    EXPECTED_BALANCE_SATS="$EXPECTED_BALANCE_SATS" \
    ELECTRUM_URL="tcp://$RPC_HOST:$ELECTRUM_PORT" \
    cargo run --manifest-path "$PROJECT_DIR/examples/rust/Cargo.toml" -p electrum
    echo "=== Electrum example passed ==="
}

run() {
    run_esplora
    run_electrum
}

down() {
    echo "=== Stopping Regtest environment ==="
    cleanup_cookie
    $DOCKER kill "$CONTAINER_NAME" 2>/dev/null || true
    echo "=== Regtest environment stopped ==="
}

status() {
    if $DOCKER ps --format '{{.Names}}' 2>/dev/null | grep -q "^$CONTAINER_NAME$"; then
        echo "Container: RUNNING"
        if [ -f /tmp/bdk-regtest.cookie ]; then
            HEIGHT=$(rpc_ping 2>/dev/null || echo "unreachable")
            echo "Bitcoin Core height: $HEIGHT"
        fi
        ESPLORA_H=$(curl -fsS "http://$RPC_HOST:$ESPLORA_PORT/blocks/tip/height" 2>/dev/null || echo "unreachable")
        echo "Esplora height: $ESPLORA_H"
        if nc -z "$RPC_HOST" "$ELECTRUM_PORT" 2>/dev/null; then
            echo "Electrum port: open"
        else
            echo "Electrum port: closed"
        fi
    else
        echo "Container: STOPPED"
    fi
}

help() {
    echo "Usage: $(basename "$0") {up|prepare|run-esplora|run-electrum|run|down|status|logs|help}"
    echo ""
    echo "Commands:"
    echo "  up             Start the Regtest environment (Docker/Podman container)"
    echo "  prepare        Fund the BDK wallet ($EXPECTED_BALANCE_SATS sats)"
    echo "  run-esplora    Run the Esplora syncing example"
    echo "  run-electrum   Run the Electrum syncing example"
    echo "  run            Run all syncing examples"
    echo "  down           Stop and clean up the environment"
    echo "  status         Check environment status"
    echo "  logs           Show container logs"
}

case "$CMD" in
    up) up ;;
    prepare) prepare ;;
    run-esplora) run_esplora ;;
    run-electrum) run_electrum ;;
    run) run ;;
    down) down ;;
    status) status ;;
    logs) $DOCKER logs "$CONTAINER_NAME" 2>/dev/null | tail -50 ;;
    help|--help|-h) help ;;
    *) echo "Unknown: $CMD"; help; exit 1 ;;
esac
