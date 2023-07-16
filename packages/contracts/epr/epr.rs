#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
pub mod epr {
    // use ink::storage::Mapping;
    use ink::prelude::string::String;
    use ink::prelude::vec::Vec;
    // use ink::env;

    use ink::storage::{
        traits::ManualKey,
        Mapping,
    };

    use scale::{
        Decode,
        Encode,
    };

    pub type HealthId = u32;

    #[derive(Default, scale::Decode, scale::Encode)]
    #[cfg_attr(
        feature = "std",
        derive(
            Debug,
            PartialEq,
            Eq,
            scale_info::TypeInfo,
            ink::storage::traits::StorageLayout
        )
    )]
    pub struct Biodata {
        name: String,
        details: String, //Hash
        finalized: bool,
        vector: Vec<u8>,
    }

    #[derive(Default, scale::Decode, scale::Encode)]
    #[cfg_attr(
        feature = "std",
        derive(
            Debug,
            PartialEq,
            Eq,
            scale_info::TypeInfo,
            ink::storage::traits::StorageLayout
        )
    )]
    pub struct ClinicalNotes {
        name: String,
        details: String, //Hash
        finalized: bool,
        vector: Vec<u8>,
    }

    #[ink(storage)]
    #[derive(Default)]
    pub struct EPR {
        record_count: Mapping<HealthId, AccountId>,
        patient_biodata: Mapping<AccountId, Biodata>,  
        patient_notes: Mapping<AccountId, ClinicalNotes>  
    }

    impl EPR {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                record_count: Default::default(),
                patient_biodata: Default::default(),
                patient_notes: Default::default(),
            }
        }

        #[ink(message)]
        pub fn create_patient(&mut self, identifier: AccountId) {
            // // Create an instance of the patient_token contract
            // let mut patient_token_instance = ink_env::call::FromAccountId::from_account_id(self.patient_token);

            // // Mint the token
            // patient_token_instance.mint(token_id);

            // // Store the health record
            // self.health_records.insert(token_id, health_record);
        }
        
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use ink_lang as ink;

    }
}