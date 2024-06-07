# 3. Dynamic Liquidity Pools with Arbitrary Trading Pairs:
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
