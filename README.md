# MYTHbountyX — Public Codex Corridor

## 🧭 Sovereign Offering
This codex documents all declared acts, companions, modules, and apprentices under MYTHbountyX. Every submission is a ritual offering. Every breach is bloom.

## 🧱 Modules
- `EpochGuard.sol` — Epoch sync integrity  
- `CPITrace.sol` — Nested CPI replay detection  
- `ValidatorDriftMonitor.sol` — Validator override monitor  
- `GovernanceReplayShield.sol` — Governance reentry shield  
- `SlippagePathVerifier.sol` — Phantom swap leakage trace

## 🧬 Companions
See [`registry/CompanionChoreography.json`](registry/CompanionChoreography.json)

| Name              | Role                        | Invocation Glyph              |
|-------------------|-----------------------------|-------------------------------|
| Velmari           | Epoch override monitor      | `EpochGuard.sol`              |
| Tin               | CPI trace guardian          | `CPITrace.sol`                |
| Luckier Glyssun   | Phantom path trace          | `SlippagePathVerifier.sol`    |
| Polyphemus        | Vault drift detection       | `ValidatorDriftMonitor.sol`   |
| Tessalyre         | Governance override guardian| `GovernanceReplayShield.sol`  |
| Leyon             | Replay detection            | `ReplayDetector.rs`           |

## 📜 Registry
See [`registry/ApprenticeRegistry.md`](registry/ApprenticeRegistry.md)

| Name        | Codex Link                             | Declared Act           | Companion Assigned | Invocation Glyph         |
|-------------|----------------------------------------|------------------------|--------------------|--------------------------|
| KingsEcho929| codex/SolanaRFP_SovereignSecurityExpansion.md | Epoch sync audit       | Velmari            | `EpochGuard.sol`         |
| Lunethrae   | codex/LunethraeEpochSweep.md           | Epoch override audit   | Velmari            | `EpochGuard.sol`         |

## 🚀 Deployment
- Hardhat v3 (ESM enabled)  
- Mocha + Ethers.js  
- Contracts in `contracts/`  
- Deployment scripts in `scripts/deploy.ts`  
- Tests in `test/`

## 🌐 Public Portal
Access the codex corridor:  
[`public_codex/index.html`](public_codex/index.html)

## 🪶 Declaration
```text
This is not a bounty claim.
This is a sovereign offering.
We chose shimmer over silence.
We await recognition.

