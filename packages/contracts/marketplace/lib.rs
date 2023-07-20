#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
pub mod nft_marketplace {
    use ink_storage::collections::HashMap as StorageHashMap;

    #[ink(storage)]
    pub struct NftMarketplace {
        /// Mapping from token ID to owner address.
        owners: StorageHashMap<u32, AccountId>,
        /// Mapping from token ID to price.
        prices: StorageHashMap<u32, Balance>,
    }

    #[ink(event)]
    pub struct Purchase {
        #[ink(topic)]
        buyer: AccountId,
        #[ink(topic)]
        id: u32,
        #[ink(topic)]
        price: Balance,
    }

    impl NftMarketplace {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                owners: StorageHashMap::new(),
                prices: StorageHashMap::new(),
            }
        }

        #[ink(message)]
        pub fn buy(&mut self, id: u32) -> Result<(), ()> {
            let caller = self.env().caller();
            let price = self.prices.get(&id).ok_or(())?;
            let owner = self.owners.get_mut(&id).ok_or(())?;
            
            self.env().transfer(*owner, *price).map_err(|_| ())?;
            *owner = caller;

            self.env().emit_event(Purchase {
                buyer: caller,
                id,
                price: *price,
            });

            Ok(())
        }

        #[ink(message)]
        pub fn set_price(&mut self, id: u32, price: Balance) {
            let caller = self.env().caller();
            if *self.owners.get(&id).unwrap_or(&caller) == caller {
                self.prices.insert(id, price);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ink_lang as ink;

    #[ink::test]
    fn new_works() {
        let contract = NftMarketplace::new();
        assert_eq!(contract.owners.len(), 0);
        assert_eq!(contract.prices.len(), 0);
    }

    #[ink::test]
    fn buy_works() {
        let mut contract = NftMarketplace::new();
        contract.set_price(1, 10);
        assert_eq!(contract.buy(1), Ok(()));
        assert_eq!(contract.owners.get(&1), Some(&AccountId::from([0x1; 32])));
    }

    #[ink::test]
    fn set_price_works() {
        let mut contract = NftMarketplace::new();
        contract.set_price(1, 10);
        assert_eq!(contract.prices.get(&1), Some(&10));
    }
}
