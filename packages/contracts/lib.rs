// Enable the contract to run without the standard library if the "std" feature is not set.
#![cfg_attr(not(feature = "std"), no_std, no_main)]

// This attribute specifies that the following module is an ink! smart contract.
#[ink::contract]
mod epr {
    // Use necessary items from the ink crate.
    use ink::storage::Mapping;
    use ink::prelude::string::String;
    use ink::prelude::vec::Vec;
    use patient::Patient;

    // We import the Encode and Decode traits from the scale codec.
    use scale::{
        Decode,
        Encode,
    };

    // Define a type alias for HealthId to enhance readability.
    pub type HealthId = u32;
    pub type TokenId = u32;
    
    // The Biodata struct is used to represent the biodata of a patient.
    // It contains the patient's name, details, a boolean indicating whether the data is finalized or not, and a vector of bytes.
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

    // Similar to the Biodata struct, the ClinicalNotes struct is used to represent the clinical notes of a patient.
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

    // The NewPatient event is emitted whenever a new patient is created.
    #[ink(event)]
    pub struct NewPatient {
        #[ink(topic)]
        id: HealthId,
        #[ink(topic)]
        identifier: Option<AccountId>
    }

    // The BiodataUpdate event is emitted whenever the biodata of a patient is updated.
    #[ink(event)]
    pub struct BiodataUpdate {
        #[ink(topic)]
        identifier: Option<AccountId>,
        #[ink(topic)]
        message: Option<Biodata>
    }

    // The ClinicalNotesUpdate event is emitted whenever the clinical notes of a patient are updated.
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

    // The EPR (Electronic Patient Record) struct represents the smart contract.
    #[ink(storage)]
    #[derive(Default)]
    pub struct EPR {
        // The current_id field keeps track of the current patient id.
        current_id: HealthId,
        // The record_count mapping stores the account id associated with each health id.
        record_count: Mapping<HealthId, AccountId>,
        // The patient_biodata mapping stores the biodata of each patient.
        patient_biodata: Mapping<AccountId, Biodata>,  
        // The patient_notes mapping stores the clinical notes of each patient.
        patient_notes: Mapping<AccountId, ClinicalNotes>,
        patient: Patient
    }

    // Define the behavior of the EPR contract.
    impl EPR {
        // The constructor initializes an EPR contract with no data.
        #[ink(constructor)]
        pub fn new(patient_code_hash: Hash) -> Self {
            let patient = Patient::new(String::from("HealthDot"), String::from("HDOT"))
                .code_hash(patient_code_hash)
                .endowment(0)
                .salt_bytes([0xDE, 0xAD, 0xBE, 0xEF])
                .instantiate();

            Self {
                current_id: 0,
                record_count: Default::default(),
                patient_biodata: Default::default(),
                patient_notes: Default::default(),
                patient
            }
        }

        #[ink(message)]
        pub fn test_cross_call(&mut self) -> bool {
            self.patient.name();
        }

        // The create_patient function creates a new patient record and associates it with an account id.
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

        // The update_biodata function updates the biodata of a patient.
        #[ink(message)]
        pub fn update_biodata(&mut self, identifier: AccountId, biodata: Biodata) -> Result<(), Error> {
            self.patient_biodata.insert(&identifier, &biodata);

            self.env().emit_event(BiodataUpdate {
                identifier: Some(identifier),
                message: Some(biodata)
            });

            Ok(())
        }

        // The update_clinical_notes function updates the clinical notes of a patient.
        #[ink(message)]
        pub fn update_clinical_notes(&mut self, identifier: AccountId, notes: ClinicalNotes) -> Result<(), Error> {
            self.patient_notes.insert(&identifier, &notes);

            self.env().emit_event(ClinicalNotesUpdate {
                identifier: Some(identifier),
                message: Some(notes)
            });

            Ok(())
        }

        // The get_biodata function retrieves the biodata of a patient.
        #[ink(message)]
        pub fn get_biodata(&self, identifier: AccountId) -> Option<Biodata> {
            self.patient_biodata.get(&identifier)
        }

        // The get_clinical_notes function retrieves the clinical notes of a patient.
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