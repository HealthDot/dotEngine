#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
pub mod epr {
    use ink::storage::Mapping;
    use ink::env;

    #[ink(storage)]
    pub struct EHR {
        health_records: Mapping<u32, String>,
        patient_token: AccountId,  // The address of the patient_token contract
    }

    impl EHR {
        #[ink(constructor)]
        pub fn new(patient_token: AccountId) -> Self {
            Self {
                health_records: Default::default(),
                patient_token,
            }
        }

        // create_patient should set patient identifier

        #[ink(message)]
        pub fn create_patient(&mut self, token_id: u32, health_record: String) {
            // Create an instance of the patient_token contract
            let mut patient_token_instance = ink_env::call::FromAccountId::from_account_id(self.patient_token);

            // Mint the token
            patient_token_instance.mint(token_id);

            // Store the health record
            self.health_records.insert(token_id, health_record);
        }

        #[ink(message)]
        pub fn update_health_record(&mut self, token_id: u32, health_record: String) {
            self.health_records.insert(token_id, health_record);
        }

        #[ink(message)]
        pub fn get_health_record(&self, token_id: u32) -> Option<String> {
            self.health_records.get(&token_id)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use ink_lang as ink;

    }
}