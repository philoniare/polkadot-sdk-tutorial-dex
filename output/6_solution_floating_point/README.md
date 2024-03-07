# Solution to Floating Point Arithmetic

Here's how we can simplify the same method:
```rust
use frame_support::pallet_prelude::*;

fn get_amount_out(amount_in: Balance, reserve_in: Balance, reserve_out: Balance) -> Result<Balance, DispatchError> {
    // Ensure that both reserve balances are non-zero
    ensure!(!reserve_in.is_zero() && !reserve_out.is_zero(), Error::<T>::InsufficientLiquidity);

    // Define the swap fee as a PerThousand value
    let swap_fee = PerThousand::from_parts(3);

    // Calculate the input amount after deducting the swap fee
    let amount_in_after_fee = amount_in.checked_sub(swap_fee.mul_floor(amount_in)).ok_or(Error::<T>::Overflow)?;

    // Calculate the numerator of the output amount formula
    let numerator = amount_in_after_fee.checked_mul(reserve_out).ok_or(Error::<T>::Overflow)?;

    // Calculate the denominator of the output amount formula
    let denominator = reserve_in.checked_add(amount_in_after_fee).ok_or(Error::<T>::Overflow)?;

    // Perform integer division to obtain the final output amount
    let amount_out = numerator.checked_div(denominator).ok_or(Error::<T>::DivisionByZero)?;

    Ok(amount_out)
}
```
We use `PerThousand::from_parts(3)` to define the swap fee as a `PerThousand` value. This represents a fee of 0.3%.
To calculate the input amount after deducting the swap fee, we use `swap_fee.mul_floor(amount_in)`. The `mul_floor`
method multiplies `amount_in` by the swap fee and rounds down the result to the nearest integer. We then subtract this fee
from the original `amount_in` to get the amount after the fee deduction.
