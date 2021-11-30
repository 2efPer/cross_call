use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env,near_bindgen,ext_contract,Gas,PromiseResult};




#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Contract {}

#[ext_contract(cat_boss)]
pub trait CatBoss {
    fn on_visit(&self) -> String;
}

#[ext_contract(ext_self)]
pub trait MyContract {
    fn my_callback(&self) -> String;
}
const CONTRACT_ID: &str = "cat_boss.lagosss.testnet";


#[near_bindgen]
impl Contract {
    pub fn catboss_visit_call(&mut self){

        cat_boss::on_visit(   
            CONTRACT_ID.parse().unwrap(), 
            0,                             // attached yocto NEAR
            Gas::from(5_000_000_000_000),             // attached gas
        ).then(ext_self::my_callback(
            env::current_account_id(), // this contract's account id
            0, // yocto NEAR to attach to the callback
            Gas::from(5_000_000_000_000) // gas to attach to the callback
        ));


        
    }

    pub fn my_callback(&self) -> String {
        assert_eq!(
            env::promise_results_count(),
            1,
            "This is a callback method"
        );

        // handle the result from the cross contract call this method is a callback for
        match env::promise_result(0) {
            PromiseResult::NotReady => unreachable!(),
            PromiseResult::Failed => "Oops! Something wrong, You can't visit the cat boss.".to_string(),
            PromiseResult::Successful(result) => {
                let cat_boss_talk = near_sdk::serde_json::from_slice::<String>(&result).unwrap();
                format!("Hi, I'm a guard for my cat boss. I delivered your message to my cat boss,
                and here is the response: {}",cat_boss_talk)
            },
        }
    }

}