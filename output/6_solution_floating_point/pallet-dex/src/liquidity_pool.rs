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
    pub fn mint(&mut self, amounts: (Balance, Balance), liquidity_minted: Balance) -> DispatchResult {
        self.reserves.0 = self.reserves.0.checked_add(amounts.0).ok_or(Error::<T>::ReserveOverflow)?;
        self.reserves.1 = self.reserves.1.checked_add(amounts.1).ok_or(Error::<T>::ReserveOverflow)?;
        self.total_liquidity = self.total_liquidity.checked_add(liquidity_minted).ok_or(Error::<T>::LiquidityOverflow)?;
        Ok(())
    }

    // Function to burn liquidity tokens and update reserves
    pub fn burn(&mut self, liquidity_burned: Balance, amounts_out: (Balance, Balance)) -> DispatchResult {
        self.reserves.0 = self.reserves.0.checked_sub(amounts_out.0).ok_or(Error::<T>::InsufficientReserves)?;
        self.reserves.1 = self.reserves.1.checked_sub(amounts_out.1).ok_or(Error::<T>::InsufficientReserves)?;
        self.total_liquidity = self.total_liquidity.checked_sub(liquidity_burned).ok_or(Error::<T>::InsufficientLiquidity)?;
        Ok(())
    }

    pub fn swap(&mut self, asset_in: AssetId, amount_in: Balance, asset_out: AssetId, min_amount_out: Balance) -> Result<Balance, DispatchError> {
        ensure!(self.assets.0 == asset_in || self.assets.1 == asset_in, Error::<T>::InvalidAssetIn);
        ensure!(self.assets.0 == asset_out || self.assets.1 == asset_out, Error::<T>::InvalidAssetOut);

        let (reserve_in, reserve_out) = if self.assets.0 == asset_in {
            (self.reserves.0, self.reserves.1)
        } else {
            (self.reserves.1, self.reserves.0)
        };

        let amount_out = Self::get_amount_out(amount_in, reserve_in, reserve_out)?;
        ensure!(amount_out >= min_amount_out, Error::<T>::InsufficientAmountOut);

        if self.assets.0 == asset_in {
            self.reserves.0 = self.reserves.0.checked_add(amount_in).ok_or(Error::<T>::ReserveOverflow)?;
            self.reserves.1 = self.reserves.1.checked_sub(amount_out).ok_or(Error::<T>::InsufficientReserves)?;
        } else {
            self.reserves.0 = self.reserves.0.checked_sub(amount_out).ok_or(Error::<T>::InsufficientReserves)?;
            self.reserves.1 = self.reserves.1.checked_add(amount_in).ok_or(Error::<T>::ReserveOverflow)?;
        }

        Ok(amount_out)
    }

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
}

