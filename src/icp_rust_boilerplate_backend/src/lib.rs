#[macro_use]
extern crate serde;
use candid::{Decode, Encode, Principal};
use ic_cdk::api::{caller, time};
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, Cell, DefaultMemoryImpl, StableBTreeMap, Storable};
use std::{borrow::Cow, cell::RefCell};

// Define types for stable memory
type Memory = VirtualMemory<DefaultMemoryImpl>;
type IdCell = Cell<u64, Memory>;

// Struct for storing an item
#[derive(candid::CandidType, Clone, Serialize, Deserialize)]
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

// Enum for the status of an item
#[derive(candid::CandidType, Serialize, Deserialize, Clone, PartialEq)]
enum ItemStatus {
    Lost,
    Found,
    Recovered,
}

// Struct for storing location details
#[derive(candid::CandidType, Clone, Serialize, Deserialize)]
struct Location {
    latitude: f64,
    longitude: f64,
}

// Struct for storing reward information
#[derive(candid::CandidType, Clone, Serialize, Deserialize)]
struct Reward {
    id: u64,
    item_id: u64,
    finder: Option<Principal>,
    amount: f64,
    status: RewardStatus,
}

// Enum for the status of a reward
#[derive(candid::CandidType, Serialize, Deserialize, Clone, PartialEq)]
enum RewardStatus {
    Available,
    Claimed,
    Distributed,
}

// Payload for reporting a new item
#[derive(candid::CandidType, Clone, Serialize, Deserialize)]
struct ReportItemPayload {
    name: String,
    description: String,
    location_last_seen: Location,
    nft_proof: String,
}

// Payload for claiming an item
#[derive(candid::CandidType, Clone, Serialize, Deserialize)]
struct ClaimItemPayload {
    item_id: u64,
    finder: Principal,
}

// Payload for claiming a reward
#[derive(candid::CandidType, Clone, Serialize, Deserialize)]
struct ClaimRewardPayload {
    reward_id: u64,
    finder: Principal,
}

// Implementing Storable for Item
impl Storable for Item {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Item {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

// Implementing Storable for Reward
impl Storable for Reward {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Reward {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

// Thread-local storage for memory management
thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    static ID_COUNTER: RefCell<IdCell> = RefCell::new(
        IdCell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))), 0)
            .expect("Cannot create a counter")
    );

    static ITEMS: RefCell<StableBTreeMap<u64, Item, Memory>> = RefCell::new(StableBTreeMap::init(
        MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1)))
    ));

    static REWARDS: RefCell<StableBTreeMap<u64, Reward, Memory>> = RefCell::new(StableBTreeMap::init(
        MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(2)))
    ));
}

// Function to report a new item as lost
#[ic_cdk::update]
fn report_item(payload: ReportItemPayload) -> Result<Item, String> {
    // Validate input data
    if payload.name.trim().is_empty()
        || payload.description.trim().is_empty()
        || payload.nft_proof.trim().is_empty()
        || payload.location_last_seen.latitude.abs() > 90.0
        || payload.location_last_seen.longitude.abs() > 180.0
    {
        return Err("Invalid input data provided.".to_string());
    }

    // Generate a new ID for the item
    let item_id = ID_COUNTER.with(|counter| {
        let current_value = *counter.borrow().get();
        counter.borrow_mut().set(current_value + 1).unwrap();
        current_value
    });

    // Create a new item
    let new_item = Item {
        id: item_id,
        owner: caller(),
        name: payload.name,
        description: payload.description,
        location_last_seen: payload.location_last_seen,
        nft_proof: payload.nft_proof,
        status: ItemStatus::Lost,
        timestamp: time(),
    };

    // Store the item
    ITEMS.with(|items| items.borrow_mut().insert(item_id, new_item.clone()));
    Ok(new_item)
}

// Function to claim an item as found
#[ic_cdk::update]
fn claim_item(payload: ClaimItemPayload) -> Result<Item, String> {
    // Retrieve and update the item
    ITEMS.with(|items| {
        let mut items_ref = items.borrow_mut();
        if let Some(mut item) = items_ref.get(&payload.item_id) {
            if item.status != ItemStatus::Lost {
                return Err("Item is not marked as lost.".to_string());
            }
            item.status = ItemStatus::Found;
            items_ref.insert(payload.item_id, item.clone());
            Ok(item)
        } else {
            Err("Item not found.".to_string())
        }
    })
}

// Function to claim a reward
#[ic_cdk::update]
fn claim_reward(payload: ClaimRewardPayload) -> Result<Reward, String> {
    // Retrieve and update the reward
    REWARDS.with(|rewards| {
        let mut rewards_ref = rewards.borrow_mut();
        if let Some(mut reward) = rewards_ref.get(&payload.reward_id) {
            if reward.status != RewardStatus::Available {
                return Err("Reward is not available.".to_string());
            }
            reward.status = RewardStatus::Claimed;
            reward.finder = Some(payload.finder);
            rewards_ref.insert(payload.reward_id, reward.clone());
            Ok(reward)
        } else {
            Err("Reward not found.".to_string())
        }
    })
}

// Export candid definitions
ic_cdk::export_candid!();
