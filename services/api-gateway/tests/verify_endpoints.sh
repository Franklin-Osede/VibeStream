#!/bin/bash

# VibeStream API Verification Script
# Tests core endpoints including Auth, Music, and Payment flows.

BASE_URL="http://localhost:3000"
API_URL="${BASE_URL}/api/v1"

# Colors
GREEN='\033[0;32m'
RED='\033[0;31m'
NC='\033[0m' # No Color

echo "🚀 Starting VibeStream Endpoint Verification..."
echo "Target: $BASE_URL"

# 1. Health Check
echo -n "Checking Health... "
HEALTH_response=$(curl -s -o /dev/null -w "%{http_code}" "$BASE_URL/health")
if [ "$HEALTH_response" == "200" ]; then
    echo -e "${GREEN}OK${NC}"
else
    echo -e "${RED}FAILED ($HEALTH_response)${NC}"
    echo "Make sure the API Gateway is running!"
    exit 1
fi

# 2. Register User (Mock/Real)
echo -n "Registering Test User... "
# Use a random email to avoid collision
RANDOM_ID=$(date +%s)
EMAIL="testuser_${RANDOM_ID}@example.com"
REGISTER_PAYLOAD=$(cat <<EOF
{
  "email": "$EMAIL",
  "username": "testuser_${RANDOM_ID}",
  "password": "Password123!",
  "confirm_password": "Password123!",
  "terms_accepted": true
}
EOF
)

REGISTER_RES=$(curl -s -X POST "$API_URL/users/register" \
  -H "Content-Type: application/json" \
  -d "$REGISTER_PAYLOAD")

# Check if registration was success (or already exists)
if echo "$REGISTER_RES" | grep -q "id"; then
    echo -e "${GREEN}OK${NC}"
else
    echo -e "${RED}FAILED${NC}"
    echo "Response: $REGISTER_RES"
    # Proceeding anyway as login might work if user exists
fi

# 3. Login
echo -n "Logging In... "
LOGIN_PAYLOAD=$(cat <<EOF
{
  "credential": "$EMAIL",
  "password": "Password123!"
}
EOF
)

LOGIN_RES=$(curl -s -X POST "$API_URL/users/login" \
  -H "Content-Type: application/json" \
  -d "$LOGIN_PAYLOAD")

TOKEN=$(echo "$LOGIN_RES" | grep -o '"token":"[^"]*' | cut -d'"' -f4)

if [ -n "$TOKEN" ]; then
    echo -e "${GREEN}OK${NC}"
    echo "Token acquired."
else
    echo -e "${RED}FAILED${NC}"
    echo "Response: $LOGIN_RES"
    exit 1
fi

# 4. Initiate Payment
echo -n "Initiating Payment... "
PAYMENT_PAYLOAD=$(cat <<EOF
{
  "payer_id": "00000000-0000-0000-0000-000000000001",
  "payee_id": "00000000-0000-0000-0000-000000000002",
  "amount": 10.00,
  "currency": "USD",
  "payment_type": "OneTime",
  "payment_method": "CreditCard",
  "metadata": {
    "user_ip": "127.0.0.1",
    "platform_version": "1.0.0",
    "additional_data": {}
  }
}
EOF
)

# Note: Using /payments for initiation as per new controller
PAYMENT_RES=$(curl -s -X POST "$API_URL/payments" \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $TOKEN" \
  -d "$PAYMENT_PAYLOAD")

# Extract Payment ID
PAYMENT_ID=$(echo "$PAYMENT_RES" | grep -o '"payment_id":"[^"]*' | cut -d'"' -f4)

if [ -n "$PAYMENT_ID" ]; then
    echo -e "${GREEN}OK${NC} (ID: $PAYMENT_ID)"
else
    echo -e "${RED}FAILED${NC}"
    echo "Response: $PAYMENT_RES"
    exit 1
fi

# 5. Process Payment
echo -n "Processing Payment... "
# Assuming mock processing
PROCESS_PAYLOAD=$(cat <<EOF
{
  "gateway_transaction_id": "mock_tx_${RANDOM_ID}",
  "gateway_status": "success"
}
EOF
)

PROCESS_RES=$(curl -s -X POST "$API_URL/payments/$PAYMENT_ID/process" \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $TOKEN" \
  -d "$PROCESS_PAYLOAD")

if echo "$PROCESS_RES" | grep -q "status"; then
     echo -e "${GREEN}OK${NC}"
else
    echo -e "${RED}FAILED${NC}"
    echo "Response: $PROCESS_RES"
fi

echo "----------------------------------------"
echo "Verification Complete."
