use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, AccountId, PanicOnDefault};

#[derive(Debug, Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct TmpOrder {
    id: u32,
    payment: u128,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    owner_id: AccountId,
    tmp_orders_list: Vec<TmpOrder>,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new_default_meta(owner_id: AccountId) -> Self {
        Self {
            owner_id,
            tmp_orders_list: vec![],
        }
    }

    #[payable]
    pub fn send_order_payment(&mut self) {}

    pub fn process_tmp_payments(&mut self) {
        assert_eq!(env::predecessor_account_id(), self.owner_id, "Unauthorized");
    }
}
