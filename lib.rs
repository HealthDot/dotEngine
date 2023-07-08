#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod healthDot {
    use ink::storage::Mapping;

    use scale::{
        Decode,
        Encode,
    };

    pub type TokenId = u64;
    pub type Approved = bool;

    #[ink(storage)]
    #[derive(Default)]
    pub struct HealthDot {
        // Mapping from token ID to owner address
        token_name: String,
        token_symbol: String,
        token_owner: Mapping<TokenId, AccountId>,
        token_approvals: Mapping<TokenId, AccountId>,
    }

    #[derive(Encode, Decode, Debug, PartialEq, Eq, Copy, Clone)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        NotOwner,
        NotApproved,
        TokenExists,
        TokenNotFound,
        NotAllowed,
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
        owner: Option<AccountId>,
        #[ink(topic)]
        spender: Option<AccountId>,
        #[ink(topic)]
        token_id: TokenId
    }

    /// @dev This emits when an operator is enabled or disabled for an owner.
    ///  The operator can manage all NFTs of the owner.
    #[ink(event)]
    pub struct ApprovalForAll {
        #[ink(topic)]
        owner: Option<AccountId>,
        #[ink(topic)]
        operator: Option<AccountId>,
        #[ink(topic)]
        approved: Approved
    }

    impl HealthDot {
        #[ink(constructor)]
        pub fn new(token_name: String, token_symbol: String) -> Self {
            Self {
                token_name,
                token_symbol,
                token_owner: Default::default(),
                token_approvals: Default::default(),
            }
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

        ////////////////////////////////
        ////// Internal Functions///////
        ////////////////////////////////
        fn approve_for(&self, address: &AccountId, token_id: TokenId) -> Result<(), Error> {
            let msg_sender: AccountId = self.env().caller();
            let owner: Option<AccountId> = self.owner_of(token_id);

            if !(owner == Some(msg_sender)) {
                return Err(Error::NotAllowed)
            };


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


    }
}