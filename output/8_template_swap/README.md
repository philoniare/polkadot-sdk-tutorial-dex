## Swap Functionality

Finally, we'll implement the most used `swap` functionality. The `swap` function in a decentralized exchange (DEX)
allows users to exchange one asset for another within a liquidity pool. It enables users to trade assets without the
need for a centralized order book or matching engine. Instead, the swap is facilitated by the liquidity pool, which
determines the exchange rate based on the relative reserves of the assets in the pool.

Here's a brief overview of its functionality:
- The user specifies the asset they want to swap (asset in), the amount they want to swap (amount in), the asset they want to receive (asset out), and the minimum amount of the asset out they are willing to accept (min amount out).
- The DEX calculates the amount of asset out that the user will receive based on the current reserves of the assets in the liquidity pool and the specified amount in.
- If the calculated amount out is greater than or equal to the specified min amount out, the swap is executed.
- The DEX updates the reserves of the liquidity pool by adding the amount in of the asset in and subtracting the calculated amount out of the asset out.
- The user receives the calculated amount out of the asset out in exchange for the amount in of the asset in.

Since we already have the `swap` implemented in the liquidity pool, we can refer to it when defining the extrinsic call:
```rust
#[pallet::call_index(3)]
#[pallet::weight(Weight::default())]
pub fn swap(
    origin: OriginFor<T>,
    asset_in: AssetId,
    asset_out: AssetId,
    amount_in: Balance,
    min_amount_out: Balance,
) -> DispatchResult {
    let sender = ensure_signed(origin)?;

    let trading_pair = (asset_a, asset_b);
    ensure!(LiquidityPools::<T>::contains_key(&trading_pair), Error::<T>::LiquidityPoolNotFound);

    let mut liquidity_pool = LiquidityPools::<T>::get(&trading_pair);

    let amount_out = liquidity_pool.swap(asset_in, amount_in, asset_out, min_amount_out)?;

    Self::transfer_asset_from_user(&sender, asset_in, amount_in)?;
    Self::transfer_asset_to_user(&sender, asset_out, amount_out)?;

    LiquidityPools::<T>::insert(&trading_pair, liquidity_pool);

    Self::deposit_event(Event::Swapped(sender, asset_in, amount_in, asset_out, amount_out));

    Ok(())
}
```
As you can see above, we've defined the `transfer_asset_from_user` and `transfer_asset_to_user` methods to help us
transfer the assets:
```rust
fn transfer_asset_from_user(
    user: &T::AccountId,
    asset_id: AssetId,
    amount: Balance,
) -> DispatchResult {
    T::Fungibles::transfer(asset_id, user, &Self::account_id(), amount, false)?;
    Ok(())
}

fn transfer_asset_to_user(
    user: &T::AccountId,
    asset_id: AssetId,
    amount: Balance,
) -> DispatchResult {
    T::Fungibles::transfer(asset_id, &Self::account_id(), user, amount, false)?;
    Ok(())
}
```

With our current implementation, what happens when we have a liquidity pool of `(DOT, USDC)` and a user requests to swap
`(USDC, DOT)`? They would be routed to a pool that doesn't exist, so we'll need to add some logic to make sure that
we route users to the same liquidity pool regardless of the order of assets as long as it's the same pair. See if you
implement it.