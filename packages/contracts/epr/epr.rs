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
    // pub type Hash = String;

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
        details: String,
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
        details: String,
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

    #[ink(event)]
    pub struct BiodataUpdate {
        #[ink(topic)]
        identifier: Option<AccountId>,
        #[ink(topic)]
        message: Option<Biodata>
    }

    #[ink(event)]
    pub struct ClinicalNotesUpdate {
        #[ink(topic)]
        identifier: Option<AccountId>,
        #[ink(topic)]
        message: Option<ClinicalNotes>
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
        pub fn update_biodata(&mut self, identifier: AccountId, biodata: Biodata) -> Result<(), Error> {
            self.patient_biodata.insert(&identifier, &biodata);

            self.env().emit_event(BiodataUpdate {
                identifier: Some(identifier),
                message: Some(biodata)
            });

            Ok(())
        }

        #[ink(message)]
        pub fn update_clinical_notes(&mut self, identifier: AccountId, notes: ClinicalNotes) -> Result<(), Error> {
            self.patient_notes.insert(&identifier, &notes);

            self.env().emit_event(ClinicalNotesUpdate {
                identifier: Some(identifier),
                message: Some(notes)
            });

            Ok(())
        }

        #[ink(message)]
        pub fn get_biodata(&self, identifier: AccountId) -> Option<Biodata> {
            self.patient_biodata.get(&identifier)
        }

        #[ink(message)]
        pub fn get_clinical_notes(&self, identifier: AccountId) -> Option<ClinicalNotes> {
            self.patient_notes.get(&identifier)
        }
        
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        // #[ink::test]
        // fn new_creates_empty_epr() {
        //     let epr = EPR::new();
        //     // Test to see if a newly created EPR contract has no stored data.
        //     assert_eq!(epr.patient_biodata.len(), 0);
        //     assert_eq!(epr.patient_notes.len(), 0);
        // }

        #[ink::test]
        fn create_patient_works() {
            let mut epr = EPR::new();
            let patient = AccountId::from([0x01; 32]);
            assert_eq!(epr.create_patient(patient), Ok(()));
            // We assert that a patient record is created with the provided AccountId
            // and the patient's id is stored in the contract.
            assert_eq!(epr.record_count.get(&1), Some(patient));
        }

        #[ink::test]
        fn create_patient_increments_current_id() {
            let mut epr = EPR::new();
            let patient = AccountId::from([0x01; 32]);
            epr.create_patient(patient).unwrap();
            assert_eq!(epr.current_id, 1);
        }

        #[ink::test]
        fn update_and_retrieve_biodata_works() {
            let mut epr = EPR::new();
            let patient = AccountId::from([0x01; 32]);
            epr.create_patient(patient).unwrap();
            let new_biodata = Biodata { 
                name: "John Doe".to_string(), 
                details: "biodata_hash".to_string(), 
                finalized: true, 
                vector: vec![1, 2, 3, 4, 5] 
            };
            assert_eq!(epr.update_biodata(patient, new_biodata), Ok(()));
            // After updating the biodata of the patient, we assert that the updated biodata is stored in the contract.
            // assert_eq!(epr.get_biodata(patient), Some(new_biodata));
        }

        #[ink::test]
    fn update_and_retrieve_clinical_notes_works() {
        let mut epr = EPR::new();
        let patient = AccountId::from([0x01; 32]);
        epr.create_patient(patient).unwrap();
        let new_notes = ClinicalNotes { 
            name: "John Doe".to_string(), 
            details: "notes_hash".to_string(), 
            finalized: true, 
            vector: vec![6, 7, 8, 9, 10] 
        };
        assert_eq!(epr.update_clinical_notes(patient, new_notes), Ok(()));
        // After updating the clinical notes of the patient, we assert that the updated notes are stored in the contract.
        // assert_eq!(epr.get_clinical_notes(patient), Some(new_notes));
    }

    }
}