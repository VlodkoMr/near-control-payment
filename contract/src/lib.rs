use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, AccountId, PanicOnDefault};

#[derive(Copy, Clone, Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
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

    pub fn get_tmp_list(&self) -> Vec<TmpOrder> {
        self.tmp_orders_list.to_vec()
    }

    #[payable]
    pub fn send_order_payment(&mut self, order_id: u32) {
        let mut tmp_list = self.tmp_orders_list.to_vec();
        tmp_list.push(TmpOrder {
            id: order_id,
            payment: env::attached_deposit(),
        });

        self.tmp_orders_list = tmp_list;
    }

    pub fn process_tmp_payments(&mut self) {
        assert_eq!(env::predecessor_account_id(), self.owner_id, "Unauthorized");
    }
}
