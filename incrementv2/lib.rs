#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod incrementv2 {
    #[ink(storage)]
    pub struct Incrementv2 {
        value: i32,
    }

    impl Incrementv2 {
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
            self.value += 2;
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
