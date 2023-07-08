#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod healthDot {
    use ink::storage::Mapping;

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

        #[ink(message)]
        pub fn owner_of(&self, id: TokenId) -> Option<AccountId> {
            self.token_owner.get(id)
        }
    }
}