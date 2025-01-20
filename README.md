# Lost and Found Management System

A decentralized lost and found management system built on the Internet Computer (ICP) blockchain. This system allows users to report lost items, claim found items, and manage rewards for item recovery.

## Features

- Report lost items with detailed information
- Claim found items with proof verification
- Reward system for item finders
- Location tracking for lost items
- NFT-based proof system for item verification
- Real-time status tracking (Lost, Found, Recovered)

## System Architecture

The system uses the Internet Computer's stable storage patterns with the following main components:

- `Item`: Manages lost item information and status
- `Reward`: Handles reward distribution for found items
- `Location`: Tracks geographical coordinates of lost/found items
- `StableBTreeMap`: Provides persistent storage for items and rewards

## Prerequisites

- Rust
- Internet Computer SDK (DFX)
- Candid Interface Description Language
- Node.js (for frontend development)

## Installation

1. Install the DFINITY Canister SDK:
```bash
sh -ci "$(curl -fsSL https://internetcomputer.org/install.sh)"
```

2. Clone the repository:
```bash
git clone https://github.com/kilingijanet/lost-and-found-platform.git
cd lost-and-found-platform
```

3. Start the local Internet Computer network:
```bash
dfx start --background --clean
```

4. Deploy the canister:
```bash
dfx deploy
```

## Usage

### Reporting a Lost Item

```rust
let report_payload = ReportItemPayload {
    name: "Gold Watch",
    description: "18k gold watch with leather strap",
    location_last_seen: Location {
        latitude: 40.7128,
        longitude: -74.0060
    },
    nft_proof: "proof_hash_string"
};

dfx canister call lost_found report_item '(report_payload)'
```

### Claiming a Found Item

```rust
let claim_payload = ClaimItemPayload {
    item_id: 1,
    finder: Principal::from_text("finder-principal-id")
};

dfx canister call lost_found claim_item '(claim_payload)'
```

### Claiming a Reward

```rust
let reward_payload = ClaimRewardPayload {
    reward_id: 1,
    finder: Principal::from_text("finder-principal-id")
};

dfx canister call lost_found claim_reward '(reward_payload)'
```

## Data Structures

### Item
```rust
struct Item {
    id: u64,
    owner: Principal,
    name: String,
    description: String,
    location_last_seen: Location,
    nft_proof: String,
    status: ItemStatus,
    timestamp: u64,
}
```

### Reward
```rust
struct Reward {
    id: u64,
    item_id: u64,
    finder: Option<Principal>,
    amount: f64,
    status: RewardStatus,
}
```

## API Methods

- `report_item`: Report a new lost item
- `claim_item`: Claim a found item
- `claim_reward`: Claim a reward for finding an item

## Security

The system implements several security measures:

- Principal-based authentication
- NFT-based proof system for item verification
- Status tracking to prevent duplicate claims
- Stable storage for data persistence

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Support

For support, please open an issue in the GitHub repository or contact the development team.