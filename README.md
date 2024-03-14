# Solution to Generic Types in Substrate

Here's the solution:
```rust
/// Type to access the Assets Pallet.
type Fungibles: fungibles::Inspect<Self::AccountId, AssetId = u32>
   + fungibles::Mutate<Self::AccountId>
   + fungibles::Create<Self::AccountId>;
```

You can check out the reference [docs](https://paritytech.github.io/polkadot-sdk/master/frame_support/traits/tokens/fungibles/index.html) in order to find which traits you'll need to implement based on your specific
use-case. In our case, we want to inspect, mutate and create a new asset, which will be used to represent a liquidity
pool token.