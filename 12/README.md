# OOP for Liquidity Pools

Let's create a file called `liquidity_pool.rs` and move the `LiquidityPool` struct that we've defined previously. We
will create our first method to `mint_liquidity`:

```rust
#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, MaxEncodedLen, TypeInfo)]
#[scale_info(skip_type_params(T))]
pub struct LiquidityPool<T: Config> {
    pub assets: (AssetIdOf<T>, AssetIdOf<T>),
    pub reserves: (AssetBalanceOf<T>, AssetBalanceOf<T>),
    pub total_liquidity: AssetBalanceOf<T>,
    pub liquidity_token: AssetIdOf<T>,
    _marker: PhantomData<T>,
}

impl<T: Config> LiquidityPool<T> {
    // Function to mint liquidity tokens and update reserves
    pub fn mint(
        &mut self,
        amounts: (AssetBalanceOf<T>, AssetBalanceOf<T>),
        liquidity_minted: AssetBalanceOf<T>,
    ) {
        self.reserves.0 = self.reserves.0 + amounts.0;
        self.reserves.1 = self.reserves.1 + amounts.1;
        self.total_liquidity = self.total_liquidity + liquidity_minted;
    }
}
```

The mint function is used to mint liquidity tokens and update the reserves of the liquidity pool. It takes the amounts
of tokens being added to the reserves and the amount of liquidity tokens minted. It updates the reserves and total
liquidity accordingly.

The `burn` method works in a similar fashion. It takes in the amounts of tokens being withdrawn from the reserves and
updates the reserves and total liquidity accordingly.
Try to implement the `burn` method.