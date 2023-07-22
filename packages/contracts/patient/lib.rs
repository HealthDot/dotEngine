// Required for environments that don't have a standard library (like a Wasm contract).
#![cfg_attr(not(feature = "std"), no_std, no_main)]

pub use self::patient::{
    Patient,
    PatientRef
};

// We're importing the ink contract language.
#[ink::contract]
mod patient {
    // This trait provides an abstraction for working with storage data structures in ink.
    use ink::storage::Mapping;

    // Importing necessary traits for encoding and decoding.
    use scale::{
        Decode,
        Encode,
    };

    use scale::alloc::string::String;

    // Define our own types for better readability.
    // TokenId represents a unique identifier for each token.
    pub type TokenId = u32;
    // Approved represents the approval status of a token.
    pub type Approved = bool;



    // Annotate the struct as the ink contract's storage.
    // The contract's storage holds its state variables.
    #[ink(storage)]
    #[derive(Default)] // Derive the Default trait to initialize the contract.
    pub struct Patient {
        // The name of the token.
        token_name: String,
        // The symbol of the token.
        token_symbol: String,
        // A mapping from a TokenId to its resource locator (the data it points to).
        token_resource_locator: Mapping<TokenId, String>,
        // A mapping from a TokenId to its owner's AccountId.
        token_owner: Mapping<TokenId, AccountId>,
        // A mapping from a TokenId to an approved AccountId (who can manage this token).
        token_approvals: Mapping<TokenId, AccountId>,
        // A mapping from an AccountId to the count of tokens it owns.
        owned_tokens_count: Mapping<AccountId, u32>
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

    // This is an event that will be emitted when the ownership of any NFT changes.
    #[ink(event)]
    pub struct Transfer {
        // The sender of the transfer (None if it's a new creation).
        #[ink(topic)]
        from: Option<AccountId>,
        // The receiver of the transfer (None if it's destroyed).
        #[ink(topic)]
        to: Option<AccountId>,
        // The id of the token being transferred.
        #[ink(topic)]
        token_id: TokenId
    }

    // This is an event that will be emitted when the approved address for an NFT changes.
    #[ink(event)]
    pub struct Approval {
        // The current owner of the token.
        #[ink(topic)]
        owner: AccountId,
        // The approved address that can manage the token.
        #[ink(topic)]
        spender: AccountId,
        // The id of the token.
        #[ink(topic)]
        token_id: TokenId
    }

    // This is an event that will be emitted when an operator's approved status changes.
    #[ink(event)]
    pub struct ApprovalForAll {
        // The owner of the tokens.
        #[ink(topic)]
        owner: AccountId,
        // The operator whose approved status has changed.
        #[ink(topic)]
        operator: AccountId,
        // Whether the operator is approved or not.
        #[ink(topic)]
        approved: Approved
    }

    // The implementation of the contract.
    impl Patient {
        // Constructor function for the contract. It takes in the token name and symbol.
        #[ink(constructor, payable)]
        pub fn new(token_name: String, token_symbol: String) -> Self {
            Self {
                token_name,
                token_symbol,
                token_resource_locator: Default::default(),
                token_owner: Default::default(),
                token_approvals: Default::default(),
                owned_tokens_count: Default::default()
            }
        }

        /// Returns the balance of the owner.
        ///
        /// This represents the amount of unique tokens the owner has.
        /// The balance is obtained through the balance_of_or_zero function which ensures that it returns zero if there are no tokens.
        /// This function is marked with the #[ink(message)] attribute making it callable from outside the contract.
        #[ink(message)]
        pub fn balance_of(&self, owner: AccountId) -> u32 {
            self.balance_of_or_zero(&owner)
        }

