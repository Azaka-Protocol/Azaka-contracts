#!/bin/bash

set -e

echo "🚀 Deploying Azaka contracts to Stellar Testnet..."
echo ""

# Check if stellar CLI is installed
if ! command -v stellar &> /dev/null; then
    echo "❌ Stellar CLI not found. Please install it first:"
    echo "   cargo install --locked stellar-cli"
    exit 1
fi

# Check if deployer identity exists
if ! stellar keys show deployer &> /dev/null; then
    echo "📝 Creating deployer identity..."
    stellar keys generate deployer --network testnet
fi

DEPLOYER=$(stellar keys address deployer)
echo "Deployer address: $DEPLOYER"
echo ""

# Build contracts
echo "🔨 Building contracts..."
cargo build --target wasm32-unknown-unknown --release
echo "✓ Contracts built"
echo ""

# Deploy registry contract
echo "📦 Deploying registry contract..."
REGISTRY_ID=$(stellar contract deploy \
    --wasm target/wasm32-unknown-unknown/release/azaka_registry.wasm \
    --source deployer \
    --network testnet)
echo "✓ Registry deployed: $REGISTRY_ID"
echo ""

# Deploy document contract
echo "📦 Deploying document contract..."
DOCUMENT_ID=$(stellar contract deploy \
    --wasm target/wasm32-unknown-unknown/release/azaka_document.wasm \
    --source deployer \
    --network testnet)
echo "✓ Document deployed: $DOCUMENT_ID"
echo ""

# Deploy escrow contract
echo "📦 Deploying escrow contract..."
ESCROW_ID=$(stellar contract deploy \
    --wasm target/wasm32-unknown-unknown/release/azaka_escrow.wasm \
    --source deployer \
    --network testnet)
echo "✓ Escrow deployed: $ESCROW_ID"
echo ""

# Deploy trade contract
echo "📦 Deploying trade contract..."
TRADE_ID=$(stellar contract deploy \
    --wasm target/wasm32-unknown-unknown/release/azaka_trade.wasm \
    --source deployer \
    --network testnet)
echo "✓ Trade deployed: $TRADE_ID"
echo ""

# Initialize contracts
echo "⚙️  Initializing contracts..."

# Initialize registry
stellar contract invoke \
    --id $REGISTRY_ID \
    --source deployer \
    --network testnet \
    -- initialize \
    --admin $DEPLOYER
echo "✓ Registry initialized"

# Initialize document
stellar contract invoke \
    --id $DOCUMENT_ID \
    --source deployer \
    --network testnet \
    -- initialize \
    --registry_contract $REGISTRY_ID
echo "✓ Document initialized"

# For escrow and trade initialization, we need a token contract
# In production, use actual USDC contract address
echo "⚠️  Note: Escrow and Trade contracts need token contract address for initialization"
echo "   Run these commands manually with your token contract:"
echo ""
echo "   stellar contract invoke --id $ESCROW_ID --source deployer --network testnet \\"
echo "     -- initialize --trade_contract <TRADE_ID> --token_contract <TOKEN_ID>"
echo ""
echo "   stellar contract invoke --id $TRADE_ID --source deployer --network testnet \\"
echo "     -- initialize --escrow_contract $ESCROW_ID --document_contract $DOCUMENT_ID"
echo ""

# Save contract IDs to .env file
echo "💾 Saving contract IDs..."
cat > .env << EOF
TRADE_CONTRACT_ID=$TRADE_ID
ESCROW_CONTRACT_ID=$ESCROW_ID
DOCUMENT_CONTRACT_ID=$DOCUMENT_ID
REGISTRY_CONTRACT_ID=$REGISTRY_ID
DEPLOYER_ADDRESS=$DEPLOYER
NETWORK=testnet
EOF
echo "✓ Contract IDs saved to .env"
echo ""

echo "✅ Deployment complete!"
echo ""
echo "Contract Addresses:"
echo "  Trade:    $TRADE_ID"
echo "  Escrow:   $ESCROW_ID"
echo "  Document: $DOCUMENT_ID"
echo "  Registry: $REGISTRY_ID"
echo ""
echo "Next steps:"
echo "  1. Initialize escrow and trade contracts with token address"
echo "  2. Run seed script: cd sdk/typescript && npm run seed"
echo "  3. Update README.md with contract addresses"
