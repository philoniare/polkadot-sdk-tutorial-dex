# Floating Point Arithmetic

Now that we have the `mint` and `burn` methods implemented, let's add the main functionality
to swap tokens in our DEX. DEX exchanges, such as Uniswap V2.0 charged a 0.3% fee per swap, so we'll want to implement
it as well.

### How are token balances stored in blockchain?
In blockchain systems, currency and token balances are typically stored as integer values rather than floating-point numbers. This is because floating-point arithmetic can introduce rounding errors and lead to precision issues, which can be problematic when dealing with financial transactions. Integer values, on the other hand, provide exact precision and help maintain the integrity of the balances.

However, when calculating exchange rates or applying fees, it's common to use fixed-point arithmetic or decimal-like calculations. In the case of the swap function, a 0.3% fee is being applied to the amount of tokens being swapped. To calculate this fee while avoiding floating-point arithmetic, the function multiplies the input amount by 997 (representing 99.7%) and performs integer division later to obtain the final amount after the fee deduction.

Here's how we would implement it:
```rust
pub fn swap(&mut self, asset_in: AssetId, amount_in: Balance, asset_out: AssetId, min_amount_out: Balance) -> Result<Balance, DispatchError> {
    // Ensure that the input asset is part of the trading pair
    ensure!(self.assets.0 == asset_in || self.assets.1 == asset_in, Error::<T>::InvalidAssetIn);
    // Ensure that the output asset is part of the trading pair
    ensure!(self.assets.0 == asset_out || self.assets.1 == asset_out, Error::<T>::InvalidAssetOut);

    // Determine the reserve balances based on the input asset
    let (reserve_in, reserve_out) = if self.assets.0 == asset_in {
        (self.reserves.0, self.reserves.1)
    } else {
        (self.reserves.1, self.reserves.0)
    };

    // Calculate the amount of tokens to receive after the swap
    let amount_out = Self::get_amount_out(amount_in, reserve_in, reserve_out)?;
    // Ensure that the calculated amount is greater than or equal to the minimum amount specified
    ensure!(amount_out >= min_amount_out, Error::<T>::InsufficientAmountOut);

    // Update the reserve balances based on the input and output assets
    if self.assets.0 == asset_in {
        // Add the input amount to the input asset reserve
        self.reserves.0 = self.reserves.0.checked_add(amount_in).ok_or(Error::<T>::ReserveOverflow)?;
        // Subtract the output amount from the output asset reserve
        self.reserves.1 = self.reserves.1.checked_sub(amount_out).ok_or(Error::<T>::InsufficientReserves)?;
    } else {
        // Subtract the output amount from the output asset reserve
        self.reserves.0 = self.reserves.0.checked_sub(amount_out).ok_or(Error::<T>::InsufficientReserves)?;
        // Add the input amount to the input asset reserve
        self.reserves.1 = self.reserves.1.checked_add(amount_in).ok_or(Error::<T>::ReserveOverflow)?;
    }

    // Return the calculated output amount
    Ok(amount_out)
}
```
And to actually calculate the fee amount, we'll create a separate method:
```rust
// Helper function to calculate the amount of tokens to receive in a swap
fn get_amount_out(amount_in: Balance, reserve_in: Balance, reserve_out: Balance) -> Result<Balance, DispatchError> {
    // Ensure that both reserve balances are non-zero
    ensure!(!reserve_in.is_zero() && !reserve_out.is_zero(), Error::<T>::InsufficientLiquidity);

    // Calculate the input amount with the swap fee (0.3%) by multiplying by 997 (99.7%)
    let amount_in_with_fee = amount_in.checked_mul(997).ok_or(Error::<T>::Overflow)?;
    // Calculate the numerator of the output amount formula
    let numerator = amount_in_with_fee.checked_mul(reserve_out).ok_or(Error::<T>::Overflow)?;
    // Calculate the denominator of the output amount formula
    let denominator = reserve_in.checked_mul(1000).ok_or(Error::<T>::Overflow)?.checked_add(amount_in_with_fee).ok_or(Error::<T>::Overflow)?;

    // Perform integer division to obtain the final output amount
    let amount_out = numerator.checked_div(denominator).ok_or(Error::<T>::DivisionByZero)?;

    // Return the calculated output amount
    Ok(amount_out)
}
```
Substrate's `frame_support` module provides a set of types and methods for performing fixed-point arithmetic operations.
These types, such as `PerBill`, `PerThousand`, and `PerMillion`, allow you to represent fractions and perform
calculations with high precision. In this guide, we'll explore these types and their related methods.

- `PerThing` Trait:
  - The `PerThing` trait is the foundation for the fixed-point types in Substrate.
  - It defines a set of constants and methods for working with fractions.
  - The trait is implemented by types like `PerBill`, `PerThousand`, and `PerMillion`.
- `PerBill` Type:
  - `PerBill` represents fractions with a precision of one billionth (10^-9).
  - It is useful for representing very small fractions or percentages.
  - The range of possible values for `PerBill` is from 0 to 1 billion (inclusive).
- `PerThousand` Type:
  - `PerThousand` represents fractions with a precision of one thousandth (10^-3).
  - It is commonly used for representing percentages with one decimal place.
  - The range of possible values for `PerThousand` is from 0 to 1,000 (inclusive).
- `PerMillion` Type:
  - `PerMillion` represents fractions with a precision of one millionth (10^-6).
  - It offers a balance between precision and range for representing percentages.
  - The range of possible values for `PerMillion` is from 0 to 1 million (inclusive).
- Creating Instances:
  - You can create instances of `PerBill`, `PerThousand`, or `PerMillion` using the `from_parts` method.
  - For example: `let fraction = PerThousand::from_parts(250);` creates a `PerThousand` instance representing 25%.
- Arithmetic Operations:
  - The fixed-point types provide methods for performing arithmetic operations.
    - `mul_floor(value)`: Multiplies the fraction by value and rounds down the result.
    - `mul_ceil(value)`: Multiplies the fraction by value and rounds up the result.
    - `div_floor(value)`: Divides value by the fraction and rounds down the result.
    - `div_ceil(value)`: Divides value by the fraction and rounds up the result.
- Comparison Methods:
  - The fixed-point types implement comparison methods for checking equality and ordering.
    - `is_zero()`: Checks if the fraction is equal to zero.
    - `is_one()`: Checks if the fraction is equal to one.
    - `deconstruct()`: Returns the raw value of the fraction as an integer.
- Conversion Methods:
  - The fixed-point types provide methods for converting between different precisions.
    - `from_rational(numerator, denominator)`: Creates a fraction from a rational number.
    - `from_rational_approximation(numerator, denominator)`: Creates an approximation of a rational number.
    - `from_float(float)`: Creates a fraction from a floating-point number.
These types and methods in Substrate's `frame_support` module provide a convenient way to work with fractions and perform precise arithmetic operations. They are particularly useful when dealing with percentages, fees, or any scenario that requires fixed-point calculations.

By leveraging the power of `PerBill`, `PerThousand`, and related methods, you can write more expressive and precise code when working with fractions and percentages in your Substrate runtime.

Use one of the methods to refactor the `get_amount_out` method. 


