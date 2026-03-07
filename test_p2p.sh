#!/bin/bash
set -e

echo "Building Orda Node..."
cargo build --bin orda-node

echo "Starting Node 1 (Port 3000)..."
rm -rf orda_data1
PORT=3000 cargo run --bin orda-node > node1.log 2>&1 &
NODE1_PID=$!

echo "Starting Node 2 (Port 3001)..."
rm -rf orda_data2
PORT=3001 cargo run --bin orda-node > node2.log 2>&1 &
NODE2_PID=$!

echo "Waiting for mDNS discovery (10s)..."
sleep 10

echo "Sending intent to Node 1 via API..."
curl -s -X POST -H 'Content-Type: application/json' -d @temp_intent.json http://127.0.0.1:3000/intent

echo "Waiting for gossipsub propagation (3s)..."
sleep 3

echo "=== Node 1 Logs ==="
cat node1.log | grep -E "Discovered|Gossip|Mempool" || true

echo "=== Node 2 Logs ==="
cat node2.log | grep -E "Discovered|Gossip|Mempool|Remote" || true

kill -9 $NODE1_PID $NODE2_PID || true
rm -rf orda_data1 orda_data2
