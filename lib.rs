#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod healthDot {
    use ink::storage::Mapping;

    pub type TokenId = u64;
    // type AccountId = <DefaultEnvironment as Environment>::AccountId;

    #[ink(storage)]
    #[derive(Default)]
    pub struct HealthDot {
        // Mapping from token ID to owner address
        token_name: String,
        token_symbol: String,
        token_owner: Mapping<TokenId, AccountId>,
        token_approvals: Mapping<TokenId, AccountId>,
    }

    impl HealthDot {
        #[ink(constructor)]
        pub fn new(name: String, symbol: String) -> Self {
            Default::default()
        }

        #[ink(message)]
        pub fn ownerOf(&self, id: TokenId) -> AccountId {
            self.token_owner.get(id);
        }
    }
}