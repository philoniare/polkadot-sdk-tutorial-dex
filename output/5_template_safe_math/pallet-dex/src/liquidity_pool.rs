#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, Default, TypeInfo)]
pub struct LiquidityPool<T: Config> {
    pub assets: (AssetId, AssetId),
    pub reserves: (Balance, Balance),
    pub total_liquidity: Balance,
    pub liquidity_token: AssetId,
    _marker: PhantomData<T>,
}

impl<T: Config> LiquidityPool<T> {
    // Function to mint liquidity tokens and update reserves
    pub fn mint(&mut self, amounts: (Balance, Balance), liquidity_minted: Balance) {
        self.reserves.0 = self.reserves.0 + amounts.0;
        self.reserves.1 = self.reserves.1 + amounts.1;
        self.total_liquidity = self.total_liquidity + liquidity_minted;
    }

    // Function to burn liquidity tokens and update reserves
    pub fn burn(&mut self, liquidity_burned: Balance, amounts_out: (Balance, Balance)) -> DispatchResult {
        self.reserves.0 = self.reserves.0.checked_sub(amounts_out.0).ok_or(Error::<T>::InsufficientReserves)?;
        self.reserves.1 = self.reserves.1.checked_sub(amounts_out.1).ok_or(Error::<T>::InsufficientReserves)?;
        self.total_liquidity = self.total_liquidity.checked_sub(liquidity_burned).ok_or(Error::<T>::InsufficientLiquidity)?;
        Ok(())
    }
}

