# Solution to Object Oriented Design

Here's a naive implementation of the `burn` method:

```rust
// Function to burn liquidity tokens and update reserves
pub fn burn(
    &mut self,
    liquidity_burned: AssetBalanceOf<T>,
    amounts_out: (AssetBalanceOf<T>, AssetBalanceOf<T>),
) {
    self.reserves.0 = self.reserves.0 - amounts_out.0;
    self.reserves.1 = self.reserves.1 - amounts_out.1;
    self.total_liquidity = self.total_liquidity - liquidity_burned;
}
```