        /// This function returns the owner of a specific token.
        /// NFTs assigned to zero address are considered invalid, and queries about them do throw.
        /// The function returns the owner of the token identified by the provided token ID.
        /// If the token doesn't exist or it's assigned to zero address, the function will return None.
        /// This function is marked with the #[ink(message)] attribute making it callable from outside the contract.
        #[ink(message)]
        pub fn owner_of(&self, token_id: TokenId) -> Option<AccountId> {
            self.token_owner.get(token_id)
        }

        /// This function approves an account to manage a token on behalf of its owner.
        /// The function first approves the address for the token ID and then returns Ok if the operation was successful.
        /// If the operation was unsuccessful, it will return an error.
        /// This function is marked with the #[ink(message)] attribute making it callable from outside the contract.
        #[ink(message)]
        pub fn approve(&mut self, address: AccountId, token_id: TokenId) -> Result<(), Error> {
            self.approve_for(&address, token_id)?;
            Ok(())
        }

        /// This function returns the account approved to manage a specific token.
        /// If there's no account approved for the given token ID, the function will return None.
        /// This function is marked with the #[ink(message)] attribute making it callable from outside the contract.
        #[ink(message)]
        pub fn get_approved(&self, token_id: TokenId) -> Option<AccountId> {
            self.token_approvals.get(token_id)
        }

        /// This function transfers a token from the caller to a recipient.
        /// First, it gets the caller's account ID, then transfers the token with the given ID from the caller to the recipient.
        /// The function will return Ok if the operation was successful, or an error if it wasn't.
        /// This function is marked with the #[ink(message)] attribute making it callable from outside the contract.
        #[ink(message)]
        pub fn transfer(&mut self, to: AccountId, id: TokenId) -> Result<(), Error> {
            let caller = self.env().caller();
            self.transfer_token_from(&caller, &to, id)?;
            Ok(())
        }

        /// This function transfers a token from a sender to a recipient.
        /// It works similarly to the transfer function, but instead of using the caller's account ID, it uses the provided sender's account ID.
        /// This function is marked with the #[ink(message)] attribute making it callable from outside the contract.
        #[ink(message)]
        pub fn transfer_from(&mut self, from: AccountId, to: AccountId, id: TokenId) -> Result<(), Error> {
            self.transfer_token_from(&from, &to, id)?;
            Ok(())
        }

        /// This function mints a new token with a specific ID.
        /// It adds the token to the caller's account and emits a Transfer event indicating the creation of a new token.
        /// The function will return Ok if the operation was successful, or an error if it wasn't.
        /// This function is marked with the #[ink(message)] attribute making it callable from outside the contract.
        #[ink(message)]
        pub fn mint(&mut self, id: TokenId) -> Result<(), Error> {
            let msg_sender: AccountId = self.env().caller();
            
            self.add_token_to(&msg_sender, id)?;
            self.env().emit_event(Transfer {
                from: Some(AccountId::from([0x0; 32])),
                to: Some(msg_sender),
                token_id: id
            });
            Ok(())
        }

        ////////////////////////////////
        ////// Internal Functions///////
        ////////////////////////////////

        
        /// This function checks the number of tokens owned by a specific account.
        /// It attempts to get the balance of an account from the owned_tokens_count map.
        /// If the account does not exist in the map (i.e., it does not own any tokens), it returns 0.
        fn balance_of_or_zero(&self, of: &AccountId) -> u32 {
            self.owned_tokens_count.get(of).unwrap_or(0)
        }

        /// This function adds a token to a specific account.
        /// It first checks if the token with the provided ID already exists, and if it does, it returns an error.
        /// If the account to receive the token is the zero address, it also returns an error.
        /// It then increases the token count of the receiving account and adds the token to the account's ownership.
        /// The function will return Ok if the operation was successful, or an error if it wasn't.
        fn add_token_to(&mut self, to: &AccountId, id: TokenId) -> Result<(), Error> {
            let Self {
                token_owner,
                owned_tokens_count,
                ..
            } = self;

            if token_owner.contains(id) {
                return Err(Error::TokenExists)
            };

            if *to == AccountId::from([0x0; 32]) {
                return Err(Error::NotAllowed)
            }

            let count = owned_tokens_count.get(to).map(|c| c + 1 ).unwrap_or(1);
            
            owned_tokens_count.insert(to, &count);
            token_owner.insert(id, to);

            Ok(())

        }
        
