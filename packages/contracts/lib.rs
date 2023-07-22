// Enable the contract to run without the standard library if the "std" feature is not set.
#![cfg_attr(not(feature = "std"), no_std, no_main)]

// This attribute specifies that the following module is an ink! smart contract.
#[ink::contract]
pub mod epr {
    // Use necessary items from the ink crate.
    use patient::PatientRef;

    use ink::storage::Mapping;
    use ink::prelude::string::String;
    use ink::prelude::vec::Vec;

    // Define a type alias for HealthId to enhance readability.
    pub type HealthId = u32;
    // pub type TokenId = u32;

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

    // Access controls
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
    pub struct Permission {
        can_access: bool
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
        which: Which,
        patient: PatientRef,
        permissions: Mapping<AccountId, Permission>
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
        #[ink(constructor, payable)]
        pub fn new(patient_code_hash: Hash) -> Self {
            let patient = PatientRef::new(String::from("HealthDOT"), String::from("HDOT"))
                .endowment(0)
                .code_hash(patient_code_hash)
                .salt_bytes([0xDE, 0xAD, 0xBE, 0xEF])
                .instantiate();

            Self {
                current_id: 0,
                record_count: Default::default(),
                patient_biodata: Default::default(),
                patient_notes: Default::default(),
                which: Which::Patient,
                patient,
                permissions: Default::default()
            }
        }

        // Function to add a user with permissions
        #[ink(message)]
        pub fn add_user_with_permissions(&mut self, user: AccountId, can_access: bool) {
            let new_permission = Permission {
                can_access
            };
            self.permissions.insert(&user, &new_permission);
        }

        #[ink(message)]
        pub fn get(&mut self) -> String {
            self.patient.name()
        }

        // The create_patient function creates a new patient record and associates it with an account id.
        #[ink(message)]
        pub fn create_patient(&mut self, requester: AccountId, identifier: AccountId) -> Result<(), Error> {
            // Check if caller has the required permissions
            let permission = self.permissions.get(&requester).ok_or(Error::PermissionDenied)?;
            if !permission.can_access {
                return Err(Error::PermissionDenied);
            }
            
            let count = self.current_id + 1;
            self.current_id = count;
            self.record_count.insert(&count, &identifier);

            self.patient.mint(count);
        
            // self.env().emit_event(NewPatient {
            //     id: count,
            //     identifier: Some(identifier)
            // });

            Ok(())
        }

        // The update_biodata function updates the biodata of a patient.
        #[ink(message)]
        pub fn update_biodata(&mut self, requester: AccountId, identifier: AccountId, biodata: Biodata) -> Result<(), Error> {
            // Check if caller has the required permissions
            let permission = self.permissions.get(&requester).ok_or(Error::PermissionDenied)?;
            if !permission.can_access {
                return Err(Error::PermissionDenied);
            }
            
            self.patient_biodata.insert(&identifier, &biodata);

            // self.env().emit_event(BiodataUpdate {
            //     identifier: Some(identifier),
            //     message: Some(biodata)
            // });

            Ok(())
        }

        // The update_clinical_notes function updates the clinical notes of a patient.
        #[ink(message)]
        pub fn update_clinical_notes(&mut self, identifier: AccountId, notes: ClinicalNotes) -> Result<(), Error> {
            self.patient_notes.insert(&identifier, &notes);

            // self.env().emit_event(ClinicalNotesUpdate {
            //     identifier: Some(identifier),
            //     message: Some(notes)
            // });

            Ok(())
        }

        // The get_biodata function retrieves the biodata of a patient.
        #[ink(message)]
        pub fn get_biodata(&self, requester: AccountId, identifier: AccountId) -> Option<Biodata> {
            // Check if the requester has permission to access biodata
            if let Some(permission) = self.permissions.get(&requester) {
                if permission.can_access {
                    return self.patient_biodata.get(&identifier);
                }
            }
            // If no permission, return None
            None
            // return self.patient_biodata.get(&identifier); 
        }

        // The get_clinical_notes function retrieves the clinical notes of a patient.
        #[ink(message)]
        pub fn get_clinical_notes(&self, requester: AccountId, identifier: AccountId) -> Option<ClinicalNotes> {
            // Check if the requester has permission to access biodata
            if let Some(permission) = self.permissions.get(&requester) {
                if permission.can_access {
                    return self.patient_notes.get(&identifier)
                }
            }
            // If no permission, return None
            None
            // return self.patient_notes.get(&identifier)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        // #[ink::test]
        // fn new_creates_contract_with_zero_id() {
        //     let patient_code_hash: Hash = Hash::from([0x00; 32]);
        //     let healthdot = Epr::new(patient_code_hash);

        //     assert_eq!(healthdot.current_id, 0);
        // }

        // #[ink::test]
        // fn add_user_with_permissions_works() {
        //     let patient_code_hash: Hash = Hash::from([0x00; 32]);
        //     let mut healthdot = Epr::new(patient_code_hash);
        //     let user: AccountId = AccountId::from([0x0; 32]);

        //     healthdot.add_user_with_permissions(user, true);
            
        //     assert_eq!(healthdot.permissions.get(&user).unwrap().can_access, true);
        // }

        // #[ink::test]
        // fn create_patient_without_permission_fails() {
        //     let patient_code_hash: Hash = Hash::repeat_byte(0x00);
        //     let mut healthdot = HealthDot::new(patient_code_hash);
        //     let requester: AccountId = AccountId::from([0x01; 32]);
        //     let identifier: AccountId = AccountId::from([0x02; 32]);

        //     assert_eq!(
        //         healthdot.create_patient(requester, identifier),
        //         Err(Error::PermissionDenied)
        //     );
        // }

        // #[ink::test]
        // fn create_patient_with_permission_increments_id() {
        //     let patient_code_hash: Hash = Hash::repeat_byte(0x00);
        //     let mut healthdot = HealthDot::new(patient_code_hash);
        //     let requester: AccountId = AccountId::from([0x01; 32]);
        //     let identifier: AccountId = AccountId::from([0x02; 32]);

        //     healthdot.add_user_with_permissions(requester, true);
        //     assert_eq!(healthdot.create_patient(requester, identifier), Ok(()));
        //     assert_eq!(healthdot.current_id, 1);
        //     assert_eq!(healthdot.record_count.get(&1), Some(&identifier));
        // }

    }

}