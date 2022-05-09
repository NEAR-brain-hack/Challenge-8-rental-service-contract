use crate::*;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct RentalReceipt {
    service: u64,
    owner: AccountId,
    stream_id: String,
    stopped_at: Option<u64>,
    created_at: u64,

}

impl Contract {
    pub(crate) fn internal_add_receipt_to_owner(
        &mut self,
        account_id: &AccountId,
        receipt_id: String,
    ) {
        let mut receipts = self.receipt_per_owner.get(account_id).unwrap_or_else(|| {
            UnorderedSet::new(
                StorageKeys::ReceiptPerOwnerInner {
                    account_id_hash: hash_account_id(&account_id),
                }
                .try_to_vec()
                .unwrap(),
            )
        });

        receipts.insert(&receipt_id);

        self.receipt_per_owner.insert(account_id, &receipts);
    }
}

#[near_bindgen]
impl Contract {
    #[payable]
    pub fn create_receipt(&mut self, service_id: u64, stream_id: String) {
        
        let account_id = env::predecessor_account_id();
        let initial_storage_usage = env::storage_usage();
        let receipt = RentalReceipt {
            service: service_id,
            owner: account_id.clone(),
            stream_id: stream_id.clone(),
            stopped_at: None,
            created_at: env::block_timestamp(),
        };

        assert!(self.receipts.insert(&stream_id, &receipt).is_none());

        self.internal_add_receipt_to_owner(&account_id, stream_id);

        let required_storage_in_bytes = env::storage_usage() - initial_storage_usage;

        let service = self.services.get(&service_id).expect("SERVICE_NOT_FOUND");
        refund_deposit(required_storage_in_bytes, service.deposit);
    }

    #[payable]
    pub fn refund_receipt_deposit(&mut self, receipt_id: String) {
        assert_one_yocto();
        let receipt = self.receipts.get(&receipt_id).expect("RECEIPT_NOT_FOUND");
        let service = self.services.get(&receipt.service).expect("SERVICE_NOT_FOUND");
        let deposit_value = service.deposit;
        let account_id = env::predecessor_account_id();
        assert_eq!(
            service.owner,
            account_id,
            "ONLY_SERVICE_OWNER"
        );
        Promise::new(receipt.owner).transfer(deposit_value);
    }

    pub fn close_receipt(&mut self, receipt_id: String) {
        let account_id = env::predecessor_account_id();
        let mut receipt = self.receipts.get(&receipt_id).expect("RECEIPT_NOT_FOUND");
        let service = self.services.get(&receipt.service).expect("SERVICE_NOT_FOUND");
        assert_eq!(
            service.owner,
            account_id,
            "ONLY_SERVICE_OWNER"
        );
        receipt.stopped_at = Some(env::block_timestamp());
        self.receipts.insert(&receipt_id, &receipt);
    }

    pub fn get_receipt(&self, receipt_id: String) -> Option<RentalReceipt> {
        self.receipts.get(&receipt_id)
    }

    //Query for all the tokens for an owner
    pub fn receipts_for_owner(
        &self,
        account_id: AccountId,
        from_index: Option<U128>,
        limit: Option<u64>,
    ) -> Vec<RentalReceipt> {
        //get the set of tokens for the passed in owner
        let receipts_for_owner_set = self.receipt_per_owner.get(&account_id);
        //if there is some set of tokens, we'll set the tokens variable equal to that set
        let receipt_ids = if let Some(receipts_for_owner_set) = receipts_for_owner_set {
            receipts_for_owner_set
        } else {
            //if there is no set of tokens, we'll simply return an empty vector. 
            return vec![];
        };

        //where to start pagination - if we have a from_index, we'll use that - otherwise start from 0 index
        let start = u128::from(from_index.unwrap_or(U128(0)));

        //iterate through the keys vector
        receipt_ids.iter()
            //skip to the index we specified in the start variable
            .skip(start as usize) 
            //take the first "limit" elements in the vector. If we didn't specify a limit, use 50
            .take(limit.unwrap_or(50) as usize) 
            //we'll map the token IDs which are strings into Json Tokens
            .map(|receipt_id| self.receipts.get(&receipt_id).unwrap())
            //since we turned the keys into an iterator, we need to turn it back into a vector to return
            .collect()
    }
}