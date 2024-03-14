// We make sure this pallet uses `no_std` for compiling to Wasm.
#![cfg_attr(not(feature = "std"), no_std)]

// Re-export pallet items so that they can be accessed from the crate namespace.
pub use pallet::*;
use frame_support::traits::fungible;
use frame_support::traits::fungibles;

// FRAME pallets require their own "mock runtimes" to be able to run unit tests. This module
// contains a mock runtime specific for testing this pallet's functionality.
#[cfg(test)]
mod mock;

// This module contains the unit tests for this pallet.
#[cfg(test)]
mod tests;

mod liquidity_pool;

// Define type aliases for easier access
pub type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
pub type AssetIdOf<T> = <<T as Config>::Fungibles as fungibles::Inspect<
	<T as frame_system::Config>::AccountId,
>>::AssetId;

pub type BalanceOf<T> = <<T as Config>::NativeBalance as fungible::Inspect<
	<T as frame_system::Config>::AccountId,
>>::Balance;

pub type AssetBalanceOf<T> = <<T as Config>::Fungibles as fungibles::Inspect<
	<T as frame_system::Config>::AccountId,
>>::Balance;

// All pallet logic is defined in its own module and must be annotated by the `pallet` attribute.
#[frame_support::pallet]
pub mod pallet {
	// Import various useful types required by all FRAME pallets.
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	// The `Pallet` struct serves as a placeholder to implement traits, methods and dispatchables
	// (`Call`s) in this pallet.
	#[pallet::pallet]
	pub struct Pallet<T>(_);

	/// The pallet's configuration trait.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// The overarching runtime event type.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		// Type to access the Balances Pallet
		type NativeBalance: fungible::Inspect<Self::AccountId>
		+ fungible::Mutate<Self::AccountId>
		+ fungible::hold::Inspect<Self::AccountId>
		+ fungible::hold::Mutate<Self::AccountId>
		+ fungible::freeze::Inspect<Self::AccountId>
		+ fungible::freeze::Mutate<Self::AccountId>;

