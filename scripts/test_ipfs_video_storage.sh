#!/bin/bash

# Test script for IPFS Video Storage P2P System
# This script demonstrates the complete video upload and streaming flow

set -e

echo "🎬 Testing IPFS Video Storage P2P System"
echo "========================================"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
API_BASE_URL="http://localhost:3000"
IPFS_NODE_URL="http://localhost:5001"

# Test data
TEST_VIDEO_FILE="test_video.mp4"
TEST_VIDEO_TITLE="Test Video Stream"
TEST_USER_ID="550e8400-e29b-41d4-a716-446655440000"
TEST_PEER_ID="peer_test_123"

echo -e "${BLUE}📋 Test Configuration:${NC}"
echo "  API Base URL: $API_BASE_URL"
echo "  IPFS Node URL: $IPFS_NODE_URL"
echo "  Test Video: $TEST_VIDEO_FILE"
echo "  Test Title: $TEST_VIDEO_TITLE"
echo ""

# Function to check if service is running
check_service() {
    local service_name=$1
    local url=$2
    
    echo -e "${YELLOW}🔍 Checking $service_name...${NC}"
    
    if curl -s --max-time 5 "$url" > /dev/null; then
        echo -e "${GREEN}✅ $service_name is running${NC}"
        return 0
    else
        echo -e "${RED}❌ $service_name is not running${NC}"
        return 1
    fi
}

# Function to create test video file
create_test_video() {
    echo -e "${YELLOW}🎬 Creating test video file...${NC}"
    
    # Create a simple test video using ffmpeg (if available)
    if command -v ffmpeg &> /dev/null; then
        ffmpeg -f lavfi -i testsrc=duration=10:size=320x240:rate=1 \
               -f lavfi -i sine=frequency=1000:duration=10 \
               -c:v libx264 -c:a aac \
               -shortest "$TEST_VIDEO_FILE" -y > /dev/null 2>&1
        
        if [ -f "$TEST_VIDEO_FILE" ]; then
            echo -e "${GREEN}✅ Test video created: $TEST_VIDEO_FILE${NC}"
            return 0
        else
            echo -e "${RED}❌ Failed to create test video${NC}"
            return 1
        fi
    else
        echo -e "${YELLOW}⚠️  ffmpeg not available, creating dummy video file${NC}"
        # Create a dummy video file
        dd if=/dev/zero of="$TEST_VIDEO_FILE" bs=1M count=10 2>/dev/null
        echo -e "${GREEN}✅ Dummy test video created: $TEST_VIDEO_FILE${NC}"
        return 0
    fi
}

# Function to test video upload
test_video_upload() {
    echo -e "${YELLOW}📤 Testing video upload to IPFS...${NC}"
    
    local upload_response=$(curl -s -X POST \
        -F "file=@$TEST_VIDEO_FILE" \
        -F "filename=$TEST_VIDEO_FILE" \
        -F "content_type=video/mp4" \
        -F "title=$TEST_VIDEO_TITLE" \
        -F "description=Test video for IPFS storage" \
        -F "stream_type=video" \
        -F "quality=High" \
        -F "format=mp4" \
        -F "bitrate=5000" \
        -F "resolution=1920x1080" \
        -F "fps=30" \
        "$API_BASE_URL/api/v1/p2p/video/upload")
    
    if echo "$upload_response" | jq -e '.status' > /dev/null 2>&1; then
        local status=$(echo "$upload_response" | jq -r '.status')
        if [ "$status" = "success" ]; then
            echo -e "${GREEN}✅ Video upload successful${NC}"
            echo "  Stream ID: $(echo "$upload_response" | jq -r '.stream_id')"
            echo "  IPFS URL: $(echo "$upload_response" | jq -r '.ipfs_url')"
            echo "  Message: $(echo "$upload_response" | jq -r '.message')"
            return 0
        else
            echo -e "${RED}❌ Video upload failed: $status${NC}"
            return 1
        fi
    else
        echo -e "${RED}❌ Invalid upload response${NC}"
        echo "Response: $upload_response"
        return 1
    fi
}

