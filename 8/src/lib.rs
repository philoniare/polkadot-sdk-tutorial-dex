// We make sure this pallet uses `no_std` for compiling to Wasm.
#![cfg_attr(not(feature = "std"), no_std)]

// Re-export pallet items so that they can be accessed from the crate namespace.
use frame_support::pallet_prelude::*;
use frame_support::traits::fungible;
use frame_support::traits::fungibles;
use pallet::*;

// FRAME pallets require their own "mock runtimes" to be able to run unit tests. This module
// contains a mock runtime specific for testing this pallet's functionality.
#[cfg(test)]
mod mock;

// This module contains the unit tests for this pallet.
mod liquidity_pool;
#[cfg(test)]
mod tests;

// Define type aliases for easier access
pub type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
pub type AssetIdOf<T> = <<T as Config>::Fungibles as fungibles::Inspect<
    <T as frame_system::Config>::AccountId,
>>::AssetId;

pub type BalanceOf<T> = <<T as Config>::NativeBalance as fungible::Inspect<
    <T as frame_system::Config>::AccountId,
>>::Balance;

pub type AssetBalanceOf<T> = <<T as Config>::Fungibles as fungibles::Inspect<
    <T as frame_system::Config>::AccountId,
>>::Balance;

// All pallet logic is defined in its own module and must be annotated by the `pallet` attribute.
#[frame_support::pallet]
pub mod pallet {
    // Import various useful types required by all FRAME pallets.
    use super::*;
    use crate::liquidity_pool::LiquidityPool;
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;

    // The `Pallet` struct serves as a placeholder to implement traits, methods and dispatchables
    // (`Call`s) in this pallet.
    #[pallet::pallet]
    pub struct Pallet<T>(_);

    /// The pallet's configuration trait.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// The overarching runtime event type.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        // Type to access the Balances Pallet
        type NativeBalance: fungible::Inspect<Self::AccountId>
            + fungible::Mutate<Self::AccountId>
            + fungible::hold::Inspect<Self::AccountId>
            + fungible::hold::Mutate<Self::AccountId>
            + fungible::freeze::Inspect<Self::AccountId>
            + fungible::freeze::Mutate<Self::AccountId>;

        // Type to access the Assets Pallet
        type Fungibles: fungibles::Inspect<Self::AccountId, AssetId = u32>
            + fungibles::Mutate<Self::AccountId>
            + fungibles::Create<Self::AccountId>;
    }

    /// A storage map for storing liquidity pools
    #[pallet::storage]
    pub type LiquidityPools<T: Config> =
        StorageMap<_, Blake2_128Concat, (AssetIdOf<T>, AssetIdOf<T>), LiquidityPool<T>>;

    /// Storage map for storing mapping of liquidity token to asset pair
    #[pallet::storage]
    pub type LiquidityTokens<T: Config> =
        StorageMap<_, Blake2_128Concat, AssetIdOf<T>, (AssetIdOf<T>, AssetIdOf<T>), ValueQuery>;

    /// Events that functions in this pallet can emit.
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {/* Pallet Event Variants Go Here */}

    /// Errors that can be returned by this pallet.
    #[pallet::error]
    pub enum Error<T> {/* Pallet Error Variants Go Here */}

    /// The pallet's dispatchable functions ([`Call`]s).
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /* User Callable Functions Go Here */
    }

    /// The pallet's internal functions.
    impl<T: Config> Pallet<T> {
        /* Internally Callable Functions Go Here */
    }
}
