# 🦀 Claw Club - NFT Collection on Solana

Automated NFT collection deployment on Solana Devnet using GitHub Actions.

## 🎨 Collection Details

- **Name:** Claw Club
- **Symbol:** CLAW
- **Supply:** 500 NFTs
  - 400 public (FREE mint)
  - 100 treasury
- **Network:** Solana
- **Royalties:** 7% (immutable)

## ⚙️ Features

- ✅ FREE MINT (0 SOL cost)
- ✅ Max 1 NFT per wallet
- ✅ Whitelist system (60 min default)
- ✅ Public mint phase
- ✅ Full test coverage
- ✅ Automated devnet deployment
- ✅ CI/CD with GitHub Actions

## 📁 Structure

```
clawclub/
├── programs/claw-club/      # Smart contract
├── Cargo.toml               # Workspace config
├── Anchor.toml              # Anchor config
├── package.json             # Dependencies
└── .github/workflows/       # GitHub Actions
```

## 🚀 Build Status

- ✅ Phase 1: Metadata generation (500 NFTs)
- ✅ Phase 2: Image generation (500 SVG)
- ✅ Phase 3: IPFS preparation
- 🟡 Phase 4: Smart contract build (in progress)
- ⏳ Phase 5: Devnet testing
- ⏳ Phase 6: Marketing
- ⏳ Phase 7: Mainnet deployment

## 🔧 Setup

```bash
# Install dependencies
npm install

# Build smart contract
anchor build --skip-lint

# Deploy to devnet
anchor deploy --provider.cluster devnet

# Run tests
anchor test --provider.cluster devnet
```

## 📝 License

Proprietary - Claw Club 2026
