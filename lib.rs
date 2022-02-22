#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod foo {
    use ink_env::debug_println;
    
    #[ink(storage)]
    pub struct Foo {
        value: bool
    }

    impl Foo {
        #[ink(constructor)]
        pub fn new(value: bool) -> Self {
            Self {
                value
            }
        }

        #[ink(message)]
        #[cfg(feature = "ink-debug")]
        pub fn test(&self) {
            debug_println!("yeah")
        }
    }
}
