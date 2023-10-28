#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod canary_contract {
    use ink::env::call::{Call, ExecutionInput, Selector};
    use ink::prelude::string::String;

    #[ink(storage)]
    pub struct CanaryContract {
        main_contract: AccountId,
        canary_contract: AccountId,
        admin: AccountId,
        total_calls: i32,
        canary_percentage: i32,
    }

    #[ink(event)]
    pub struct CallForwardedInitiated {
        contract_account: AccountId,
        is_canary: bool,
    }

    impl CanaryContract {
        
    }
}