use crate::*;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Service {
    pub title: String,
    pub description: String,
    pub owner: AccountId,
    pub price_per_day: Balance,
    pub deposit: Balance
}

#[near_bindgen]
impl Contract {
    #[payable]
    pub fn create_service(
        &mut self, 
        title: String, 
        description: String, 
        price_per_day: U128,
        deposit: U128
    ) {
        let initial_storage_usage = env::storage_usage();
        let account_id = env::predecessor_account_id();
        let service = Service {
            title,
            description,
            owner: account_id,
            price_per_day: price_per_day.0,
            deposit: deposit.0,
        };
        self.service_serial += 1;
        self.services.insert(&self.service_serial, &service);

        let required_storage_in_bytes = env::storage_usage() - initial_storage_usage;

        refund_deposit(required_storage_in_bytes, 0u128);
    }

    pub fn get_services(&self, from_index: Option<U128>, limit: Option<u64>) -> Vec<(u64 ,Service)> {
        let start = u128::from(from_index.unwrap_or(U128(0)));
        self.services.keys()
            .skip(start as usize) 
            .take(limit.unwrap_or(50) as usize) 
            .map(|service_id| (service_id ,self.get_service(service_id.clone()).unwrap()))
            .collect()
    }

    pub fn get_service(&self, service_id: u64) -> Option<Service> {
        self.services.get(&service_id)
    }
}