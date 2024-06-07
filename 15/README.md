# Using Safe Math

Now, let's convert the burn function to use safe math:

```rust
// Function to burn liquidity tokens and update reserves
pub fn burn(
    &mut self,
    liquidity_burned: AssetBalanceOf<T>,
    amounts_out: (AssetBalanceOf<T>, AssetBalanceOf<T>),
) -> DispatchResult {
    self.reserves.0 = self
        .reserves
        .0
        .checked_sub(&amounts_out.0)
        .ok_or(Error::<T>::InsufficientReserves)?;
    self.reserves.1 = self
        .reserves
        .1
        .checked_sub(&amounts_out.1)
        .ok_or(Error::<T>::InsufficientReserves)?;
    self.total_liquidity = self
        .total_liquidity
        .checked_sub(&liquidity_burned)
        .ok_or(Error::<T>::InsufficientLiquidity)?;
    Ok(())
}
```

In this updated version:

- We use `checked_sub` for each subtraction operation.
- If the subtraction is successful, `checked_sub` returns `Some(result)`, which we unwrap using `ok_or` and return the
  result.
- If the subtraction fails (i.e., returns `None`), we return an appropriate error using `ok_or`. In this case, we
  return `Error::<T>::InsufficientReserves` for insufficient reserves and `Error::<T>::InsufficientLiquidity` for
  insufficient liquidity.
- The function returns a `DispatchResult`, indicating the success or failure of the operation.

By using safe math, we ensure that the burn function handles potential underflow situations gracefully and returns
appropriate errors when necessary.

Remember to define the corresponding error variants (`InsufficientReserves` and `InsufficientLiquidity`) in your
pallet's `Error` enum like so:

```rust
/// Errors that can be returned by this pallet.
#[pallet::error]
pub enum Error<T> {
    /// Insufficient liquidity available in the pool.
    InsufficientLiquidity,

    /// Insufficient reserves available in the pool for the requested operation.
    InsufficientReserves,
}
```

Using safe math is essential for writing secure and reliable runtime pallets in Substrate. It helps prevent panics,
ensures data consistency, and provides a way to handle arithmetic errors in a controlled manner.

As a challenge, refactor the `mint` method to use safe math.