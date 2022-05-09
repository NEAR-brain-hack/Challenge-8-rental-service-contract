use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{UnorderedMap, UnorderedSet, LookupMap};
use near_sdk::serde::{Serialize, Deserialize};
use near_sdk::{env, BorshStorageKey, near_bindgen, AccountId, PanicOnDefault, Balance, Promise, CryptoHash};
use near_sdk::json_types::{U128};

pub use crate::service::*;
pub use crate::receipt::*;
use crate::internal::*;

mod service;
mod receipt;
mod internal;

// const ONE_DAY_NANOSECOND: u64 = 86400000000000;

#[derive(BorshStorageKey, BorshSerialize)]
pub enum StorageKeys {
    Service,
    Receipt,
    ReceiptPerOwner,
    ReceiptPerOwnerInner {
        account_id_hash: CryptoHash
    }
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    pub owner_id: AccountId,
    pub services: UnorderedMap<u64, Service>,
    pub receipts: UnorderedMap<String, RentalReceipt>,
    pub receipt_per_owner: LookupMap<AccountId, UnorderedSet<String>>,
    pub service_serial: u64
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(owner_id: AccountId) -> Self {
        let this = Self {
            owner_id,
            services: UnorderedMap::new(StorageKeys::Service),
            receipts:  UnorderedMap::new(StorageKeys::Receipt),
            receipt_per_owner: LookupMap::new(StorageKeys::ReceiptPerOwner),
            service_serial: 0
        };
        this
    }

    #[private]
    #[init(ignore_state)]
    pub fn migrate() -> Self {
        let old_state: Contract = env::state_read().expect("failed");
        assert_eq!(
            env::predecessor_account_id(), 
            old_state.owner_id
        );
        old_state
    }
}