# Function to test video streaming
test_video_streaming() {
    echo -e "${YELLOW}🎥 Testing video streaming...${NC}"
    
    # Get available streams
    local streams_response=$(curl -s "$API_BASE_URL/api/v1/p2p/streaming/streams")
    
    if echo "$streams_response" | jq -e '.streams' > /dev/null 2>&1; then
        local stream_count=$(echo "$streams_response" | jq '.streams | length')
        echo -e "${GREEN}✅ Found $stream_count available streams${NC}"
        
        if [ "$stream_count" -gt 0 ]; then
            local first_stream_id=$(echo "$streams_response" | jq -r '.streams[0].id')
            echo "  First Stream ID: $first_stream_id"
            
            # Test joining stream
            local join_response=$(curl -s -X POST \
                -H "Content-Type: application/json" \
                -d "{
                    \"user_id\": \"$TEST_USER_ID\",
                    \"peer_id\": \"$TEST_PEER_ID\",
                    \"connection_quality\": {
                        \"bandwidth_mbps\": 100.0,
                        \"latency_ms\": 50
                    }
                }" \
                "$API_BASE_URL/api/v1/p2p/streaming/streams/$first_stream_id/join")
            
            if echo "$join_response" | jq -e '.status' > /dev/null 2>&1; then
                echo -e "${GREEN}✅ Successfully joined stream${NC}"
                echo "  Viewer ID: $(echo "$join_response" | jq -r '.viewer_id')"
                echo "  Quality: $(echo "$join_response" | jq -r '.quality')"
                return 0
            else
                echo -e "${RED}❌ Failed to join stream${NC}"
                return 1
            fi
        else
            echo -e "${YELLOW}⚠️  No streams available for testing${NC}"
            return 0
        fi
    else
        echo -e "${RED}❌ Failed to get streams${NC}"
        return 1
    fi
}

# Function to test P2P analytics
test_p2p_analytics() {
    echo -e "${YELLOW}📊 Testing P2P analytics...${NC}"
    
    # Test streaming stats
    local streaming_stats=$(curl -s "$API_BASE_URL/api/v1/p2p/analytics/stats")
    if echo "$streaming_stats" | jq -e '.total_streams' > /dev/null 2>&1; then
        echo -e "${GREEN}✅ Streaming stats retrieved${NC}"
        echo "  Total Streams: $(echo "$streaming_stats" | jq -r '.total_streams')"
        echo "  Active Streams: $(echo "$streaming_stats" | jq -r '.active_streams')"
        echo "  Total Viewers: $(echo "$streaming_stats" | jq -r '.total_viewers')"
    else
        echo -e "${RED}❌ Failed to get streaming stats${NC}"
    fi
    
    # Test peer stats
    local peer_stats=$(curl -s "$API_BASE_URL/api/v1/p2p/analytics/peers")
    if echo "$peer_stats" | jq -e '.active_peers' > /dev/null 2>&1; then
        echo -e "${GREEN}✅ Peer stats retrieved${NC}"
        echo "  Active Peers: $(echo "$peer_stats" | jq -r '.active_peers')"
        echo "  Total Connections: $(echo "$peer_stats" | jq -r '.total_connections')"
        echo "  Avg Latency: $(echo "$peer_stats" | jq -r '.average_latency_ms')ms"
    else
        echo -e "${RED}❌ Failed to get peer stats${NC}"
    fi
    
    # Test network stats
    local network_stats=$(curl -s "$API_BASE_URL/api/v1/p2p/analytics/network")
    if echo "$network_stats" | jq -e '.network_health_score' > /dev/null 2>&1; then
        echo -e "${GREEN}✅ Network stats retrieved${NC}"
        echo "  Network Health: $(echo "$network_stats" | jq -r '.network_health_score')"
        echo "  Content Availability: $(echo "$network_stats" | jq -r '.content_availability_score')"
    else
        echo -e "${RED}❌ Failed to get network stats${NC}"
    fi
}

