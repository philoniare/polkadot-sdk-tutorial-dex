# Generic Types in Substrate

To implement the liquidity pool functionality, we need to store and manage liquidity pools on-chain. In Substrate, we can utilize the powerful storage capabilities provided by the framework to achieve this.

Let's first define the generic types that we'll be using. In Substrate, generic types are extensively used for various entities such as `AccountId`, `AssetId` and `BalanceOf`.
The primary reason for using generic types is to provide flexibility and customization options for developers when
building their blockchain runtime.

Let's take a closer look at a specific example:
```rust
pub type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
```

This type definition is creating an alias `AccountIdOf<T>` that represents the AccountId type defined in the
`frame_system::Config` trait for a given runtime configuration `T`. In other words, it allows the actual type of
`AccountId` to be determined by the runtime that implements the `frame_system::Config` trait.

The benefits of using generic types in blockchain development, compared to using specific types, are as follows:
1. **Flexibility and Customization**:
    - By using generic types, Substrate allows developers to define their own types for `AccountId`, `AssetId`,
      `BalanceOf`, and other entities based on their specific requirements.
    - Different blockchain projects may have different needs in terms of the size, structure, and representation of
      these types. Generic types provide the flexibility to customize them according to the project's needs.
    - For example, one project might use a 32-byte `AccountId`, while another project might opt for a 64-byte
      `AccountId`. With generic types, each project can define its own AccountId type without modifying the underlying
      Substrate framework.
2. **Interoperability and Composability**:
    - Generic types enable better interoperability and composability between different pallets and runtime configurations.
    - By using generic types, pallets can be designed to work with various runtime configurations without being tightly coupled to specific types.
    - This allows for easier integration and reuse of pallets across different blockchain projects, promoting code modularity and reducing duplication.
3. **Upgradability and Future-proofing**:
    - As blockchain technologies evolve, the requirements for types like `AccountId`, `AssetId`, and `BalanceOf` may change over time.
    - By using generic types, Substrate enables easier upgradability of the runtime without breaking existing code.
    - If a project decides to change the underlying type for AccountId in the future, they can do so by updating the runtime configuration without modifying the pallets that depend on it.
4. **Runtime Optimization**:
    - Generic types allow for runtime optimization based on the specific types used by the runtime.
    - The Substrate compiler can generate optimized code for the specific types defined in the runtime configuration, potentially leading to improved performance and reduced runtime overhead.
    - In contrast, using specific types instead of generics would lead to a more rigid and inflexible system. It would require modifying the pallets and runtime code whenever a change in the underlying types is needed, making upgrades and customization more difficult and error-prone.

By leveraging generic types, Substrate provides a powerful and flexible framework for blockchain development. It allows developers to tailor the types to their specific needs while maintaining interoperability, composability, and upgradability. This flexibility is crucial in the rapidly evolving blockchain ecosystem, enabling projects to adapt and innovate without being constrained by fixed type definitions.

Some common type aliases that are needed to interact with the rest of the blockchain have been provided for you in this section:
```
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
```

If you try to run `cargo build` or `cargo test`, you'll see the following error:
```rust
error[E0433]: failed to resolve: use of undeclared crate or module `fungibles`
  --> src/lib.rs:18:54
   |
18 | pub type AssetIdOf<T> = <<T as Config>::Fungibles as fungibles::Inspect<
   |                                                      ^^^^^^^^^ use of undeclared crate or module `fungibles`
   |
help: consider importing this module
   |
5  + use frame_support::traits::fungibles;
   |
```
Rust is smart enough to point out how we can import those traits. Let's add those imports on top of our `lib.rs` file:
```rust
pub use pallet::*;

// Add these imports
use frame_support::traits::fungible;
use frame_support::traits::fungibles;
```
Next, we'll need to define the `Fungibles` and `NativeBalance` inside our `#[pallet::config]`. This trait specifies the types and associated types that the pallet depends on and expects the runtime to provide. Here's how we can define the `NativeBalance`:
```rust
type NativeBalance: fungible::Inspect<Self::AccountId>
    + fungible::Mutate<Self::AccountId>
    + fungible::hold::Inspect<Self::AccountId>
    + fungible::hold::Mutate<Self::AccountId>
    + fungible::freeze::Inspect<Self::AccountId>
    + fungible::freeze::Mutate<Self::AccountId>;
```
The above means this type provides functionality for inspecting, mutating, holding, and freezing balances associated with accounts (`Self::AccountId`).
It provides a way to interact with the pallet [balances](https://github.com/paritytech/polkadot-sdk/tree/master/substrate/frame/balances).

Now, try to define the `Fungibles` type in a similar way in order to fix the last error:
```rust
error[E0576]: cannot find associated type `Fungibles` in trait `Config`
  --> src/lib.rs:20:41
   |
20 | pub type AssetIdOf<T> = <<T as Config>::Fungibles as fungibles::Inspect<
   |                                         ^^^^^^^^^ not found in `Config`

error[E0576]: cannot find associated type `Fungibles` in trait `Config`
  --> src/lib.rs:28:46
   |
28 | pub type AssetBalanceOf<T> = <<T as Config>::Fungibles as fungibles::Inspect<
   |                                              ^^^^^^^^^ not found in `Config`
```
The `Fungibles` type needs to implement the trains from pallet [assets](https://github.com/paritytech/polkadot-sdk/tree/master/substrate/frame/assets).

