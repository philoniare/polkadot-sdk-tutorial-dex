use super::*;
use frame_support::pallet_prelude::*;
use sp_runtime::traits::{
    CheckedAdd, CheckedDiv, CheckedMul, CheckedSub, SaturatedConversion, Zero,
};
use sp_runtime::Permill;

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, MaxEncodedLen, TypeInfo)]
#[scale_info(skip_type_params(T))]
pub struct LiquidityPool<T: Config> {
    pub assets: (AssetIdOf<T>, AssetIdOf<T>),
    pub reserves: (AssetBalanceOf<T>, AssetBalanceOf<T>),
    pub total_liquidity: AssetBalanceOf<T>,
    pub liquidity_token: AssetIdOf<T>,
}

impl<T: Config> LiquidityPool<T> {
    // Function to mint liquidity tokens and update reserves
    pub fn mint(
        &mut self,
        amounts: (AssetBalanceOf<T>, AssetBalanceOf<T>),
        liquidity_minted: AssetBalanceOf<T>,
    ) -> DispatchResult {
        self.reserves.0 = self
            .reserves
            .0
            .checked_add(&amounts.0)
            .ok_or(Error::<T>::ReserveOverflow)?;
        self.reserves.1 = self
            .reserves
            .1
            .checked_add(&amounts.1)
            .ok_or(Error::<T>::ReserveOverflow)?;
        self.total_liquidity = self
            .total_liquidity
            .checked_add(&liquidity_minted)
            .ok_or(Error::<T>::LiquidityOverflow)?;
        Ok(())
    }

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

        // Define the swap fee as a Permill value (0.3%)
        let swap_fee = Permill::from_perthousand(3);

        // Calculate the input amount after deducting the swap fee
        let amount_in_after_fee = amount_in
            .checked_sub(&swap_fee.mul_floor(amount_in))
            .ok_or(Error::<T>::ArithmeticOverflow)?;

        // Calculate the numerator of the output amount formula
        let numerator = amount_in_after_fee
            .checked_mul(&reserve_out)
            .ok_or(Error::<T>::ArithmeticOverflow)?;

        // Calculate the denominator of the output amount formula
        let denominator = reserve_in
            .checked_add(&amount_in_after_fee)
            .ok_or(Error::<T>::ArithmeticOverflow)?;

        // Perform integer division to obtain the final output amount
        let amount_out = numerator
            .checked_div(&denominator)
            .ok_or(Error::<T>::DivisionByZero)?;

        // Return the calculated output amount
        Ok(amount_out)
    }
}