# Function to test WebRTC functionality
test_webrtc() {
    echo -e "${YELLOW}🌐 Testing WebRTC functionality...${NC}"
    
    # Test peer connection
    local connect_response=$(curl -s -X POST \
        -H "Content-Type: application/json" \
        -d "{}" \
        "$API_BASE_URL/api/v1/p2p/webrtc/peer/$TEST_PEER_ID/connect")
    
    if echo "$connect_response" | jq -e '.status' > /dev/null 2>&1; then
        local status=$(echo "$connect_response" | jq -r '.status')
        if [ "$status" = "connected" ]; then
            echo -e "${GREEN}✅ WebRTC peer connection successful${NC}"
            echo "  Connection ID: $(echo "$connect_response" | jq -r '.connection_id')"
            return 0
        else
            echo -e "${RED}❌ WebRTC peer connection failed: $status${NC}"
            return 1
        fi
    else
        echo -e "${RED}❌ Invalid WebRTC response${NC}"
        return 1
    fi
}

# Function to cleanup test files
cleanup() {
    echo -e "${YELLOW}🧹 Cleaning up test files...${NC}"
    
    if [ -f "$TEST_VIDEO_FILE" ]; then
        rm "$TEST_VIDEO_FILE"
        echo -e "${GREEN}✅ Removed test video file${NC}"
    fi
}

# Main test execution
main() {
    echo -e "${BLUE}🚀 Starting IPFS Video Storage P2P System Tests${NC}"
    echo ""
    
    # Check if required services are running
    if ! check_service "API Gateway" "$API_BASE_URL/health"; then
        echo -e "${RED}❌ Please start the API Gateway service first${NC}"
        exit 1
    fi
    
    if ! check_service "IPFS Node" "$IPFS_NODE_URL/api/v0/version"; then
        echo -e "${YELLOW}⚠️  IPFS node not running, some features may not work${NC}"
    fi
    
    echo ""
    
    # Create test video file
    if ! create_test_video; then
        echo -e "${RED}❌ Failed to create test video${NC}"
        exit 1
    fi
    
    echo ""
    
    # Run tests
    local tests_passed=0
    local tests_total=0
    
    # Test 1: Video Upload
    tests_total=$((tests_total + 1))
    if test_video_upload; then
        tests_passed=$((tests_passed + 1))
    fi
    
    echo ""
    
    # Test 2: Video Streaming
    tests_total=$((tests_total + 1))
    if test_video_streaming; then
        tests_passed=$((tests_passed + 1))
    fi
    
    echo ""
    
    # Test 3: WebRTC
    tests_total=$((tests_total + 1))
    if test_webrtc; then
        tests_passed=$((tests_passed + 1))
    fi
    
    echo ""
    
    # Test 4: P2P Analytics
    tests_total=$((tests_total + 1))
    if test_p2p_analytics; then
        tests_passed=$((tests_passed + 1))
    fi
    
    echo ""
    
    # Test results summary
    echo -e "${BLUE}📊 Test Results Summary${NC}"
    echo "=========================="
    echo -e "Tests Passed: ${GREEN}$tests_passed${NC}/$tests_total"
    echo -e "Success Rate: ${GREEN}$((tests_passed * 100 / tests_total))%${NC}"
    
    if [ $tests_passed -eq $tests_total ]; then
        echo -e "${GREEN}🎉 All tests passed! IPFS Video Storage P2P System is working correctly.${NC}"
    else
        echo -e "${YELLOW}⚠️  Some tests failed. Please check the logs above for details.${NC}"
    fi
    
    echo ""
    
    # Cleanup
    cleanup
    
    echo -e "${BLUE}✅ Test execution completed${NC}"
}

# Run main function
main "$@" 