#!/bin/bash

echo "🧪 TESTING ZK PROOF REAL INTEGRATION"
echo "====================================="

# Test 1: Health check
echo "1. Testing ZK Service Health..."
curl -s -X GET http://localhost:8003/health | jq .

# Test 2: Generate Solvency Proof
echo -e "\n2. Testing Solvency Proof Generation..."
SOLVENCY_PROOF=$(curl -s -X POST http://localhost:8003/generate \
  -H "Content-Type: application/json" \
  -d '{"proof_type": {"Solvency": {"balance": 1000, "threshold": 500}}}')

echo "Generated Proof:"
echo $SOLVENCY_PROOF | jq .

# Test 3: Verify Solvency Proof
echo -e "\n3. Testing Solvency Proof Verification..."
curl -s -X POST http://localhost:8003/verify \
  -H "Content-Type: application/json" \
  -d "{\"proof\": $SOLVENCY_PROOF}" | jq .

# Test 4: Generate Listen Proof (Real Circuit)
echo -e "\n4. Testing Listen Proof Generation (Real Circuit)..."
LISTEN_PROOF=$(curl -s -X POST http://localhost:8003/generate \
  -H "Content-Type: application/json" \
  -d '{
    "proof_type": {
      "Listen": {
        "start_time": 1000,
        "current_time": 1030,
        "end_time": 2000,
        "song_hash": "abc123",
        "user_signature": ["sig1", "sig2", "sig3"],
        "user_public_key": ["pub1", "pub2"],
        "nonce": "nonce123"
      }
    }
  }')

echo "Generated Listen Proof:"
echo $LISTEN_PROOF | jq .

# Test 5: Verify Listen Proof
echo -e "\n5. Testing Listen Proof Verification..."
curl -s -X POST http://localhost:8003/verify \
  -H "Content-Type: application/json" \
  -d "{\"proof\": $LISTEN_PROOF}" | jq .

# Test 6: Get Stats
echo -e "\n6. Testing ZK Service Stats..."
curl -s -X GET http://localhost:8003/stats | jq .

echo -e "\n✅ ZK PROOF REAL INTEGRATION TEST COMPLETED!"
