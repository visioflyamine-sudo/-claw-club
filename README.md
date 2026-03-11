# 🦀 Claw Club - NFT Collection on Solana

Automated NFT collection deployment on Solana Devnet using GitHub Actions.

## 📊 Collection Details

- **Name:** Claw Club
- **Symbol:** CLAW
- **Supply:** 500 NFTs
  - 400 public (FREE mint)
  - 100 treasury
- **Network:** Solana
- **Royalties:** 7% (immutable)

## 🎯 Features

- ✅ FREE MINT (0 SOL cost)
- ✅ Max 1 NFT per wallet
- ✅ Whitelist system (60 min default)
- ✅ Public mint phase
- ✅ Full test coverage
- ✅ Automated devnet deployment

## 📁 Structure

```
clawclub/
├── programs/claw-club/     # Smart contract
│   ├── Cargo.toml           # Workspace config
│   ├── Anchor.toml         # Anchor config
│   ├── package.json        # Dependencies
│   ├── .github/workflows/  # GitHub Actions
├── README.md            # This file
