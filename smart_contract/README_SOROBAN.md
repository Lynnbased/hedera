# Craft Platform - Soroban Smart Contracts

This directory contains the Stellar/Soroban smart contracts for the Craft Platform, converted from the original Solidity implementation.

## Contracts

### 1. Registry Contract (`registry.rs`)
Manages user registration for artisans and clients.
- Register as artisan or client
- Verify artisans
- Query user details and counts

### 2. Token Contract (`token.rs`)
USDT-like token for payments.
- ERC20-compatible token interface
- Claim tokens for new users
- Relayer-approved operations

### 3. CraftCoin Contract (`craft_coin.rs`)
Platform token for artisans.
- ERC20-compatible token interface
- Time-gated minting (30-day intervals)
- Burn mechanism for gig applications

### 4. PaymentProcessor Contract (`payment_processor.rs`)
Handles payment processing and escrow.
- Create payments with platform fees
- Release funds to artisans
- Refund clients for cancelled gigs
- Track spending and earnings

### 5. GigMarketplace Contract (`gig_marketplace.rs`)
Core marketplace functionality.
- Create and manage gigs
- Apply for gigs
- Hire artisans
- Track gig completion status
- Calculate required CraftCoin stakes

### 6. ReviewSystem Contract (`review_system.rs`)
Review and rating system.
- Submit reviews (both client and artisan)
- Calculate average ratings
- Query review history

### 7. ChatSystem Contract (`chat_system.rs`)
Decentralized chat system.
- Start conversations between clients and artisans
- Update conversation root hashes
- Query conversation status

## Building

```bash
# Install Soroban CLI
cargo install soroban-cli

# Build the contracts
cargo build --target wasm32-unknown-unknown --release

# The compiled WASM files will be in target/wasm32-unknown-unknown/release/
```

## Deployment

```bash
# Deploy to testnet
soroban contract deploy --wasm target/wasm32-unknown-unknown/release/craft_platform.wasm --source <your-key> --network testnet

# Initialize contracts (example for Registry)
soroban contract invoke \
  --id <contract-id> \
  --source <your-key> \
  --network testnet \
  -- \
  initialize \
  --relayer <relayer-address>
```

## Cross-Contract Calls

Note: The current implementation uses placeholder comments for cross-contract calls. In a production environment, you'll need to implement proper cross-contract calls using the Soroban SDK's contract client functionality.

Example of how to implement cross-contract calls:

```rust
use soroban_sdk::contractclient;

#[contractclient(name = "RegistryClient")]
trait RegistryInterface {
    fn is_artisan(&self, env: &Env, artisan_address: Address) -> bool;
}

// Usage in another contract:
let registry_client = RegistryClient::new(&env, &registry_address);
let is_artisan = registry_client.is_artisan(&env, &user_address);
```

## Testing

```bash
# Run tests
cargo test

# Run specific contract tests
cargo test --package craft_platform --lib registry::tests
```

## Architecture Notes

- **Storage**: Uses Soroban's persistent storage with instance and temporary storage
- **Events**: All state changes emit events for off-chain indexing
- **Access Control**: Relayer-based access control pattern (similar to the Solidity implementation)
- **Token Standards**: Implements Soroban Token Interface (SIP-3 compatible)

## Migration from Solidity

Key differences from the Solidity version:
- Uses Rust instead of Solidity
- Soroban SDK for blockchain interactions
- Different storage patterns (instance vs contract storage)
- Cross-contract calls require explicit contract client setup
- Events use Soroban's event system
- No gas optimization concerns (Soroban uses different fee model)
