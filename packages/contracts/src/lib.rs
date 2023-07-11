#![cfg_attr(not(feature = "std"), no_std, no_main)]

mod patient_dot;

#[ink::contract]
pub mod health_dot {

    // use ink::storage::{
    //     collections::HashMap as StorageHashMap,
    //     traits::{PackedLayout, SpreadLayout},
    // };

    use ink::storage::Mapping;
    
    use scale::{
        Decode,
        Encode,
    };
    
    #[ink(storage)]
    #[derive(Default)]
    // pub struct Patient {
    //     pub uri: Option<String>,
    // }

    pub struct health_dot {
        patients: Mapping<u64, String>,
    }

    impl health_dot {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                patients: Default::default(),
            }
        }

        #[ink(message)]
        pub fn create_patient(&mut self, token_id: u64, uri: String) {
            self.patients.insert(token_id, &uri);
        }
    }


}