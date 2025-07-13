#!/bin/bash

# VibeStream S3 & CloudFront Setup Script
# Automatically configures AWS infrastructure for audio storage and streaming

set -e

echo "🚀 VibeStream S3 & CloudFront Setup"
echo "===================================="

# Configuration
BUCKET_NAME="vibestream-audio-production"
REGION="us-east-1"
CDN_DOMAIN="cdn.vibestream.com"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Check if AWS CLI is installed
if ! command -v aws &> /dev/null; then
    echo -e "${RED}❌ AWS CLI not found. Please install it first.${NC}"
    echo "Install: https://aws.amazon.com/cli/"
    exit 1
fi

# Check AWS credentials
if ! aws sts get-caller-identity &> /dev/null; then
    echo -e "${RED}❌ AWS credentials not configured.${NC}"
    echo "Run: aws configure"
    exit 1
fi

echo -e "${GREEN}✅ AWS CLI configured${NC}"

# Step 1: Create S3 Bucket
echo -e "${YELLOW}📦 Creating S3 bucket: $BUCKET_NAME${NC}"
if aws s3api head-bucket --bucket "$BUCKET_NAME" 2>/dev/null; then
    echo -e "${GREEN}✅ Bucket already exists${NC}"
else
    aws s3api create-bucket \
        --bucket "$BUCKET_NAME" \
        --region "$REGION" \
        --create-bucket-configuration LocationConstraint="$REGION"
    echo -e "${GREEN}✅ S3 bucket created${NC}"
fi

# Step 2: Configure bucket for audio files
echo -e "${YELLOW}🔧 Configuring S3 bucket settings${NC}"

# Enable versioning
aws s3api put-bucket-versioning \
    --bucket "$BUCKET_NAME" \
    --versioning-configuration Status=Enabled

# Configure CORS for web uploads
aws s3api put-bucket-cors \
    --bucket "$BUCKET_NAME" \
    --cors-configuration '{
        "CORSRules": [
            {
                "AllowedOrigins": ["*"],
                "AllowedMethods": ["GET", "POST", "PUT", "DELETE", "HEAD"],
                "AllowedHeaders": ["*"],
                "ExposeHeaders": ["ETag"],
                "MaxAgeSeconds": 3000
            }
        ]
    }'

# Configure lifecycle policy for cost optimization
aws s3api put-bucket-lifecycle-configuration \
    --bucket "$BUCKET_NAME" \
    --lifecycle-configuration '{
        "Rules": [
            {
                "ID": "AudioFileLifecycle",
                "Status": "Enabled",
                "Transitions": [
                    {
                        "Days": 30,
                        "StorageClass": "STANDARD_IA"
                    },
                    {
                        "Days": 90,
                        "StorageClass": "GLACIER"
                    }
                ]
            }
        ]
    }'

echo -e "${GREEN}✅ S3 bucket configured${NC}"

# Step 3: Create CloudFront Distribution
echo -e "${YELLOW}🌐 Creating CloudFront distribution${NC}"

# Create distribution configuration
DISTRIBUTION_CONFIG='{
    "CallerReference": "vibestream-'$(date +%s)'",
    "Comment": "VibeStream Audio CDN",
    "DefaultCacheBehavior": {
        "TargetOriginId": "S3-'$BUCKET_NAME'",
        "ViewerProtocolPolicy": "redirect-to-https",
        "TrustedSigners": {
            "Enabled": false,
            "Quantity": 0
        },
        "ForwardedValues": {
            "QueryString": false,
            "Cookies": {"Forward": "none"},
            "Headers": {
                "Quantity": 1,
                "Items": ["Origin"]
            }
        },
        "MinTTL": 0,
        "DefaultTTL": 86400,
        "MaxTTL": 31536000,
        "Compress": true
    },
    "Origins": {
        "Quantity": 1,
        "Items": [
            {
                "Id": "S3-'$BUCKET_NAME'",
                "DomainName": "'$BUCKET_NAME'.s3.'$REGION'.amazonaws.com",
                "S3OriginConfig": {
                    "OriginAccessIdentity": ""
                }
            }
        ]
    },
    "Enabled": true,
    "PriceClass": "PriceClass_All"
}'

# Create the distribution
DISTRIBUTION_ID=$(aws cloudfront create-distribution \
    --distribution-config "$DISTRIBUTION_CONFIG" \
    --query 'Distribution.Id' \
    --output text)

echo -e "${GREEN}✅ CloudFront distribution created: $DISTRIBUTION_ID${NC}"

# Get the CloudFront domain name
CLOUDFRONT_DOMAIN=$(aws cloudfront get-distribution \
    --id "$DISTRIBUTION_ID" \
    --query 'Distribution.DomainName' \
    --output text)

echo -e "${GREEN}✅ CloudFront domain: $CLOUDFRONT_DOMAIN${NC}"

# Step 4: Create environment configuration
echo -e "${YELLOW}📝 Creating environment configuration${NC}"

cat > .env.production << EOF
# VibeStream Production Configuration - Generated $(date)
AWS_S3_BUCKET=$BUCKET_NAME
AWS_REGION=$REGION
AWS_ACCESS_KEY_ID=\${AWS_ACCESS_KEY_ID}
AWS_SECRET_ACCESS_KEY=\${AWS_SECRET_ACCESS_KEY}

# CloudFront CDN
VIBESTREAM_CDN_DOMAIN=$CLOUDFRONT_DOMAIN

# Storage Settings
MAX_AUDIO_FILE_SIZE=200000000
UPLOAD_TIMEOUT_SECONDS=300

# Performance
ENABLE_AUDIO_COMPRESSION=true
ENABLE_METADATA_EXTRACTION=true
ENABLE_AUDIO_PREVIEW_GENERATION=true

# Security
ENABLE_VIRUS_SCANNING=true
ALLOWED_AUDIO_FORMATS=mp3,flac,wav,aac,ogg,m4a

# Monitoring
ENABLE_UPLOAD_ANALYTICS=true
ENABLE_STREAMING_ANALYTICS=true
LOG_LEVEL=info
EOF

echo -e "${GREEN}✅ Environment configuration created: .env.production${NC}"

# Step 5: Test the setup
echo -e "${YELLOW}🧪 Testing S3 setup${NC}"

# Create a test file
echo "VibeStream S3 Test - $(date)" > test-file.txt

# Upload test file
aws s3 cp test-file.txt s3://"$BUCKET_NAME"/test/
echo -e "${GREEN}✅ Test upload successful${NC}"

# Download test file
aws s3 cp s3://"$BUCKET_NAME"/test/test-file.txt downloaded-test.txt
echo -e "${GREEN}✅ Test download successful${NC}"

# Clean up test files
rm test-file.txt downloaded-test.txt
aws s3 rm s3://"$BUCKET_NAME"/test/test-file.txt

echo ""
echo -e "${GREEN}🎉 Setup Complete!${NC}"
echo "=================================="
echo "S3 Bucket: $BUCKET_NAME"
echo "CloudFront Domain: $CLOUDFRONT_DOMAIN"
echo "Distribution ID: $DISTRIBUTION_ID"
echo ""
echo "Next steps:"
echo "1. Set your AWS credentials in the environment"
echo "2. Source the production environment: source .env.production"
echo "3. Deploy your application with S3 storage enabled"
echo ""
echo -e "${YELLOW}⚠️  Note: CloudFront deployment takes 15-20 minutes to complete${NC}" 