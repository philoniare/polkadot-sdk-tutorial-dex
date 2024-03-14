use crate::pallet::Config;
use crate::{AssetBalanceOf, AssetIdOf};
use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::dispatch::TypeInfo;
use frame_support::RuntimeDebug;
use std::marker::PhantomData;

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
}