        /// This function transfers a token from one account to another.
        /// It first checks if the token exists, and if it doesn't, it returns an error.
        /// It then removes the token from the sender's account and adds it to the recipient's account.
        /// After transferring the token, it emits a Transfer event.
        /// The function will return Ok if the operation was successful, or an error if it wasn't.
        fn transfer_token_from(&mut self, from: &AccountId, to: &AccountId, id: TokenId) -> Result<(), Error> {
            // let msg_sender: AccountId = self.env().caller();
            
            if !self.exists(id) {
                return Err(Error::TokenNotFound)
            };

            self.remove_token_from(from, id)?;
            self.add_token_to(to, id)?;

            self.env().emit_event(Transfer {
                from: Some(*from),
                to: Some(*to),
                token_id: id
            });

            Ok(())
        }

        /// This function removes a token from a specific account.
        /// It first checks if the token exists, and if it doesn't, it returns an error.
        /// It then decreases the token count of the account and removes the token from the account's ownership.
        /// The function will return Ok if the operation was successful, or an error if it wasn't.
        fn remove_token_from(&mut self, from: &AccountId, id: TokenId) -> Result<(), Error> {
            let Self {
                token_owner,
                owned_tokens_count,
                ..
            } = self;

            if !token_owner.contains(id) {
                return Err(Error::TokenNotFound)
            };

            let count = owned_tokens_count.get(from).map(|c| c - 1).ok_or(Error::CannotFetchValue)?;
            
            owned_tokens_count.insert(from, &count);
            token_owner.remove(id);

            Ok(())
        }

        /// This function checks if a token exists by checking if it has an owner.
        fn exists(&self, id: TokenId) -> bool {
            self.token_owner.contains(id)
        }

        /// This function approves an account to manage a specific token on behalf of its owner.
        /// It first checks if the caller is the owner of the token, and if it's not, it returns an error.
        /// It also checks if the account to be approved is the zero address or if the token is already approved, and if either is true, it returns an error.
        /// If everything is in order, it adds the account to the token's approvals.
        /// After approving the account, it emits an Approval event.
        /// The function will return Ok if the operation was successful, or an error if it wasn't.
        fn approve_for(&mut self, address: &AccountId, token_id: TokenId) -> Result<(), Error> {
            let msg_sender: AccountId = self.env().caller();
            let owner: Option<AccountId> = self.owner_of(token_id);

            if !(owner == Some(msg_sender)) {
                return Err(Error::NotAllowed)
            };

            if *address == AccountId::from([0x0; 32]) {
                return Err(Error::NotAllowed)
            }

            if self.token_approvals.contains(token_id) {
                return Err(Error::NotAllowed)
            } else {
                self.token_approvals.insert(token_id, address);
            }

            self.env().emit_event(Approval {
                owner: msg_sender,
                spender: *address,
                token_id
            });

            Ok(())
        }


        ////////////////////////////////
        ////// Metadata Extension///////
        ////////////////////////////////
        
        /// This function retrieves the name of the token contract.
        /// It clones the token name from the contract's state and returns it.
        #[ink(message)]
        pub fn name(&self) -> String {
            self.token_name.clone()
        }

        /// This function retrieves the symbol of the token contract.
        /// It clones the token symbol from the contract's state and returns it.
        #[ink(message)]
        pub fn symbol(&self) -> String {
            self.token_symbol.clone()
        }

        /// This function retrieves the Uniform Resource Identifier (URI) of a specific token.
        /// The URI is a unique identifier for the token in a given context.
        /// It retrieves the URI from the token_resource_locator map using the provided token ID.
        /// If the token does not exist (i.e., it does not have an URI), it returns None.
        #[ink(message)]
        pub fn token_uri(&self, id: TokenId) -> Option<String> {
            self.token_resource_locator.get(id)
        }

