# Call to create liquidity pool

## Let's get started by implementing the call to create a liquidity pool:

```rust
#[pallet::call_index(0)]
#[pallet::weight(Weight::default())]
pub fn create_liquidity_pool(
    origin: OriginFor<T>,
    asset_a: AssetIdOf<T>,
    asset_b: AssetIdOf<T>,
    liquidity_token: AssetIdOf<T>,
) -> DispatchResult {
    // ensure that the origin has been signed
    let sender = ensure_signed(origin)?;

    let trading_pair = (asset_a, asset_b);
    ensure!(
                !LiquidityPools::<T>::contains_key(trading_pair),
                Error::<T>::LiquidityPoolAlreadyExists
            );

    // Create a new liquidity pool
    let liquidity_pool = LiquidityPool {
        assets: trading_pair,
        reserves: (Zero::zero(), Zero::zero()),
        total_liquidity: Zero::zero(),
        liquidity_token,
    };

    // Insert the new liquidity pool into the storage
    LiquidityPools::<T>::insert(trading_pair, liquidity_pool);

    // Log an event indicating that the pool was created
    Self::deposit_event(Event::LiquidityPoolCreated(sender, trading_pair));

    Ok(())
}
```

Make sure to update your pallet's `Error` enum to include the `LiquidityPoolAlreadyExists` variant, and add
the `LiquidityPoolCreated` variant to your Event enum.
