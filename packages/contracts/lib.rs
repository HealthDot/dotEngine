// Enable the contract to run without the standard library if the "std" feature is not set.
#![cfg_attr(not(feature = "std"), no_std, no_main)]

// This attribute specifies that the following module is an ink! smart contract.
#[ink::contract]
mod epr {
    // Use necessary items from the ink crate.
    use patient::PatientRef;

    use ink::storage::Mapping;
    use ink::prelude::string::String;
    use ink::prelude::vec::Vec;

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

    // Define an Error enum to handle errors.
    #[derive(Debug, Copy, Clone, PartialEq, Eq, scale::Decode, scale::Encode)]
    #[cfg_attr(
        feature = "std",
        derive(ink::storage::traits::StorageLayout, scale_info::TypeInfo)
    )]
    pub enum Error {
        NotAllowed,
        CannotFetchValue,
        PermissionDenied
    }

    /// The initial state is `Adder`.
    #[derive(Debug, Copy, Clone, PartialEq, Eq, scale::Decode, scale::Encode)]
    #[cfg_attr(
        feature = "std",
        derive(ink::storage::traits::StorageLayout, scale_info::TypeInfo)
    )]
    pub enum Which {
        Patient
    }

    // The EPR (Electronic Patient Record) struct represents the smart contract.
    #[ink(storage)]
    pub struct Epr {
        // The current_id field keeps track of the current patient id.
        current_id: HealthId,
        // The record_count mapping stores the account id associated with each health id.
        record_count: Mapping<HealthId, AccountId>,
        // The patient_biodata mapping stores the biodata of each patient.
        patient_biodata: Mapping<AccountId, Biodata>,  
        // The patient_notes mapping stores the clinical notes of each patient.
        patient_notes: Mapping<AccountId, ClinicalNotes>,
        // which: Which,
        patient: PatientRef
        // permissions: Mapping<AccountId, Permission>
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

    // Define the behavior of the EPR contract.
    impl Epr {
        // The constructor initializes an EPR contract with no data.
        #[ink(constructor)]
        pub fn new(patient_code_hash: Hash, version: u32) -> Self {
            let salt = version.to_le_bytes();
            let patient_contract = PatientRef::new(String::from("HealthDOT"), String::from("HDOT"))
                .endowment(0)
                .code_hash(patient_code_hash)
                .salt_bytes(salt)
                .instantiate();
            Self {
                current_id: 0,
                record_count: Default::default(),
                patient_biodata: Default::default(),
                patient_notes: Default::default(),
                // which: Which::Patient,
                patient: patient_contract
            }
        }

        // #[ink(message)]
        // pub fn get(&self) -> String {
        //     self.patient.name()
        // }

        // The create_patient function creates a new patient record and associates it with an account id.
        #[ink(message)]
        pub fn create_patient(&mut self, identifier: AccountId) -> Result<(), Error> {
            
            
            let count = self.current_id + 1;
            self.current_id = count;
            self.record_count.insert(&count, &identifier);

            // self.patient.mint(count);
        
            // self.env().emit_event(NewPatient {
            //     id: count,
            //     identifier: Some(identifier)
            // });

            Ok(())
        }
        

    }

}