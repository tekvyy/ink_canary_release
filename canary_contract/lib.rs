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

        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(main_contract: AccountId, canary_contract: AccountId) -> Self {
            Self {
                admin: Self::env().caller(),
                main_contract,
                canary_contract,
                total_calls: 0,
                canary_percentage: 0,
            }
        }

        /// Simply returns the current value of prod contract
        #[ink(message)]
        pub fn get_main_contract(&self) -> AccountId {
            self.main_contract
        }

        #[ink(message)]
        pub fn get_total_calls(&self) -> i32 {
            self.total_calls
        }

        #[ink(message)]
        pub fn get_canary_contract(&self) -> AccountId {
            self.canary_contract
        }

        #[ink(message)]
        pub fn get_canary_percentage(&self) -> i32 {
            self.canary_percentage
        }

        #[ink(message)]
        pub fn change_main_contract_address(&mut self, new_address: AccountId) {
            assert_eq!(
                self.env().caller(),
                self.admin,
                "caller {:?} does not have sufficient permissions, only {:?} does",
                self.env().caller(),
                self.admin,
            );
            self.main_contract = new_address;
        }

        #[ink(message)]
        pub fn update_canary_percentage(&mut self, percentage: i32) {
            assert_eq!(
                self.env().caller(),
                self.admin,
                "caller {:?} does not have sufficient permissions, only {:?} does",
                self.env().caller(),
                self.admin,
            );
            self.canary_percentage = percentage;
        }

        #[ink(message)]
        pub fn change_canary_contract_address(&mut self, new_address: AccountId) {
            assert_eq!(
                self.env().caller(),
                self.admin,
                "caller {:?} does not have sufficient permissions, only {:?} does",
                self.env().caller(),
                self.admin,
            );
            self.canary_contract = new_address;
        }

        #[ink(message, payable)]
        pub fn forward(&mut self, method_name: Option<String>) -> Result<(),()> {
            self.total_calls += 1;
            let (contract_account, is_canary) = self.get_contract_account_according_to_percentage();
            let mut converted_method_name = ink::selector_bytes!("new");

            self.env().emit_event(CallForwardedInitiated {
                contract_account,
                is_canary,
            });

            match method_name {
                Some(s) => {
                    converted_method_name = ink::selector_bytes!("increment");
                }
                None => {
                    converted_method_name = ink::selector_bytes!("increment");
                }
            }

            let execution_result = ink::env::call::build_call::<ink::env::DefaultEnvironment>()
                .call_type(
                    Call::new(contract_account)
                        .transferred_value(self.env().transferred_value())
                        .gas_limit(0),
                )
                .call_flags(ink::env::CallFlags::default().set_tail_call(true))
                .exec_input(ExecutionInput::new(Selector::new(converted_method_name)))
                .returns::<bool>()
                .invoke();

            Ok(())
        }
    }
}