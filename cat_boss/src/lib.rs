use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen,AccountId};
use near_sdk::collections::LookupMap;
use std::convert::TryFrom;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    visit_counts: LookupMap<AccountId,i128>,
}


impl Default for Contract {
    fn default() -> Self {
        Self {
            visit_counts: LookupMap::new(b"v".to_vec()),
        }
    }
}

#[near_bindgen]
impl Contract {


    pub fn on_visit(&mut self) -> String {
        assert_eq!(
            env::predecessor_account_id(),
            AccountId::try_from("dog_guard.lagosss.testnet".to_string()).unwrap(),
            "mewwwww! How dare you interrupt my sleep.You should talk to my dog guard!"
        );
        let user_name = env::signer_account_id();
        let visit_cnt = self.visit_counts.get(&env::signer_account_id());
        let visit_msg:String = match visit_cnt{
            Some(num) => {
                let new_num= num+1;
                self.visit_counts.insert(&env::signer_account_id(),&new_num);
                format!("It's your {} time to visit me, I remember you ",new_num)
            },
            None => {
                self.visit_counts.insert(&env::signer_account_id(),&1);
                "It's your first time to visit me".to_owned()
            }
        };
        let mood:String = if env::block_timestamp()%2==0{
            "huh, I'm not happy right now ,levae me alone,human".to_owned()
        }else{
            "meww, I'm in a good mood ".to_owned()
        };
        format!("{} ,{}, {}",visit_msg,&user_name,mood)
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};

    fn get_context(input: Vec<u8>, is_view: bool,payment:u128) -> VMContext {
        VMContext {
            current_account_id: "dev-1638263753737-72501332295036".to_string(),
            signer_account_id: "bob_near".to_string(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id: "bob_near".to_string(),
            input,
            block_index: 0,
            epoch_height: 0,
            block_timestamp: 0,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage: 0,
            attached_deposit: payment,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view,
            output_data_receivers: vec![],
        }
    }

    #[test]
    fn nonexisting_pool() {
        let context = get_context(vec![], false,0);
        testing_env!(context);
        let mut contract = Contract::default();
        let options1 = contract.on_visit();
        print!("{}",options1);
        let options2 = contract.on_visit();
        print!("{}",options2);
    }
}
