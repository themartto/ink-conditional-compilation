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
        
        // won't compile 
        // error[E0599]: no function or associated item named `test` found for struct `Foo` in the current scope
        // #[cfg(feature = "ink-debug")]
        // #[ink(message)]
        // pub fn test(&self) {
        //     debug_println!("yeah")
        // }
        
        
        // cfg!(feature = "ink-debug") will always return false no matter if --release flag is set or not
        #[ink(message)]
        pub fn test2(&self) {
            if cfg!(feature = "ink-debug") {
                debug_println!("DEBUG mode")    
            } else {
                debug_println!("RELEASE mode ")
            }
            
        }
    }
}
