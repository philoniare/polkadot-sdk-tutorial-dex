# Using Floating Point Arithmetic

Here's how we would implement it:

```rust
// Function to swap tokens based on pool reserves
pub fn swap(
    &mut self,
    asset_in: AssetIdOf<T>,
    amount_in: AssetBalanceOf<T>,
    asset_out: AssetIdOf<T>,
    min_amount_out: AssetBalanceOf<T>,
) -> Result<AssetBalanceOf<T>, DispatchError> {
    ensure!(
            self.assets.0 == asset_in || self.assets.1 == asset_in,
            Error::<T>::InvalidAssetIn
        );
    ensure!(
            self.assets.0 == asset_out || self.assets.1 == asset_out,
            Error::<T>::InvalidAssetOut
        );

    let (reserve_in, reserve_out) = if self.assets.0 == asset_in {
        (self.reserves.0, self.reserves.1)
    } else {
        (self.reserves.1, self.reserves.0)
    };

    let amount_out = Self::get_amount_out(amount_in, reserve_in, reserve_out)?;
    ensure!(
            amount_out >= min_amount_out,
            Error::<T>::InsufficientAmountOut
        );

    if self.assets.0 == asset_in {
        self.reserves.0 = self
            .reserves
            .0
            .checked_add(&amount_in)
            .ok_or(Error::<T>::ReserveOverflow)?;
        self.reserves.1 = self
            .reserves
            .1
            .checked_sub(&amount_out)
            .ok_or(Error::<T>::InsufficientReserves)?;
    } else {
        self.reserves.0 = self
            .reserves
            .0
            .checked_sub(&amount_out)
            .ok_or(Error::<T>::InsufficientReserves)?;
        self.reserves.1 = self
            .reserves
            .1
            .checked_add(&amount_in)
            .ok_or(Error::<T>::ReserveOverflow)?;
    }

    Ok(amount_out)
}
```

And to actually calculate the fee amount, we'll create a separate method:

```rust
// Helper function to calculate the amount of tokens to receive in a swap
fn get_amount_out(
    amount_in: AssetBalanceOf<T>,
    reserve_in: AssetBalanceOf<T>,
    reserve_out: AssetBalanceOf<T>,
) -> Result<AssetBalanceOf<T>, DispatchError> {
    // Ensure that both reserve balances are non-zero
    ensure!(
            !reserve_in.is_zero() && !reserve_out.is_zero(),
            Error::<T>::InsufficientLiquidity
        );

    // Calculate the input amount with the swap fee (0.3%) by multiplying by 997 (99.7%)
    let amount_in_with_fee = amount_in
        .checked_mul(&AssetBalanceOf::<T>::saturated_from(997u128))
        .ok_or(Error::<T>::ArithmeticOverflow)?;
    // Calculate the numerator of the output amount formula
    let numerator = amount_in_with_fee
        .checked_mul(&reserve_out)
        .ok_or(Error::<T>::ArithmeticOverflow)?;
    // Calculate the denominator of the output amount formula
    let denominator = reserve_in
        .checked_mul(&AssetBalanceOf::<T>::saturated_from(1000u128))
        .ok_or(Error::<T>::ArithmeticOverflow)?
        .checked_add(&amount_in_with_fee)
        .ok_or(Error::<T>::ArithmeticOverflow)?;

    // Perform integer division to obtain the final output amount
    let amount_out = numerator
        .checked_div(&denominator)
        .ok_or(Error::<T>::DivisionByZero)?;

    // Return the calculated output amount
    Ok(amount_out)
}
```