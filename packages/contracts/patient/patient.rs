#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
pub mod patient {
    use ink::storage::Mapping;

    use scale::{
        Decode,
        Encode,
    };

    pub type TokenId = u32;
    pub type Approved = bool;

    #[ink(storage)]
    #[derive(Default)]
    pub struct Patient {
        // Mapping from token ID to owner address
        token_name: String,
        token_symbol: String,
        token_resource_locator: Mapping<TokenId, String>,
        token_owner: Mapping<TokenId, AccountId>,
        token_approvals: Mapping<TokenId, AccountId>,
        owned_tokens_count: Mapping<AccountId, u32>
    }

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

    /// @dev This emits when ownership of any NFT changes by any mechanism.
    ///  This event emits when NFTs are created (`from` == 0) and destroyed
    ///  (`to` == 0). 
    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: Option<AccountId>,
        #[ink(topic)]
        token_id: TokenId
    }

    /// @dev This emits when the approved address for an NFT is changed or
    ///  reaffirmed. When a Transfer event emits, this also indicates that 
    ///  the approved address for that NFT (if any) is reset to none.
    #[ink(event)]
    pub struct Approval {
        #[ink(topic)]
        owner: AccountId,
        #[ink(topic)]
        spender: AccountId,
        #[ink(topic)]
        token_id: TokenId
    }

    /// @dev This emits when an operator is enabled or disabled for an owner.
    ///  The operator can manage all NFTs of the owner.
    #[ink(event)]
    pub struct ApprovalForAll {
        #[ink(topic)]
        owner: AccountId,
        #[ink(topic)]
        operator: AccountId,
        #[ink(topic)]
        approved: Approved
    }

    impl Patient {
        #[ink(constructor)]
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
        #[ink(message)]
        pub fn balance_of(&self, owner: AccountId) -> u32 {
            self.balance_of_or_zero(&owner)
        }

        

        /// @notice Find the owner of an NFT
        /// @dev NFTs assigned to zero address are considered invalid, and queries
        ///  about them do throw.
        /// @param TokenId The identifier for an NFT
        /// @return The address of the owner of the NFT
        #[ink(message)]
        pub fn owner_of(&self, token_id: TokenId) -> Option<AccountId> {
            self.token_owner.get(token_id)
        }

        #[ink(message)]
        pub fn approve(&mut self, address: AccountId, token_id: TokenId) -> Result<(), Error> {
            self.approve_for(&address, token_id)?;
            Ok(())
        }

        #[ink(message)]
        pub fn get_approved(&self, token_id: TokenId) -> Option<AccountId> {
            self.token_approvals.get(token_id)
        }

        #[ink(message)]
        pub fn transfer(&mut self, to: AccountId, id: TokenId) -> Result<(), Error> {
            let caller = self.env().caller();
            self.transfer_token_from(&caller, &to, id)?;
            Ok(())
        }

        #[ink(message)]
        pub fn transfer_from(&mut self, from: AccountId, to: AccountId, id: TokenId) -> Result<(), Error> {
            self.transfer_token_from(&from, &to, id)?;
            Ok(())
        }

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

        
        fn balance_of_or_zero(&self, of: &AccountId) -> u32 {
            self.owned_tokens_count.get(of).unwrap_or(0)
        }

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
        
        fn transfer_token_from(&mut self, from: &AccountId, to: &AccountId, id: TokenId) -> Result<(), Error> {
            let msg_sender: AccountId = self.env().caller();
            
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

        fn exists(&self, id: TokenId) -> bool {
            self.token_owner.contains(id)
        }

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
        
        #[ink(message)]
        pub fn name(&self) -> String {
            self.token_name.clone()
        }

        #[ink(message)]
        pub fn symbol(&self) -> String {
            self.token_symbol.clone()
        }

        #[ink(message)]
        pub fn token_uri(&self, id: TokenId) -> Option<String> {
            self.token_resource_locator.get(id)
        }

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