        /// This function sets the Uniform Resource Identifier (URI) for a specific token.
        /// The URI is a unique identifier for the token in a given context.
        /// It inserts the provided URI into the token_resource_locator map with the provided token ID as the key.
        /// The function will return Ok if the operation was successful, or an error if it wasn't.
        #[ink(message)]
        pub fn set_token_uri(&mut self, id: TokenId, uri: String) -> Result<(), Error> {
            let Self {
                token_resource_locator,
                ..
            } = self;

            token_resource_locator.insert(id, &uri);

            Ok(())
        }
    }

    /// Unit tests
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        #[ink::test]
        fn mint_works() {
            let accounts =
                ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            // Create a new contract instance.
            let mut patient = Patient::new(String::from("HealthDot"), String::from("HDOT"));
            // Token 1 does not exists.
            assert_eq!(patient.owner_of(1), None);
            // Alice does not owns tokens.
            assert_eq!(patient.balance_of(accounts.alice), 0);
            // Create token Id 1.
            assert_eq!(patient.mint(1), Ok(()));
            // Alice owns 1 token.
            assert_eq!(patient.balance_of(accounts.alice), 1);
        }

        #[ink::test]
        fn mint_existing_should_fail() {
            let accounts =
                ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            // Create a new contract instance.
            let mut patient = Patient::new(String::from("HealthDot"), String::from("HDOT"));
            // Create token Id 1.
            assert_eq!(patient.mint(1), Ok(()));
            // The first Transfer event takes place
            assert_eq!(1, ink::env::test::recorded_events().count());
            // Alice owns 1 token.
            assert_eq!(patient.balance_of(accounts.alice), 1);
            // Alice owns token Id 1.
            assert_eq!(patient.owner_of(1), Some(accounts.alice));
            // Cannot create  token Id if it exists.
            // Bob cannot own token Id 1.
            assert_eq!(patient.mint(1), Err(Error::TokenExists));
        }

        #[ink::test]
        fn transfer_works() {
            let accounts =
                ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            // Create a new contract instance.
            let mut patient = Patient::new(String::from("HealthDot"), String::from("HDOT"));
            // Create token Id 1 for Alice
            assert_eq!(patient.mint(1), Ok(()));
            // Alice owns token 1
            assert_eq!(patient.balance_of(accounts.alice), 1);
            // Bob does not owns any token
            assert_eq!(patient.balance_of(accounts.bob), 0);
            // The first Transfer event takes place
            assert_eq!(1, ink::env::test::recorded_events().count());
            // Alice transfers token 1 to Bob
            assert_eq!(patient.transfer(accounts.bob, 1), Ok(()));
            // The second Transfer event takes place
            assert_eq!(2, ink::env::test::recorded_events().count());
            // Bob owns token 1
            assert_eq!(patient.balance_of(accounts.bob), 1);
        }

        #[ink::test]
        fn invalid_transfer_should_fail() {
            let accounts =
                ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            // Create a new contract instance.
            let mut patient = Patient::new(String::from("HealthDot"), String::from("HDOT"));
            // Transfer token fails if it does not exists.
            assert_eq!(patient.transfer(accounts.bob, 2), Err(Error::TokenNotFound));
            // Token Id 2 does not exists.
            assert_eq!(patient.owner_of(2), None);
            // Create token Id 2.
            assert_eq!(patient.mint(2), Ok(()));
            // Alice owns 1 token.
            assert_eq!(patient.balance_of(accounts.alice), 1);
            // Token Id 2 is owned by Alice.
            assert_eq!(patient.owner_of(2), Some(accounts.alice));
            // Set Bob as caller
            set_caller(accounts.bob);
        }

        fn set_caller(sender: AccountId) {
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(sender);
        }

    }
}