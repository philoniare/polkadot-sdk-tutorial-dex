# Solution to Safe Math

Here's the solution: 
```rust
pub fn mint(&mut self, amounts: (Balance, Balance), liquidity_minted: Balance) -> DispatchResult {
    self.reserves.0 = self.reserves.0.checked_add(amounts.0).ok_or(Error::<T>::ReserveOverflow)?;
    self.reserves.1 = self.reserves.1.checked_add(amounts.1).ok_or(Error::<T>::ReserveOverflow)?;
    self.total_liquidity = self.total_liquidity.checked_add(liquidity_minted).ok_or(Error::<T>::LiquidityOverflow)?;
    Ok(())
}
```