# Solution: Substrate Storage

Here's the solution to the previous assignment:

```rust
#[pallet::storage]
pub type LiquidityTokens<T: Config> = StorageMap<_, Blake2_128Concat, AssetIdOf<T>, (AssetIdOf<T>, AssetIdOf<T>), ValueQuery>;
```
