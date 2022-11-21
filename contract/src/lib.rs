use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{AccountId, env, near_bindgen, Promise, require};
use near_sdk::json_types::U128;

#[derive(Copy, Clone, Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct TmpOrder {
    order_id: u32,
    payment: U128,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    tmp_orders_list: Vec<TmpOrder>,
}

impl Default for Contract {
    fn default() -> Self {
        Self {
            tmp_orders_list: vec![],
        }
    }
}

#[near_bindgen]
impl Contract {
    pub fn get_tmp_list(&self) -> Vec<TmpOrder> {
        self.tmp_orders_list.to_vec()
    }

    #[payable]
    pub fn send_order_payment(&mut self, order_list: Vec<TmpOrder>, to_account: AccountId) -> Promise {
        let mut tmp_list = self.tmp_orders_list.to_vec();

        let mut total_pay = 0;
        for order in order_list.iter() {
            tmp_list.push(order.clone());
            total_pay += order.payment.0;
        }

        require!(total_pay == env::attached_deposit(), "Wrong payment amount");
        self.tmp_orders_list = tmp_list;

        Promise::new(to_account).transfer(env::attached_deposit())
    }

    pub fn cleanup_tmp_payments(&mut self, orders: Vec<u32>) {
        assert_eq!(env::predecessor_account_id(), env::current_account_id(), "Unauthorized");

        for order_id in orders {
            let index = self.tmp_orders_list.iter().position(|r| r.order_id == order_id);
            if index.is_some() {
                self.tmp_orders_list.remove(index.unwrap());
            }
        }
    }
}
