# Multiple Liquidity Pools with Fixed Trading Pairs
To store multiple liquidity pools with fixed trading pairs, we can use a `StorageMap`:
```rust
#[pallet::storage]
pub type TradingPairBalances<T: Config> = StorageMap<_, Blake2_128Concat, (AssetId, AssetId), (Balance, Balance), ValueQuery>;
```
Here, we define a `StorageMap` called `TradingPairBalances` that maps a tuple of `(AssetId, AssetId)` representing a trading pair to a tuple of `(Balance, Balance)` representing the balances of each asset in the liquidity pool. The `Blake2_128Concat` hasher is used for key hashing.

Drawbacks:
- Fixed trading pairs limit the flexibility of adding new trading pairs dynamically.
- Requires manual management of trading pair mappings.