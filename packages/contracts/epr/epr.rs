#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
pub mod epr {
    use ink::storage::Mapping;
    use ink::prelude::string::String;
    use ink::prelude::vec::Vec;
    // use ink::env;

    use scale::{
        Decode,
        Encode,
    };

    pub type HealthId = u32;
    pub type Hash = String;

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
        details: Hash,
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
        details: Hash,
        finalized: bool,
        vector: Vec<u8>,
    }

    #[ink(event)]
    pub struct NewPatient {
        #[ink(topic)]
        id: HealthId,
        #[ink(topic)]
        identifier: Option<AccountId>
    }

    // Define an Error enum to handle errors.
    #[derive(Encode, Decode, Debug, PartialEq, Eq, Copy, Clone)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        NotOwner,
        NotApproved,
        TokenExists,
        TokenNotFound,
        NotAllowed,
        CannotFetchValue
    }

    #[ink(storage)]
    #[derive(Default)]
    pub struct EPR {
        current_id: HealthId,
        record_count: Mapping<HealthId, AccountId>,
        patient_biodata: Mapping<AccountId, Biodata>,  
        patient_notes: Mapping<AccountId, ClinicalNotes>  
    }

    impl EPR {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                current_id: 0,
                record_count: Default::default(),
                patient_biodata: Default::default(),
                patient_notes: Default::default(),
            }
        }

        #[ink(message)]
        pub fn create_patient(&mut self, identifier: AccountId) -> Result<(), Error> {
            let count = self.current_id + 1;
            
            self.current_id = count;
            self.record_count.insert(&count, &identifier);
        
            self.env().emit_event(NewPatient {
                id: count,
                identifier: Some(identifier)
            });

            Ok(())
        }

        #[ink(message)]
        pub fn update_biodata(&mut self, identifier: AccountId, biodata: Hash) -> Result<(), Error> {
            self.patient_biodata.insert(&identifier, &biodata);

            Ok(())
        }
        
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use ink_lang as ink;

    }
}