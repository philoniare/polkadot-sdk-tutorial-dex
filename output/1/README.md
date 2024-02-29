# Initialize the pallet project

In this step, we will initialize the pallet project, where we can start building simple logic for the DEX pallet.

1. Create a new Rust project by running:
    ```sh
    cargo new pallet-dex
    ```
2. Change to the `pallet-dex` directory by running:
    ```sh
    cd pallet-dex`
    ```
3. Let's name the `main.rs` to `lib.rs` by running since we're running a library crate:
    ```
    mv src/main.rs src/lib.rs
    ```
4. Let's add the dependencies to the `Cargo.toml` file:
    ```toml
    [package]
    name = "pallet-dex"
    version = "4.0.0-dev"
    description = "DEX FRAME pallet"
    authors = ["Substrate DevHub <https://github.com/substrate-developer-hub>"]
    homepage = "https://substrate.io"
    edition = "2021"
    license = "MIT-0"
    publish = false
    
    [package.metadata.docs.rs]
    targets = ["x86_64-unknown-linux-gnu"]
    
    [dependencies]
    codec = { package = "parity-scale-codec", version = "3.6.1", default-features = false, features = [
        "derive",
    ] }
    scale-info = { version = "2.5.0", default-features = false, features = ["derive"] }
    frame-benchmarking = { version = "4.0.0-dev", default-features = false, optional = true, git = "https://github.com/paritytech/substrate.git", branch = "polkadot-v1.0.0" }
    frame-support = { version = "4.0.0-dev", default-features = false, git = "https://github.com/paritytech/substrate.git", branch = "polkadot-v1.0.0" }
    frame-system = { version = "4.0.0-dev", default-features = false, git = "https://github.com/paritytech/substrate.git", branch = "polkadot-v1.0.0" }
    
    [dev-dependencies]
    sp-core = { version = "21.0.0", git = "https://github.com/paritytech/substrate.git", branch = "polkadot-v1.0.0" }
    sp-io = { version = "23.0.0", git = "https://github.com/paritytech/substrate.git", branch = "polkadot-v1.0.0" }
    sp-runtime = { version = "24.0.0", git = "https://github.com/paritytech/substrate.git", branch = "polkadot-v1.0.0" }
    
    [features]
    default = ["std"]
    std = [
        "codec/std",
        "frame-benchmarking?/std",
        "frame-support/std",
        "frame-system/std",
        "scale-info/std",
    ]
    runtime-benchmarks = ["frame-benchmarking/runtime-benchmarks"]
    try-runtime = ["frame-support/try-runtime"]
    ```
5. Fill out the `lib.rs` with the following pallet code:
   ```rust
   // We make sure this pallet uses `no_std` for compiling to Wasm.
   #![cfg_attr(not(feature = "std"), no_std)]
   
   // Re-export pallet items so that they can be accessed from the crate namespace.
   pub use pallet::*;
   
   // FRAME pallets require their own "mock runtimes" to be able to run unit tests. This module
   // contains a mock runtime specific for testing this pallet's functionality.
   #[cfg(test)]
   mod mock;
   
   // This module contains the unit tests for this pallet.
   #[cfg(test)]
   mod tests;
   
   // All pallet logic is defined in its own module and must be annotated by the `pallet` attribute.
   #[frame_support::pallet]
   pub mod pallet {
       // Import various useful types required by all FRAME pallets.
       use super::*;
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
       }
   
       /// A storage item for this pallet.
       #[pallet::storage]
       pub type SomeItem<T> = StorageValue<_, u32>;
   
       /// A storage map for this pallet.
       #[pallet::storage]
       pub type SomeMap<T> = StorageMap<_, Blake2_128Concat, u32, u32>;
   
       /// Events that functions in this pallet can emit.
       #[pallet::event]
       #[pallet::generate_deposit(pub(super) fn deposit_event)]
       pub enum Event<T: Config> {
           /* Pallet Event Variants Go Here */
       }
   
       /// Errors that can be returned by this pallet.
       #[pallet::error]
       pub enum Error<T> {
           /* Pallet Error Variants Go Here */
       }
   
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
   ```
6. In order to test out the code, we will create a mock runtime that the pallet can run on. Create a `mock.rs` with the
   following code:
   ```rust
   use crate as pallet_dex;
   use frame_support::{
       derive_impl,
       traits::{ConstU16, ConstU64},
   };
   use sp_core::H256;
   use sp_runtime::{
       traits::{BlakeTwo256, IdentityLookup},
       BuildStorage,
   };
   
   type Block = frame_system::mocking::MockBlock<Test>;
   
   // Configure a mock runtime to test the pallet.
   frame_support::construct_runtime!(
       pub enum Test
       {
           System: frame_system,
           Dex: pallet_dex,
       }
   );
   
   #[derive_impl(frame_system::config_preludes::TestDefaultConfig as frame_system::DefaultConfig)]
   impl frame_system::Config for Test {
       type BaseCallFilter = frame_support::traits::Everything;
       type BlockWeights = ();
       type BlockLength = ();
       type DbWeight = ();
       type RuntimeOrigin = RuntimeOrigin;
       type RuntimeCall = RuntimeCall;
       type Nonce = u64;
       type Hash = H256;
       type Hashing = BlakeTwo256;
       type AccountId = u64;
       type Lookup = IdentityLookup<Self::AccountId>;
       type Block = Block;
       type RuntimeEvent = RuntimeEvent;
       type BlockHashCount = ConstU64<250>;
       type Version = ();
       type PalletInfo = PalletInfo;
       type AccountData = ();
       type OnNewAccount = ();
       type OnKilledAccount = ();
       type SystemWeightInfo = ();
       type SS58Prefix = ConstU16<42>;
       type OnSetCode = ();
       type MaxConsumers = frame_support::traits::ConstU32<16>;
   }
   
   impl pallet_dex::Config for Test {
       type RuntimeEvent = RuntimeEvent;
   }
   
   // Build genesis storage according to the mock runtime.
   pub fn new_test_ext() -> sp_io::TestExternalities {
       frame_system::GenesisConfig::<Test>::default().build_storage().unwrap().into()
   }
   ```
7. Finally, let's initialize the `tests.rs` file to write out the code for testing the pallet:
   ```rust
   use crate::{mock::*, Error, Event};
   use frame_support::{assert_noop, assert_ok};
   
   #[test]
   fn basic_test() {
      new_test_ext().execute_with(|| {
         // Go past genesis block so events get deposited
         System::set_block_number(1);
         // Future test conditions would go here.
      });
   }
   ```
8. Try running `cargo test` to make sure that everything works. It should output the following:
   ```sh
   running 2 tests
   test mock::__construct_runtime_integrity_test::runtime_integrity_tests ... ok
   test tests::basic_test ... ok
   
   test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.07s
   
      Doc-tests pallet-template
   
   running 0 tests
   
   test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
   ```