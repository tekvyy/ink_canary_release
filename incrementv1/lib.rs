#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod incrementv1 {


    #[ink(storage)]
    pub struct Incrementv1 {

        value: i32,
    }

    impl Incrementv1 {
        #[ink(constructor)]
        pub fn new(init_value: i32) -> Self {
            Self { value: init_value }
        }

        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(Default::default())
        }

        #[ink(message)]
        pub fn increment(&mut self) -> bool {
            self.value += 1;
            true
        }

        #[ink(message)]
        pub fn clear(&mut self) -> bool {
            self.value = 0;
            true
        }

        #[ink(message)]
        pub fn get(&self) -> i32 {
            self.value
        }
    }
}
