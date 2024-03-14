# Substrate Storage

Substrate provides a powerful and flexible storage system that allows you to store and retrieve data on the blockchain.

### The most commonly used storage types are:
- **StorageValue**: Used to store a single value of a specific type.
- **StorageMap**: Allows key-value pair storage, where each key is associated with a corresponding value.
- **StorageDoubleMap**: Enables storing values with two keys, providing a two-dimensional mapping.

These storage types are defined within the pallet using the `#[pallet::storage]` attribute and can be accessed using
the generated API.

Now, let's explore some naive storage primitives that can be used to represent liquidity pools and their associated data.

### 1. Single Liquidity Pool:
To store a single liquidity pool, we can use a `StorageValue`:
```rust
#[pallet::storage]
pub type LiquidityPoolBalance<T: Config> = StorageValue<_, Balance, ValueQuery>;
```
In this example, we define a `StorageValue` named `LiquidityPoolBalance` that stores a single `Balance` value.
The `ValueQuery` parameter indicates that a getter function will be generated to retrieve the balance.

Drawbacks:
- Limited to storing only one liquidity pool
- Cannot handle multiple trading pairs or liquidity pools

### 2. Multiple Liquidity Pools with Fixed Trading Pairs
To store multiple liquidity pools with fixed trading pairs, we can use a `StorageMap`:
```rust
#[pallet::storage]
pub type TradingPairBalances<T: Config> = StorageMap<_, Blake2_128Concat, (AssetId, AssetId), (Balance, Balance), ValueQuery>;
```
Here, we define a `StorageMap` called `TradingPairBalances` that maps a tuple of `(AssetId, AssetId)` representing a trading pair to a tuple of `(Balance, Balance)` representing the balances of each asset in the liquidity pool. The `Blake2_128Concat` hasher is used for key hashing.

Drawbacks:
- Fixed trading pairs limit the flexibility of adding new trading pairs dynamically.
- Requires manual management of trading pair mappings.

### 3. Dynamic Liquidity Pools with Arbitrary Trading Pairs:
Instead of just storing balance tuples, we can do better to encapsulate the liquidity pool state in a struct called
`LiquidityPool`:
```rust
#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, Default, TypeInfo)]
pub struct LiquidityPool<T: Config> {
    pub assets: (AssetId, AssetId),
    pub reserves: (Balance, Balance),
    pub total_liquidity: Balance,
    pub liquidity_token: AssetId,
    _marker: PhantomData<T>,
}
```
This will help us define the storage like so:
```rust
#[pallet::storage]
pub type LiquidityPools<T: Config> = StorageMap<_, Blake2_128Concat, (AssetIdOf<T>, AssetIdOf<T>), LiquidityPool<T>>;
```

This advanced storage structure provides a more optimized and flexible approach to managing liquidity pools in a Substrate pallet. It allows for efficient access, dynamic management, and extensibility as needed.
With this storage structure, you can efficiently manage liquidity pools and perform operations such as minting, burning, and swapping tokens.

Example usage:
```rust
// Minting liquidity
let trading_pair = (asset_a, asset_b);
let liquidity_pool = LiquidityPools::<T>::get(trading_pair);
// Update liquidity_pool reserves and total_liquidity
LiquidityPools::<T>::insert(trading_pair, liquidity_pool);

// Burning liquidity
let liquidity_token = AssetId::from(...);
let trading_pair = LiquidityTokens::<T>::get(liquidity_token);
let liquidity_pool = LiquidityPools::<T>::get(trading_pair);
// Update liquidity_pool reserves and total_liquidity
LiquidityPools::<T>::insert(trading_pair, liquidity_pool);
```

Now, define a storage called `LiquidityTokens` that allows mapping an `AssetId` of a liquidity token to its associated
trading pair `(AssetId, AssetId)`. This would allow efficient lookup of the trading pair, which is useful for scenarios
such as liquidity token burning or redemption.
