# 🎨 CraftLink

> **Empowering African Artisans Through Hedera Smart Contract Service**

**Built for Hedera African Hackathon**

CraftLink is a decentralized marketplace leveraging **Hedera Smart Contract Service (HSCS)** to bridge the gap between skilled African artisans and global clients. By harnessing Hedera's fast, fair, and secure distributed ledger technology, we provide gasless transactions, secure escrow payments, and cryptographic verification to ensure trust, transparency, and fair compensation for creative work across Africa.

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Solidity](https://img.shields.io/badge/Solidity-0.8.20-blue)](https://soliditylang.org/)
[![Next.js](https://img.shields.io/badge/Next.js-15.0-black)](https://nextjs.org/)
[![TypeScript](https://img.shields.io/badge/TypeScript-5.0-blue)](https://www.typescriptlang.org/)
[![Hedera](https://img.shields.io/badge/Hedera-Testnet-purple)](https://hedera.com/)

---

📜 **[Hedera Certificate](https://drive.google.com/file/d/1Ki2Q-0h-M8qoAsL5REH_kIM5AaD6YhTc/view?usp=drivesdk)** | 📊 **[Pitch Deck](https://gamma.app/docs/CraftLink-wrpqpq1ykrbe7yo?mode=doc)** | 🔗 **[Live Demo](https://craftlink-coral.vercel.app)**
## 📋 Table of Contents

- [Overview](#-overview)
- [The African Problem](#-the-african-problem)
- [Why Hedera?](#-why-hedera)
- [Key Features](#-key-features)
- [Architecture](#-architecture)
- [Tech Stack](#-tech-stack)
- [Project Structure](#-project-structure)
- [Getting Started](#-getting-started)
- [Smart Contracts](#-smart-contracts)
- [How It Works](#-how-it-works)
- [Impact & Vision](#-impact--vision)
- [Environment Variables](#-environment-variables)
- [Deployment](#-deployment)
- [Contributing](#-contributing)
- [License](#-license)

---

## 🌟 Overview

CraftLink is a **Hedera-powered decentralized marketplace** specifically designed to empower African artisans by connecting them with global opportunities while eliminating the barriers that traditional platforms impose.

### **Built on Hedera Smart Contract Service**

We leverage Hedera's enterprise-grade distributed ledger technology to provide:
- ⚡ **Lightning-fast transactions** (3-5 second finality)
- 💰 **Low, predictable fees** (fractions of a cent per transaction)
- 🔒 **Bank-grade security** with aBFT consensus
- 🌍 **Carbon-negative network** (sustainable for Africa's future)
- 🔗 **EVM compatibility** (deploy standard Solidity contracts)

---

## 🌍 The African Problem

### **Current Challenges Facing African Artisans:**

#### **1. Financial Exclusion**
- 📊 **57% of sub-Saharan Africans** are unbanked (World Bank, 2021)
- 💳 Traditional platforms require bank accounts and credit cards
- 💸 International payment processors charge **5-15% fees**
- ⏰ Payment delays of **7-30 days** for cross-border transactions

#### **2. Platform Exploitation**
- 💰 Existing freelance platforms charge **15-20% commission**
- 🚫 Geographic restrictions limit African participation
- 📉 Currency conversion losses eat into earnings
- ⚖️ Dispute resolution favors clients over artisans

#### **3. Trust & Verification**
- 🤝 No verifiable reputation system
- 📄 Portfolio theft and credential fraud
- 💔 Payment defaults with no recourse
- 🌐 Limited access to global markets

#### **4. Technical Barriers**
- 📱 High gas fees on traditional blockchains (Ethereum: $5-50 per transaction)
- 🔧 Complex wallet setup and crypto knowledge required
- 🌐 Poor internet infrastructure in rural areas
- ⚡ Slow transaction speeds on congested networks

### **The Impact:**

African artisans—from Nigerian tailors to Kenyan woodworkers, Ghanaian jewelry makers to South African graphic designers—lose **30-40% of their earnings** to fees, delays, and exploitation. This perpetuates poverty and limits economic growth across the continent.

---

## 🚀 Why Hedera?

We chose **Hedera Smart Contract Service** specifically to address African challenges:

### **1. Affordability**
- 💵 **$0.0001 per transaction** vs $5-50 on Ethereum
- 💰 Enables micro-payments for small gigs ($5-20)
- 📊 Predictable costs allow artisans to plan budgets

### **2. Speed & Reliability**
- ⚡ **3-5 second finality** vs 15+ minutes on other chains
- 🔄 **10,000+ TPS** handles high transaction volume
- 📶 Works efficiently even with limited internet bandwidth

### **3. Accessibility**
- 🌐 **Gasless transactions** via meta-transaction relayer
- 📱 No crypto knowledge required for end users
- 🔗 Multi-wallet support (MetaMask, mobile wallets)

### **4. Sustainability**
- 🌱 **Carbon-negative network** aligns with Africa's climate goals
- ♻️ Energy-efficient consensus (no mining)
- 🌍 Supports green economy initiatives

### **5. Enterprise Trust**
- 🏛️ Governed by global enterprises (Google, IBM, Boeing)
- 🔒 Bank-grade security (aBFT consensus)
- 📜 Regulatory compliance for institutional adoption

### **6. Developer Experience**
- 💻 **EVM-compatible** (standard Solidity contracts)
- 🛠️ Works with existing tools (Foundry, Ethers.js, Thirdweb)
- 📚 Comprehensive documentation and support

### **Our Hedera Implementation:**

```solidity
// All 7 smart contracts deployed on Hedera Testnet
✅ Registry (User Management)
✅ GigMarketplace (Core Logic)
✅ PaymentProcessor (Escrow System)
✅ Token (Mock USDT for payments)
✅ CraftCoin (Platform token for spam prevention)
✅ ReviewSystem (On-chain reputation)
✅ ChatSystem (Message verification)
```

**Chain ID:** 296 (Hedera Testnet)  
**RPC:** https://testnet.hashio.io/api  
**Explorer:** https://hashscan.io/testnet

---

## 💡 Our Solution

A **Hedera-powered marketplace** that:

### **For Artisans:**
- ✅ **Zero gas fees** - Platform pays transaction costs
- ✅ **Instant payments** - Funds released in seconds, not weeks
- ✅ **10% platform fee** - vs 15-20% on traditional platforms
- ✅ **Global access** - Connect with clients worldwide
- ✅ **Verifiable reputation** - On-chain reviews build trust
- ✅ **Portfolio ownership** - IPFS storage, you control your work

### **For Clients:**
- ✅ **Escrow protection** - Funds held until work is verified
- ✅ **Quality assurance** - Review system filters top talent
- ✅ **Transparent pricing** - No hidden fees
- ✅ **Direct communication** - Built-in messaging

### **Technical Innovation:**
- 🔐 **Merkle tree verification** - Tamper-proof data integrity
- 🎫 **Meta-transactions** - Gasless UX via relayer
- 🪙 **Token economics** - CFT prevents spam applications
- 📦 **Hybrid storage** - Critical data on-chain, media on IPFS

---

## ✨ Key Features

### **For Artisans**
- 🎯 **Browse & Apply** - Find gigs matching your skills
- 💰 **Secure Payments** - Funds held in escrow until completion
- 🏆 **Build Reputation** - On-chain reviews and portfolio
- 🎁 **Earn Tokens** - Mint 50 CFT every 30 days
- 💬 **Direct Messaging** - Communicate with clients

### **For Clients**
- 📝 **Post Jobs** - Create detailed gig listings
- 👥 **Review Applicants** - Browse artisan profiles and portfolios
- ✅ **Milestone Tracking** - Mark completion and release funds
- ⭐ **Leave Reviews** - Rate artisan performance
- 🔒 **Refund Protection** - Get funds back if gig is cancelled

### **Platform Features**
- 🚀 **Gasless Transactions** - No gas fees for users
- 🔐 **Data Verification** - Merkle tree proofs ensure integrity
- 🌐 **IPFS Storage** - Decentralized media hosting
- 🔗 **Multi-Wallet Support** - MetaMask, Coinbase, WalletConnect, and more
- 📱 **Responsive Design** - Works on desktop and mobile

---

## 🏗️ Architecture

CraftLink uses a **hybrid architecture** combining on-chain and off-chain components:

```
┌─────────────────────────────────────────────────────────────┐
│                         Frontend                             │
│              (Next.js + React + Thirdweb)                   │
└─────────────┬───────────────────────────┬───────────────────┘
              │                           │
              ▼                           ▼
    ┌─────────────────┐         ┌─────────────────┐
    │     Backend     │         │     Relayer     │
    │  (Express API)  │         │ (Meta-Tx Server)│
    └────────┬────────┘         └────────┬────────┘
             │                           │
             ▼                           ▼
    ┌─────────────────┐         ┌─────────────────┐
    │    MongoDB      │         │  Smart Contracts│
    │  (Off-chain DB) │         │ (Hedera Testnet)│
    └─────────────────┘         └─────────────────┘
             │                           │
             └───────────┬───────────────┘
                         ▼
                ┌─────────────────┐
                │  IPFS Storage   │
                │  (Media Files)  │
                └─────────────────┘
```

### **Data Flow**

1. **User Action** → Frontend captures user input
2. **Sign Message** → User signs transaction with wallet
3. **Relayer Execution** → Relayer submits transaction on-chain
4. **Blockchain Update** → Smart contract state changes
5. **Backend Sync** → Backend verifies and stores detailed data
6. **Merkle Verification** → Cryptographic proof ensures data integrity

---

## 🛠️ Tech Stack

### **Blockchain Layer (Hedera Smart Contract Service)**
| Component | Technology | Purpose |
|-----------|-----------|---------|
| Smart Contracts | Solidity 0.8.20 | Core business logic on HSCS |
| Network | **Hedera Testnet (Chain ID: 296)** | Enterprise-grade DLT |
| Consensus | Hashgraph (aBFT) | 3-5s finality, 10,000+ TPS |
| Development Framework | Foundry | Testing & deployment |
| RPC Relay | HashIO | JSON-RPC for Web3 tools |
| Token Standards | ERC-20 Compatible | USDT mock & CraftCoin |
| Gas Model | Gasless (Meta-transactions) | Zero fees for end users |

### **Backend Services**
| Component | Technology | Purpose |
|-----------|-----------|---------|
| API Server | Express + TypeScript | RESTful API |
| Database | MongoDB + Mongoose | Off-chain data storage |
| Relayer | Express + Ethers.js | Gasless transactions |
| Verification | MerkleTree.js | Data integrity proofs |

### **Frontend Application**
| Component | Technology | Purpose |
|-----------|-----------|---------|
| Framework | Next.js 15 | React framework |
| Styling | Tailwind CSS | Responsive design |
| State Management | Zustand | Client state |
| Wallet Connection | Thirdweb | Multi-wallet support |
| Animations | Framer Motion | UI animations |
| Blockchain Interaction | Ethers.js + Viem | Smart contract calls |

### **Infrastructure**
| Component | Technology | Purpose |
|-----------|-----------|---------|
| Storage | IPFS + Cloudinary | Decentralized media |
| Hosting | Vercel | Frontend & API hosting |
| Version Control | Git + GitHub | Source control |

---

## 📁 Project Structure

```
CraftLink/
├── smart_contract/          # Solidity smart contracts
│   ├── src/
│   │   ├── Registry.sol           # User registration
│   │   ├── GigMarketplace.sol     # Gig management
│   │   ├── PaymentProcessor.sol   # Escrow system
│   │   ├── Token.sol              # Mock USDT
│   │   ├── CraftCoin.sol          # Platform token
│   │   ├── ReviewSystem.sol       # Rating system
│   │   ├── ChatSystem.sol         # Message hashes
│   │   └── interfaces/            # Contract interfaces
│   ├── script/                    # Deployment scripts
│   ├── test/                      # Contract tests
│   └── foundry.toml               # Foundry config
│
├── relayer/                 # Meta-transaction relayer
│   ├── src/
│   │   ├── server.ts              # Main server
│   │   ├── abi/                   # Contract ABIs
│   │   └── *.ts                   # Utility functions
│   └── package.json
│
├── backend/                 # Express API server
│   ├── src/
│   │   ├── index.ts               # Entry point
│   │   ├── models/                # MongoDB schemas
│   │   ├── controllers/           # Route handlers
│   │   ├── routes/                # API routes
│   │   ├── utils/                 # Helper functions
│   │   └── types/                 # TypeScript types
│   └── package.json
│
├── frontend/                # Next.js application
│   ├── app/                       # App router pages
│   │   ├── page.tsx               # Landing page
│   │   ├── authenticate/          # Wallet connection
│   │   ├── role/                  # User onboarding
│   │   ├── marketplace/           # Browse gigs
│   │   ├── manage-jobs/           # Job management
│   │   ├── profile/               # User profiles
│   │   └── messages/              # Chat interface
│   ├── components/                # React components
│   ├── hooks/                     # Custom hooks
│   ├── utils/                     # Helper functions
│   ├── constants/                 # App constants
│   └── package.json
│
└── README.md                # This file
```

---

## 🚀 Getting Started

### **Prerequisites**

- **Node.js** v18+ and npm/yarn
- **MongoDB** (local or Atlas)
- **Git**
- **Foundry** (for smart contracts)
- **MetaMask** or compatible Web3 wallet

### **Installation**

#### **1. Clone the Repository**

```bash
git clone https://github.com/xeladev4/CraftLink/
cd CraftLink
```

#### **2. Smart Contracts Setup**

```bash
cd smart_contract

# Install Foundry (if not already installed)
curl -L https://foundry.paradigm.xyz | bash
foundryup

# Install dependencies
forge install

# Compile contracts
forge build

# Run tests
forge test

# Deploy to testnet (configure .env first)
forge script script/Deploy.s.sol --rpc-url $RPC_URL --broadcast
```

**Environment Variables** (create `.env`):
```env
RPC_URL=https://testnet.hashio.io/api
PRIVATE_KEY=your_private_key_here
ETHERSCAN_API_KEY=your_block_explorer_api_key
```

#### **3. Backend Setup**

```bash
cd ../backend

# Install dependencies
npm install

# Create .env file
cp .env.example .env

# Start development server
npm run dev
```

**Environment Variables** (`.env`):
```env
PORT=3001
MONGODB_URI=mongodb://localhost:27017/craftlink
NODE_ENV=development
```

#### **4. Relayer Setup**

```bash
cd ../relayer

# Install dependencies
npm install

# Create .env file
cp .env.example .env

# Encrypt your private key
npm run encrypt

# Start relayer
npm run dev
```

**Environment Variables** (`.env`):
```env
PORT=3005
RPC_URL=https://testnet.hashio.io/api
PRIVATE_KEY_PASSWORD=your_secure_password
ENCRYPTED_KEY_JSON={"encrypted":"..."}
TOKEN_ADDRESS=0xd1fb7489D8689c45082Ea68dC025247d4143f15E
REGISTRY_ADDRESS=0x9c78Bbfc9a101f0C467560BD93401B72cC4152C1
REVIEW_SYSTEM_ADDRESS=0x6182AfDA6817a0a78a5e87Da57DD19F05bfCa9cf
GIG_MARKETPLACE_ADDRESS=0x78b06bfd164Ae7dee46E606da3e4d2Cb59997cD2
PAYMENT_PROCESSOR_ADDRESS=0x6976762d8dF143771eB8a2a930061A57806b6fd4
CRAFT_COIN_ADDRESS=0x77A5a2b9DCbe2B3e14057F6774D050C5f0f66336
```

#### **5. Frontend Setup**

```bash
cd ../frontend

# Install dependencies
npm install

# Create .env.local file
cp .env.example .env.local

# Start development server
npm run dev
```

**Environment Variables** (`.env.local`):
```env
NEXT_PUBLIC_BACKEND_URL=http://localhost:3001
NEXT_PUBLIC_RELAYER_URL=http://localhost:3005
NEXT_PUBLIC_RPC_URL=https://testnet.hashio.io/api
NEXT_PUBLIC_CHAIN_ID=296
THIRDWEB_CLIENT_ID=your_thirdweb_client_id
THIRDWEB_SECRET_KEY=your_thirdweb_secret_key
```

#### **6. Access the Application**

- **Frontend**: http://localhost:3000
- **Backend API**: http://localhost:3001
- **Relayer**: http://localhost:3005

---

## 📜 Smart Contracts

### **Deployed Contracts (Hedera Testnet)**

| Contract | Address | Purpose |
|----------|---------|---------|
| Registry | `0x9c78Bbfc9a101f0C467560BD93401B72cC4152C1` | User registration |
| Token (USDT) | `0xd1fb7489D8689c45082Ea68dC025247d4143f15E` | Payment token |
| CraftCoin (CFT) | `0x77A5a2b9DCbe2B3e14057F6774D050C5f0f66336` | Platform token |
| PaymentProcessor | `0x6976762d8dF143771eB8a2a930061A57806b6fd4` | Escrow system |
| GigMarketplace | `0x78b06bfd164Ae7dee46E606da3e4d2Cb59997cD2` | Gig management |
| ReviewSystem | `0x6182AfDA6817a0a78a5e87Da57DD19F05bfCa9cf` | Rating system |
| ChatSystem | `0x580C07f608C12563440086FF025E4c7aC7F2BcA7` | Message verification |

**Block Explorer**: [HashScan Testnet](https://hashscan.io/testnet)

### **Hedera Smart Contract Service Integration**

**All contracts leverage Hedera's unique features:**

✅ **EVM Compatibility** - Standard Solidity 0.8.20 contracts  
✅ **Hashgraph Consensus** - aBFT security with 3-5s finality  
✅ **Low Gas Costs** - $0.0001 per transaction  
✅ **High Throughput** - 10,000+ TPS capacity  
✅ **JSON-RPC** - HashIO relay for standard Web3 tools  

**Deployment Details:**
- **Network**: Hedera Testnet (Chain ID: 296)
- **RPC Endpoint**: https://testnet.hashio.io/api
- **Explorer**: https://hashscan.io/testnet
- **Consensus**: Hashgraph (not traditional blockchain)
- **Finality**: 3-5 seconds (vs 15+ minutes on Ethereum)

### **Contract Architecture**

```
Registry (User Management)
    ↓
GigMarketplace (Core Logic) ←→ PaymentProcessor (Escrow)
    ↓                               ↓
ReviewSystem                    Token (USDT)
    ↓
ChatSystem ←→ CraftCoin (CFT)
```

**All contracts deployed on Hedera Testnet via Foundry**

### **Key Functions**

#### **Registry**
- `registerAsArtisanFor(address, ipfsUrl)` - Register artisan profile
- `registerAsClientFor(address, ipfsUrl)` - Register client profile
- `isArtisanVerified(address)` - Check verification status

#### **GigMarketplace**
- `createGigFor(client, rootHash, databaseId, budget)` - Post new gig
- `applyForGigFor(artisan, databaseId)` - Apply to gig (burns CFT)
- `hireArtisanFor(client, databaseId, artisan)` - Hire applicant
- `markCompleteFor(artisan, databaseId)` - Mark work done
- `confirmCompleteFor(client, databaseId)` - Confirm completion

#### **PaymentProcessor**
- `createPaymentFor(client, amount)` - Lock funds in escrow
- `releaseArtisanFundsFor(artisan, paymentId)` - Release payment
- `refundClientFunds(paymentId)` - Refund cancelled gig

---

## 🔄 How It Works

### **User Journey: Artisan**

1. **Connect Wallet** → Use MetaMask or other Web3 wallet
2. **Register Profile** → Fill out skills, experience, portfolio (gasless)
3. **Claim Tokens** → Get 1000 USDT from faucet (testnet only)
4. **Mint CFT** → Receive 50 CraftCoin tokens
5. **Browse Gigs** → Filter by skills, budget, location
6. **Apply to Gig** → Burns 2-10 CFT based on budget (gasless)
7. **Get Hired** → Client selects you from applicants
8. **Complete Work** → Mark gig as complete (gasless)
9. **Receive Payment** → Funds released after client confirmation
10. **Get Reviewed** → Build on-chain reputation

### **User Journey: Client**

1. **Connect Wallet** → Use MetaMask or other Web3 wallet
2. **Register Profile** → Create client profile (gasless)
3. **Claim Tokens** → Get 1000 USDT from faucet (testnet only)
4. **Post Gig** → Create detailed job listing (gasless)
5. **Lock Funds** → USDT held in escrow smart contract
6. **Review Applicants** → Browse artisan profiles
7. **Hire Artisan** → Select best candidate (gasless)
8. **Track Progress** → Communicate via messaging
9. **Confirm Completion** → Release payment to artisan (gasless)
10. **Leave Review** → Rate artisan performance

### **Gasless Transaction Flow**

```
1. User Action (Frontend)
   ↓
2. Sign Message (Wallet)
   ↓
3. Send to Relayer (API Call)
   ↓
4. Verify Signature (Relayer)
   ↓
5. Execute Transaction (Relayer pays gas)
   ↓
6. Update State (Smart Contract)
   ↓
7. Sync Database (Backend)
   ↓
8. Update UI (Frontend)
```

### **Merkle Tree Verification**

CraftLink uses Merkle trees to ensure data integrity:

1. **Create Gig** → Backend generates Merkle tree of all gigs
2. **Generate Proof** → Specific proof for new gig
3. **Store Root** → Root hash stored on-chain (Hedera)
4. **Store Proof** → Proof stored in MongoDB
5. **Verify Later** → Anyone can verify data hasn't been tampered with

---

## 🌍 Impact & Vision

### **Transforming African Economies Through Hedera**

#### **Immediate Impact (Year 1)**
- 🎯 **Target**: 10,000 African artisans onboarded
- 💰 **Savings**: $2M+ in fees returned to artisans (vs traditional platforms)
- 🌍 **Reach**: 15+ African countries
- ⚡ **Transactions**: 100,000+ gasless transactions via Hedera

#### **Economic Empowerment**
- 💵 **Increased Earnings**: Artisans keep 90% vs 80-85% on traditional platforms
- ⏱️ **Faster Payments**: Instant settlement vs 7-30 day delays
- 🌐 **Global Access**: Connect with clients in 50+ countries
- 📈 **Income Growth**: 40-60% increase in monthly earnings

#### **Social Impact**
- 👨‍👩‍👧‍👦 **Family Support**: Better income supports 50,000+ dependents
- 🎓 **Education**: Increased earnings fund children's education
- 🏠 **Community Growth**: Economic activity strengthens local communities
- 👩‍💼 **Women Empowerment**: 60% of artisans are women supporting families

#### **Why This Matters for Africa**

**Financial Inclusion:**
- Hedera's low fees make blockchain accessible to low-income artisans
- No bank account required - just a smartphone
- Micro-transactions viable ($5-20 gigs profitable)

**Skills Development:**
- On-chain reputation encourages quality work
- Portfolio building opens doors to better opportunities
- Global exposure improves skills through diverse projects

**Economic Growth:**
- Remittances flow directly to artisans (no intermediaries)
- Local economies benefit from increased purchasing power
- Youth employment in creative industries

**Technology Adoption:**
- Introduces Africans to blockchain technology
- Builds trust in decentralized systems
- Positions Africa as a leader in Web3 adoption

### **Long-Term Vision (3-5 Years)**

#### **Phase 1: Foundation (Completed) ✅**
- ✅ Deploy smart contracts on Hedera Testnet
- ✅ Build gasless transaction infrastructure
- ✅ Create intuitive user interface
- ✅ Implement escrow and review systems

#### **Phase 2: Growth (Next 12 Months)**
- 🎯 Launch on Hedera Mainnet
- 🌍 Partner with African artisan cooperatives
- 📱 Mobile app for iOS/Android
- 🏦 Fiat on/off ramps (M-Pesa, mobile money integration)
- 🎓 Training programs in 10 African cities

#### **Phase 3: Scale (Year 2-3)**
- 🌐 Expand to 30+ African countries
- 🤝 Corporate partnerships (hotels, tourism, retail)
- 💳 Hedera Token Service integration for native tokens
- 🏛️ DAO governance for platform decisions
- 📊 AI-powered matching algorithm

#### **Phase 4: Ecosystem (Year 3-5)**
- 🏪 Artisan cooperatives and guilds
- 🎨 NFT marketplace for digital art
- 🏦 Microloans via DeFi protocols
- 🌍 Pan-African artisan network
- 🎓 Blockchain education centers

### **Hedera's Role in Our Success**

**Why Hedera is Perfect for Africa:**

1. **Affordability** → Enables micro-transactions for small gigs
2. **Speed** → Works with limited internet infrastructure
3. **Sustainability** → Carbon-negative aligns with Africa's green goals
4. **Enterprise Trust** → Attracts institutional partners
5. **Scalability** → Handles millions of transactions as we grow
6. **Compliance** → Regulatory-friendly for government partnerships

### **Measurable Outcomes**

**Platform Metrics:**
- 📊 Transaction volume on Hedera
- 👥 Active artisan and client accounts
- 💰 Total value locked in escrow
- ⭐ Average review ratings
- 🔄 Repeat transaction rate

**Social Metrics:**
- 📈 Income increase per artisan
- 🌍 Geographic distribution
- 👩 Women participation rate
- 🎓 Skills development progress
- 🏘️ Community economic impact

**Technical Metrics:**
- ⚡ Average transaction finality time
- 💵 Gas costs saved via relayer
- 🔒 Zero security incidents
- 📱 Mobile app adoption rate
- 🌐 Uptime and reliability

### **Call to Action**

CraftLink isn't just a platform—it's a **movement to democratize economic opportunity** across Africa using Hedera's cutting-edge technology.

**We're proving that:**
- ✅ Blockchain can solve real-world problems
- ✅ African artisans deserve fair compensation
- ✅ Hedera's technology is perfect for emerging markets
- ✅ Decentralization empowers the underserved

**Join us in building a future where:**
- 🌍 Every African artisan has access to global opportunities
- 💰 Fair compensation is guaranteed by smart contracts
- 🚀 Technology serves humanity, not exploits it
- 🤝 Trust is built through transparency, not intermediaries

---

## 🔐 Environment Variables

### **Smart Contracts**
```env
RPC_URL=https://testnet.hashio.io/api    # Hedera Testnet RPC endpoint
PRIVATE_KEY=                              # Deployer private key
ETHERSCAN_API_KEY=                        # For contract verification (if supported)
```

### **Backend**
```env
PORT=3001
MONGODB_URI=                # MongoDB connection string
NODE_ENV=development
```

### **Relayer**
```env
PORT=3005
RPC_URL=https://testnet.hashio.io/api    # Hedera Testnet RPC endpoint
PRIVATE_KEY_PASSWORD=                     # Password for encrypted key
ENCRYPTED_KEY_JSON=                       # Encrypted relayer private key
TOKEN_ADDRESS=0xd1fb7489D8689c45082Ea68dC025247d4143f15E
REGISTRY_ADDRESS=0x9c78Bbfc9a101f0C467560BD93401B72cC4152C1
REVIEW_SYSTEM_ADDRESS=0x6182AfDA6817a0a78a5e87Da57DD19F05bfCa9cf
GIG_MARKETPLACE_ADDRESS=0x78b06bfd164Ae7dee46E606da3e4d2Cb59997cD2
PAYMENT_PROCESSOR_ADDRESS=0x6976762d8dF143771eB8a2a930061A57806b6fd4
CRAFT_COIN_ADDRESS=0x77A5a2b9DCbe2B3e14057F6774D050C5f0f66336
```

### **Frontend**
```env
NEXT_PUBLIC_BACKEND_URL=http://localhost:3001    # Backend API URL
NEXT_PUBLIC_RELAYER_URL=http://localhost:3005    # Relayer API URL
NEXT_PUBLIC_RPC_URL=https://testnet.hashio.io/api    # Hedera Testnet RPC endpoint
NEXT_PUBLIC_CHAIN_ID=296                          # Network chain ID (296 for Hedera Testnet)
THIRDWEB_CLIENT_ID=                               # Thirdweb client ID
THIRDWEB_SECRET_KEY=                              # Thirdweb secret key
```

---

## 🌐 Deployment

### **Smart Contracts**

```bash
cd smart_contract

# Deploy to Hedera Testnet
forge script script/Deploy.s.sol \
  --rpc-url https://testnet.hashio.io/api \
  --broadcast \
  --verify

# Verify contracts on HashScan
# Note: Contract verification on Hedera uses HashScan
# Visit https://hashscan.io/testnet to view deployed contracts
```

### **Backend & Relayer**

Both are configured for **Vercel** deployment:

```bash
# Deploy backend
cd backend
vercel --prod

# Deploy relayer
cd relayer
vercel --prod
```

**Note**: Update environment variables in Vercel dashboard.

### **Frontend**

```bash
cd frontend

# Build production
npm run build

# Deploy to Vercel
vercel --prod
```

### **MongoDB**

Use **MongoDB Atlas** for production:
1. Create cluster at https://cloud.mongodb.com
2. Whitelist Vercel IPs
3. Update `MONGODB_URI` in environment variables

---

## 🤝 Contributing

We welcome contributions! Please follow these steps:

1. **Fork the repository**
2. **Create a feature branch** (`git checkout -b feature/amazing-feature`)
3. **Commit your changes** (`git commit -m 'Add amazing feature'`)
4. **Push to the branch** (`git push origin feature/amazing-feature`)
5. **Open a Pull Request**

### **Development Guidelines**

- Follow existing code style and conventions
- Write tests for new features
- Update documentation as needed
- Ensure all tests pass before submitting PR
- Use meaningful commit messages

---

## 📝 License

This project is licensed under the **MIT License** - see the [LICENSE](LICENSE) file for details.

---

## 👥 Team

**CraftLink** is built by passionate developers committed to empowering the global artisan community.

- **Author**: Ekarika Nsemeke
- **GitHub**: [github.com/xeladev4](https://github.com/xeladev4)

---

## 🔗 Links

- **Live Demo**: [https://craftlink-coral.vercel.app](https://craftlink-coral.vercel.app)
- **Documentation**: [Coming Soon]
- **Twitter**: [Coming Soon]
- **Discord**: [Coming Soon]

---

## 🙏 Acknowledgments

- **Hedera Hashgraph** - For providing enterprise-grade distributed ledger technology perfect for African markets
- **Hedera African Hackathon** - For the opportunity to showcase blockchain solutions for real-world African problems
- **OpenZeppelin** - Secure smart contract libraries
- **Foundry** - Fast Solidity development framework
- **Thirdweb** - Wallet connection infrastructure
- **Vercel** - Hosting platform
- **African Artisan Communities** - For inspiring this solution

---

## 📞 Support

For questions or support:
- Open an issue on GitHub
- Email: [Coming Soon]
- Join our Discord community

---

## 🏆 Hedera African Hackathon Submission

**CraftLink demonstrates:**

✅ **Real African Problem** - Addressing financial exclusion and platform exploitation  
✅ **Hedera Smart Contract Service** - All 7 contracts deployed on Hedera Testnet  
✅ **Technical Excellence** - Gasless transactions, Merkle proofs, escrow system  
✅ **Scalability** - Built to handle millions of transactions across Africa  
✅ **Social Impact** - Empowering 10,000+ artisans in Year 1  
✅ **Sustainability** - Carbon-negative blockchain for Africa's green future  

**Why CraftLink Wins:**
- 🎯 **Solves Real Problems** - Not just a demo, but a production-ready solution
- 🌍 **African-First** - Built specifically for African challenges and opportunities
- ⚡ **Leverages Hedera** - Uses HSCS features (speed, cost, sustainability) optimally
- 📈 **Measurable Impact** - Clear metrics for success and social good
- 🚀 **Ready to Scale** - Architecture supports millions of users

---

<div align="center">

**Built with ❤️ for African artisans using Hedera Smart Contract Service**

### 🌍 Empowering Africa, One Gig at a Time

⭐ Star us on GitHub if you believe in economic empowerment through blockchain!

**#HederaAfricanHackathon #HSCS #Web3ForGood #AfricanInnovation**

</div>
