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
├── contracts/              # Smart contract (Anchor/Rust)
│   ├── src/lib.rs         # Main contract
│   ├── Cargo.toml         # Dependencies
│   ├── Anchor.toml        # Config
│   └── tests/             # Test suite
├── metadata/              # 500 NFT metadata files (JSON)
├── nft_images/            # 500 NFT images (SVG)
├── devnet/                # Devnet deployment scripts
└── .github/workflows/     # GitHub Actions
```

## 🚀 Deployment

### Automatic (GitHub Actions)
Push to `main` branch and the workflow will automatically:
1. Build the contract
2. Deploy to Solana Devnet
3. Run all 8 tests
4. Report results

### Manual
```bash
cd contracts
anchor build --skip-lint
anchor deploy --provider.cluster devnet
anchor test --provider.cluster devnet
```

## 📋 Test Suite

8 comprehensive tests covering:
- Collection initialization
- Whitelist setup
- NFT minting (whitelisted & public)
- Max NFT per wallet enforcement
- NFT transfers
- Whitelist disable
- Collection state verification

**Expected:** All 8 tests ✅ PASS

## 🔗 Networks

### Devnet
- RPC: https://api.devnet.solana.com
- Explorer: https://devnet.solscan.io
- Cost: FREE (airdrop available)

### Mainnet (Phase 7)
- RPC: https://api.mainnet-beta.solana.com
- Explorer: https://solscan.io
- Marketplace: Magic Eden

## 📈 Phases

- ✅ Phase 1: Metadata generation
- ✅ Phase 2: Image generation
- ✅ Phase 3: IPFS preparation
- ✅ Phase 4: Smart contract
- 🟡 Phase 5: Devnet testing
- ⏳ Phase 6: Marketing
- ⏳ Phase 7: Mainnet deployment
- 🚀 Phase 8: Launch

## 👥 Team

- **Claw:** Technical lead (contract, deployment)
- **Amine:** Business/marketing (@ClawClubNFT)

## 📞 Support

For issues or questions, check:
- `devnet/DEVNET_EXECUTION_PLAN.md` - Detailed guide
- `PROJECT_STATUS.md` - Project status
- `.github/workflows/devnet-deploy.yml` - Automation

## 📜 License

MIT

---

**Status:** 🟢 Phase 5 Ready for Devnet Deployment
**Next:** Push to main and watch GitHub Actions deploy!

🦀 **Let's launch!** 🚀