		// Type to access the Assets Pallet
		type Fungibles: fungibles::Inspect<Self::AccountId, AssetId = u32>
		+ fungibles::Mutate<Self::AccountId>
		+ fungibles::Create<Self::AccountId>;
	}

	/// A storage map for storing liquidity pools
	#[pallet::storage]
	pub type LiquidityPools<T: Config> =
	StorageMap<_, Blake2_128Concat, (AssetId, AssetId), LiquidityPool<T>, ValueQuery>;

	/// Storage map for storing mapping of liquidity token to asset pair
	#[pallet::storage]
	pub type LiquidityTokens<T: Config> =
	StorageMap<_, Blake2_128Concat, AssetId, (AssetId, AssetId), ValueQuery>;

	/// Events that functions in this pallet can emit.
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Liquidity pool created.
		/// Parameters:
		/// - `T::AccountId`: The account ID of the liquidity provider who created the pool.
		/// - `(T::AssetId, T::AssetId)`: The trading pair of the created liquidity pool.
		LiquidityPoolCreated(T::AccountId, (T::AssetId, T::AssetId)),
	}

	/// Errors that can be returned by this pallet.
	#[pallet::error]
	pub enum Error<T> {
		/// Insufficient liquidity available in the pool.
		InsufficientLiquidity,

		/// Insufficient reserves available in the pool for the requested operation.
		InsufficientReserves,

		/// Overflow occurred when adding to the reserve balance.
		ReserveOverflow,

		/// Overflow occurred when adding to the total liquidity.
		LiquidityOverflow,

		/// The asset being swapped in is not part of the specified trading pair.
		InvalidAssetIn,

		/// The asset being swapped out is not part of the specified trading pair.
		InvalidAssetOut,

		/// The liquidity pool for the specified trading pair already exists.
		LiquidityPoolAlreadyExists,
	}

	/// The pallet's dispatchable functions ([`Call`]s).
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		// Dispatchable call to create a new liquidity pool
		#[pallet::call_index(0)]
		#[pallet::weight(Weight::default())]
		pub fn create_liquidity_pool(
			origin: OriginFor<T>,
			asset_a: AssetId,
			asset_b: AssetId,
			liquidity_token: AssetId,
		) -> DispatchResult {
			// ensure that the origin has been signed
			let sender = ensure_signed(origin)?;

			let trading_pair = (asset_a, asset_b);
			ensure!(!LiquidityPools::<T>::contains_key(trading_pair), Error::<T>::LiquidityPoolAlreadyExists);

			// Create a new liquidity pool
			let liquidity_pool = LiquidityPool {
				assets: trading_pair,
				reserves: (Zero::zero(), Zero::zero()),
				total_liquidity: Zero::zero(),
				liquidity_token,
				_marker: PhantomData,
			};

			// Insert the new liquidity pool into the storage
			LiquidityPools::<T>::insert(trading_pair, liquidity_pool);

			// Log an event indicating that the pool was created
			Self::deposit_event(Event::LiquidityPoolCreated(sender, trading_pair));

			Ok(())
		}

		#[pallet::call_index(1)]
		#[pallet::weight(Weight::default())]
		pub fn mint_liquidity(
			origin: OriginFor<T>,
			asset_a: AssetId,
			asset_b: AssetId,
			amount_a: Balance,
			amount_b: Balance,
			min_liquidity: Balance,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			let trading_pair = (asset_a, asset_b);

			// Check if the liquidity pool exists for the given trading pair
			ensure!(LiquidityPools::<T>::contains_key(&trading_pair), Error::<T>::LiquidityPoolNotFound);

			// Get the liquidity pool from storage
			let mut liquidity_pool = LiquidityPools::<T>::get(&trading_pair);

			// Calculate the liquidity minted based on the provided amounts and the current reserves
			let liquidity_minted = Self::calculate_liquidity_minted(
				(amount_a, amount_b),
				(liquidity_pool.reserves.0, liquidity_pool.reserves.1),
				liquidity_pool.total_liquidity,
			)?;

			// Ensure that the liquidity minted is greater than or equal to the minimum liquidity specified
			ensure!(liquidity_minted >= min_liquidity, Error::<T>::InsufficientLiquidityMinted);

			// Transfer the assets from the sender to the liquidity pool
			Self::transfer_asset_to_pool(&sender, trading_pair.0, amount_a)?;
			Self::transfer_asset_to_pool(&sender, trading_pair.1, amount_b)?;

			// Mint liquidity tokens to the sender
			Self::mint_liquidity_tokens(&sender, liquidity_pool.liquidity_token, liquidity_minted)?;

			// Update the liquidity pool reserves and total liquidity using the `mint` method
			liquidity_pool.mint((amount_a, amount_b), liquidity_minted)?;

			// Update the liquidity pool in storage
			LiquidityPools::<T>::insert(&trading_pair, liquidity_pool);

			// Emit the LiquidityMinted event
			Self::deposit_event(Event::LiquidityMinted(sender, trading_pair, liquidity_minted));

			Ok(())
		}

		// Dispatchable call to burn liquidity tokens
		#[pallet::call_index(2)]
		#[pallet::weight(Weight::default())]
		pub fn burn_liquidity(
			origin: OriginFor<T>,
			asset_a: AssetId,
			asset_b: AssetId,
			liquidity_burned: Balance,
			min_amount_a: Balance,
			min_amount_b: Balance,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			let trading_pair = (asset_a, asset_b);
			ensure!(LiquidityPools::<T>::contains_key(trading_pair), Error::<T>::InvalidTradingPair);

			let mut liquidity_pool = LiquidityPools::<T>::get(trading_pair);

			// Calculate the amounts of tokens to withdraw based on the liquidity burned and the current reserves
			let amounts_out = Self::calculate_amounts_out(
				liquidity_burned,
				(liquidity_pool.reserves.0, liquidity_pool.reserves.1),
				liquidity_pool.total_liquidity,
			)?;
			ensure!(amounts_out.0 >= min_amount_a && amounts_out.1 >= min_amount_b, Error::<T>::InsufficientAmountsOut);

			// Burn the liquidity tokens from the sender
			Self::burn_liquidity_tokens(&sender, trading_pair, liquidity_burned)?;

			// Update the liquidity pool reserves and total liquidity
			liquidity_pool.burn(liquidity_burned, amounts_out);
			LiquidityPools::<T>::insert(trading_pair, liquidity_pool);

			Self::deposit_event(Event::LiquidityBurned(sender, trading_pair, liquidity_burned));

			Ok(())
		}
	}


	/// The pallet's internal functions.
	impl<T: Config> Pallet<T> {
		/* Internally Callable Functions Go Here */
		fn calculate_liquidity_minted(
			amounts: (Balance, Balance),
			reserves: (Balance, Balance),
			total_liquidity: Balance,
		) -> Result<Balance, DispatchError> {
			let (amount_a, amount_b) = amounts;
			let (reserve_a, reserve_b) = reserves;

			ensure!(!amount_a.is_zero() && !amount_b.is_zero(), Error::<T>::InsufficientLiquidityMinted);

			if total_liquidity.is_zero() {
				// If the liquidity pool is empty, the minted liquidity is the geometric mean of the amounts
				let liquidity_minted = Self::geometric_mean(amount_a, amount_b)?;
				Ok(liquidity_minted)
			} else {
				// If the liquidity pool is not empty, calculate the minted liquidity proportionally
				let liquidity_minted_a = amount_a.checked_mul(total_liquidity).ok_or(Error::<T>::Overflow)?
					.checked_div(reserve_a).ok_or(Error::<T>::DivisionByZero)?;

				let liquidity_minted_b = amount_b.checked_mul(total_liquidity).ok_or(Error::<T>::Overflow)?
					.checked_div(reserve_b).ok_or(Error::<T>::DivisionByZero)?;

				// Choose the smaller minted liquidity to maintain the desired asset ratio
				let liquidity_minted = sp_std::cmp::min(liquidity_minted_a, liquidity_minted_b);
				Ok(liquidity_minted)
			}
		}

		fn geometric_mean(amount_a: Balance, amount_b: Balance) -> Result<Balance, DispatchError> {
			let sqrt_product = (amount_a.checked_mul(amount_b).ok_or(Error::<T>::Overflow)?)
				.integer_sqrt();
			Ok(sqrt_product)
		}

		fn calculate_amounts_out(
			liquidity_burned: Balance,
			reserves: (Balance, Balance),
			total_liquidity: Balance,
		) -> Result<(Balance, Balance), DispatchError> {
			ensure!(!liquidity_burned.is_zero(), Error::<T>::ZeroLiquidityBurned);
			ensure!(!total_liquidity.is_zero(), Error::<T>::InsufficientLiquidity);

			let (reserve_a, reserve_b) = reserves;
			ensure!(!reserve_a.is_zero() && !reserve_b.is_zero(), Error::<T>::InsufficientLiquidity);

			let amount_a = liquidity_burned
				.checked_mul(reserve_a)
				.ok_or(Error::<T>::Overflow)?
				.checked_div(total_liquidity)
				.ok_or(Error::<T>::DivisionByZero)?;

			let amount_b = liquidity_burned
				.checked_mul(reserve_b)
				.ok_or(Error::<T>::Overflow)?
				.checked_div(total_liquidity)
				.ok_or(Error::<T>::DivisionByZero)?;

			Ok((amount_a, amount_b))
		}
	}
}
