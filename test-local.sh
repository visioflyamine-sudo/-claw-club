#!/bin/bash

# Claw Club Local Test Script
echo "=== CLAW CLUB NFT - LOCAL TEST SCRIPT ==="
echo "Started: $(date)"
echo ""

# 1. Check Rust
echo "Step 1: Checking Rust..."
if command -v rustc &> /dev/null; then
    echo "OK: Rust installed: $(rustc --version)"
else
    echo "INSTALL: Rust not found"
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source $HOME/.cargo/env
fi

# 2. Check Solana
echo ""
echo "Step 2: Checking Solana..."
if command -v solana &> /dev/null; then
    echo "OK: Solana installed: $(solana --version)"
else
    echo "INSTALL: Solana not found"
    sh -c "$(curl -sSfL https://release.solana.com/v1.18.0/install)"
fi

# 3. Check Anchor
echo ""
echo "Step 3: Checking Anchor..."
if command -v anchor &> /dev/null; then
    echo "OK: Anchor installed"
else
    echo "INSTALL: Anchor not found"
    npm install -g @coral-xyz/anchor-cli
fi

# 4. Build
echo ""
echo "Step 4: Building smart contract..."
cd . || exit 1
anchor build --skip-lint

if [ $? -eq 0 ]; then
    echo "SUCCESS: Build completed"
else
    echo "ERROR: Build failed"
    exit 1
fi

# 5. Check artifacts
echo ""
echo "Step 5: Checking artifacts..."
if [ -d "target/deploy" ]; then
    ls -lah target/deploy/
    echo "OK: Artifacts found"
else
    echo "ERROR: No artifacts"
fi

echo ""
echo "=== COMPLETE ==="
echo "Finished: $(date)"
