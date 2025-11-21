# 🎨 CraftLink

> **Empowering Artisans Worldwide Through Hedera Smart Contract Service**

**Built for Hedera Ascension Hackathon**

CraftLink is a decentralized marketplace leveraging **Hedera Smart Contract Service (HSCS)** to bridge the gap between skilled artisans and global clients. By harnessing Hedera's fast, fair, and secure distributed ledger technology, we provide gasless transactions, secure escrow payments, and cryptographic verification to ensure trust, transparency, and fair compensation for creative work across **emerging markets and underserved communities globally**.

[](https://opensource.org/licenses/MIT)
[](https://soliditylang.org/)
[](https://nextjs.org/)
[](https://www.typescriptlang.org/)
[](https://hedera.com/)

-----

 📊 **[Pitch Deck](https://gamma.app/docs/CraftLink-wrpqpq1ykrbe7yo?mode=doc)** | 🔗 **[Live Demo](https://craftlink-coral.vercel.app)**

## 📋 Table of Contents

  - [Overview](https://www.google.com/search?q=%23-overview)
  - [The Global Problem](https://www.google.com/search?q=%23-the-global-problem)
  - [Why Hedera?](https://www.google.com/search?q=%23-why-hedera)
  - [Key Features](https://www.google.com/search?q=%23-key-features)
  - [Architecture](https://www.google.com/search?q=%23-architecture)
  - [Tech Stack](https://www.google.com/search?q=%23-tech-stack)
  - [Project Structure](https://www.google.com/search?q=%23-project-structure)
  - [Getting Started](https://www.google.com/search?q=%23-getting-started)
  - [Smart Contracts](https://www.google.com/search?q=%23-smart-contracts)
  - [How It Works](https://www.google.com/search?q=%23-how-it-works)
  - [Impact & Vision](https://www.google.com/search?q=%23-impact--vision)
  - [Environment Variables](https://www.google.com/search?q=%23-environment-variables)
  - [Deployment](https://www.google.com/search?q=%23-deployment)
  - [Contributing](https://www.google.com/search?q=%23-contributing)
  - [License](https://www.google.com/search?q=%23-license)

-----

## 🌟 Overview

CraftLink is a **Hedera-powered decentralized marketplace** specifically designed to empower **artisans in emerging markets** by connecting them with global opportunities while eliminating the barriers that traditional platforms impose.

### **Built on Hedera Smart Contract Service**

We leverage Hedera's enterprise-grade distributed ledger technology to provide:

  - ⚡ **Lightning-fast transactions** (3-5 second finality)
  - 💰 **Low, predictable fees** (fractions of a cent per transaction)
  - 🔒 **Bank-grade security** with aBFT consensus
  - 🌍 **Carbon-negative network** (sustainable for the future)
  - 🔗 **EVM compatibility** (deploy standard Solidity contracts)

-----

## 🌍 The Global Problem

### **Current Challenges Facing Artisans in Emerging Markets:**

#### **1. Financial Exclusion**

  - 📊 **High rates of unbanked** populations in target regions.
  - 💳 Traditional platforms require bank accounts and credit cards
  - 💸 International payment processors charge **5-15% fees** for cross-border payments
  - ⏰ Payment delays of **7-30 days** for international transactions

#### **2. Platform Exploitation**

  - 💰 Existing freelance platforms charge **15-20% commission**
  - 🚫 Geographic and regulatory restrictions limit global participation
  - 📉 Currency conversion losses eat into earnings
  - ⚖️ Dispute resolution often favors clients over independent creators

#### **3. Trust & Verification**

  - 🤝 Lack of verifiable, global reputation systems
  - 📄 Portfolio theft and credential fraud
  - 💔 Payment defaults with no recourse, especially across borders
  - 🌐 Limited access to premium global markets

#### **4. Technical Barriers**

  - 📱 High gas fees on traditional blockchains (Ethereum: $5-50 per transaction)
  - 🔧 Complex wallet setup and crypto knowledge required
  - 🌐 Poor internet infrastructure in rural areas
  - ⚡ Slow transaction speeds on congested networks

### **The Impact:**

Skilled artisans worldwide—from textile workers in Asia to metalworkers in Latin America, and graphic designers across all emerging economies—lose **30-40% of their earnings** to fees, delays, and exploitation. This perpetuates financial hardship and limits economic opportunity.

-----

## 🚀 Why Hedera?

We chose **Hedera Smart Contract Service** specifically to address the challenges in global emerging markets:

### **1. Affordability**

  - 💵 **$0.0001 per transaction** vs $5-50 on Ethereum
  - 💰 Enables **micro-payments** for small gigs ($5-20), making them profitable
  - 📊 Predictable costs allow artisans to plan budgets reliably

### **2. Speed & Reliability**

  - ⚡ **3-5 second finality** vs 15+ minutes on other chains
  - 🔄 **10,000+ TPS** handles high transaction volume, crucial for a global user base
  - 📶 Works efficiently even with limited internet bandwidth

### **3. Accessibility**

  - 🌐 **Gasless transactions** via meta-transaction relayer
  - 📱 No crypto knowledge required for end users
  - 🔗 Multi-wallet support (MetaMask, mobile wallets)

### **4. Sustainability**

  - 🌱 **Carbon-negative network** aligns with global climate goals
  - ♻️ Energy-efficient consensus (no mining)
  - 🌍 Supports the growth of a sustainable global economy

### **5. Enterprise Trust**

  - 🏛️ Governed by global enterprises (Google, IBM, Boeing)
  - 🔒 Bank-grade security (aBFT consensus)
  - 📜 Regulatory compliance for institutional and corporate adoption

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
**RPC:** [https://testnet.hashio.io/api](https://testnet.hashio.io/api)  
**Explorer:** [https://hashscan.io/testnet](https://hashscan.io/testnet)

-----

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
  - ✅ **Quality assurance** - Review system filters top talent globally
  - ✅ **Transparent pricing** - No hidden fees
  - ✅ **Direct communication** - Built-in messaging

### **Technical Innovation:**

  - 🔐 **Merkle tree verification** - Tamper-proof data integrity
  - 🎫 **Meta-transactions** - Gasless UX via relayer
  - 🪙 **Token economics** - CFT prevents spam applications
  - 📦 **Hybrid storage** - Critical data on-chain, media on IPFS

-----

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

-----

## 🏗️ Architecture

CraftLink uses a **hybrid architecture** combining on-chain and off-chain components:

```
┌─────────────────────────────────────────────────────────────┐
│                         Frontend                             │
│              (Next.js + React + Thirdweb)                   │
└─────────────┬───────────────────────────┬───────────────────┘
              │                           │
              ▼                           ▼
    ┌─────────────────┐         ┌─────────────────┐
    │     Backend     │         │     Relayer     │
    │  (Express API)  │         │ (Meta-Tx Server)│
    └────────┬────────┘         └────────┬────────┘
             │                           │
             ▼                           ▼
    ┌─────────────────┐         ┌─────────────────┐
    │    MongoDB      │         │  Smart Contracts│
    │  (Off-chain DB) │         │ (Hedera Testnet)│
    └─────────────────┘         └─────────────────┘
             │                           │
             └───────────┬───────────────┘
                         ▼
                ┌─────────────────┐
                │  IPFS Storage   │
                │  (Media Files)  │
                └─────────────────┘
```

### **Data Flow**

1.  **User Action** → Frontend captures user input
2.  **Sign Message** → User signs transaction with wallet
3.  **Relayer Execution** → Relayer submits transaction on-chain
4.  **Blockchain Update** → Smart contract state changes
5.  **Backend Sync** → Backend verifies and stores detailed data
6.  **Merkle Verification** → Cryptographic proof ensures data integrity

-----

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

-----

## 📁 Project Structure

```
CraftLink/
├── smart_contract/          # Solidity smart contracts
│   ├── src/
│   │   ├── Registry.sol           # User registration
...
│
├── relayer/                 # Meta-transaction relayer
...
│
├── backend/                 # Express API server
...
│
├── frontend/                # Next.js application
...
│
└── README.md                # This file
```

-----

## 🚀 Getting Started

### **Prerequisites**

  - **Node.js** v18+ and npm/yarn
  - **MongoDB** (local or Atlas)
  - **Git**
  - **Foundry** (for smart contracts)
  - **MetaMask** or compatible Web3 wallet

*(Installation steps remain the same)*

-----

## 📜 Smart Contracts

*(Contract details remain the same)*

### **Contract Architecture**

```
Registry (User Management)
    ↓
GigMarketplace (Core Logic) ←→ PaymentProcessor (Escrow)
    ↓                               ↓
ReviewSystem                    Token (USDT)
    ↓
ChatSystem ←→ CraftCoin (CFT)
```

-----

## 🔄 How It Works

*(User Journey: Artisan and Client steps remain the same)*

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

1.  **Create Gig** → Backend generates Merkle tree of all gigs
2.  **Generate Proof** → Specific proof for new gig
3.  **Store Root** → Root hash stored on-chain (Hedera)
4.  **Store Proof** → Proof stored in MongoDB
5.  **Verify Later** → Anyone can verify data hasn't been tampered with

-----

## 🌍 Impact & Vision

### **Transforming Global Economies Through Hedera**

#### **Immediate Impact (Year 1)**

  - 🎯 **Target**: 10,000 artisans from emerging markets onboarded
  - 💰 **Savings**: $2M+ in fees returned to artisans (vs traditional platforms)
  - 🌍 **Reach**: 15+ countries globally
  - ⚡ **Transactions**: 100,000+ gasless transactions via Hedera

#### **Economic Empowerment**

  - 💵 **Increased Earnings**: Artisans keep 90% vs 80-85% on traditional platforms
  - ⏱️ **Faster Payments**: Instant settlement vs 7-30 day delays for international payments
  - 🌐 **Global Access**: Connect with clients in 50+ countries
  - 📈 **Income Growth**: 40-60% increase in monthly earnings for underserved communities

#### **Social Impact**

  - 👨‍👩‍👧‍👦 **Family Support**: Better income supports thousands of dependents
  - 🎓 **Education**: Increased earnings fund education and skills development
  - 🏠 **Community Growth**: Economic activity strengthens local communities globally
  - 👩‍💼 **Empowerment**: Providing a fair, transparent platform for all, especially marginalized groups

#### **Why This Matters for Emerging Markets**

**Financial Inclusion:**

  - Hedera's low fees make blockchain accessible to low-income artisans
  - No bank account required - just a smartphone/web access
  - Micro-transactions viable ($5-20 gigs profitable)

**Skills Development:**

  - On-chain reputation encourages quality work and professionalism
  - Portfolio building opens doors to better global opportunities

**Economic Growth:**

  - Cross-border remittances flow directly to artisans (no intermediaries)
  - Local economies benefit from increased purchasing power
  - Fosters youth employment in creative industries

*(Long-Term Vision, Hedera's Role, and Measurable Outcomes sections remain the same, with adjusted language for "global" instead of "African")*

-----

## 🔐 Environment Variables

*(This section remains the same)*

-----

## 🌐 Deployment

*(This section remains the same)*

-----

## 🤝 Contributing

*(This section remains the same)*

-----

## 📝 License

*(This section remains the same)*

-----

## 👥 Team

**CraftLink** is built by passionate developers committed to empowering the global artisan community.

  - **Author**: Ebele Lynda Okolo
  - **GitHub**: [https://github.com/Lynnbased/hedera]

-----

## 🔗 Links

  - **Live Demo**: [https://craftlink-coral.vercel.app](https://craftlink-coral.vercel.app)
  - **Documentation**: [Coming Soon]
  - **Twitter**: [Coming Soon]
  - **Discord**: [Coming Soon]

-----

## 🙏 Acknowledgments

  - **Hedera Hashgraph** - For providing enterprise-grade distributed ledger technology perfect for emerging markets
  - **Hedera Ascension Hackathon** - For the opportunity to showcase blockchain solutions for real-world global problems
  - **OpenZeppelin** - Secure smart contract libraries
  - **Foundry** - Fast Solidity development framework
  - **Thirdweb** - Wallet connection infrastructure
  - **Vercel** - Hosting platform
  - **Global Artisan Communities** - For inspiring this solution

-----

## 📞 Support

*(This section remains the same)*

-----

## 🏆 Hedera Ascension Hackathon Submission

**CraftLink demonstrates:**

✅ **Real Global Problem** - Addressing financial exclusion and platform exploitation in emerging markets  
✅ **Hedera Smart Contract Service** - All 7 contracts deployed on Hedera Testnet  
✅ **Technical Excellence** - Gasless transactions, Merkle proofs, escrow system  
✅ **Scalability** - Built to handle millions of transactions globally  
✅ **Social Impact** - Empowering 10,000+ artisans in Year 1  
✅ **Sustainability** - Carbon-negative blockchain for a greener future  

**Why CraftLink Wins:**

  - 🎯 **Solves Real Problems** - Not just a demo, but a production-ready solution
  - 🌍 **Globally Relevant** - Built specifically for the challenges and opportunities in emerging markets
  - ⚡ **Leverages Hedera** - Uses HSCS features (speed, cost, sustainability) optimally
  - 📈 **Measurable Impact** - Clear metrics for success and social good
  - 🚀 **Ready to Scale** - Architecture supports millions of users

-----

\<div align="center"\>


### 🌍 Empowering Global Creators, One Gig at a Time

⭐ Star us on GitHub if you believe in economic empowerment through blockchain\!

**\#HederaAscensionHackathon \#HSCS \#Web3ForGood \#GlobalInnovation**

\</div\>

-----
