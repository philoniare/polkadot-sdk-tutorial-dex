# Extrinsic Calls

Now, that we have the necessary methods implemented in our `liquidity_pool`. Let's define pallet calls that will call
these methods. 

### What are pallet calls?

Pallet calls are the entry points for interacting with a pallet's functionality from the outside world. They define the public API of the pallet and allow users or other pallets to trigger specific actions or updates within the pallet's logic.

In Substrate, pallet calls are defined using the `#[pallet::call]` attribute macro. This macro is used to define a Rust trait that represents the dispatchable functions of the pallet.

Necessary Functionalities of Pallet Calls:

#### 1. Authorization and Validation:
   - Pallet calls should perform necessary authorization checks to ensure that the caller has the required permissions to execute the function.
   - The origin parameter represents the source of the call (e.g., a user account, another pallet) and can be used for authentication and authorization purposes.
   - The pallet should validate the input parameters to ensure they meet the expected constraints and requirements.
#### 2. State Modification:
   - Pallet calls often involve modifying the pallet's storage items or interacting with other pallets' storage.
   - The function should perform the necessary storage updates, such as inserting, updating, or removing data from storage maps or values.
   - It's crucial to handle storage interactions safely and consistently, considering edge cases and potential conflicts.
#### 3. Event Emission:
   - Pallet calls can emit events to notify users or other pallets about important actions or state changes.
   - Events provide transparency and allow external entities to react to specific occurrences within the pallet.
   - The pallet should define an event type using the `#[pallet::event]` attribute and emit relevant events using the `Self::deposit_event()` function.
#### 4. Error Handling:
   - Pallet calls should handle potential errors that may occur during execution.
   - The pallet should define a comprehensive set of error variants using the `#[pallet::error]` attribute.
   - Functions should return a `DispatchResult` type, which is either `Ok(())` for success or `Err(Error::<T>::SomeError)` for specific error cases.
#### 5. Weight Annotation:
   - Each pallet call should have a weight annotation that specifies the expected execution cost of the function.
   - The weight represents the computational resources required to execute the call and is used by the Substrate runtime to manage and prioritize transactions.
   - The `#[pallet::weight()]` attribute is used to assign a weight to a pallet call.
#### 6. Documentation and Comments:
   - Pallet calls should be well-documented with clear explanations of their purpose, parameters, and expected behavior.
   - Use comments and doc comments (`///`) to provide detailed information about each call, including any prerequisites, side effects, or important considerations.
#### 7. Testing:
   - Pallet calls should be thoroughly tested to ensure their correctness and robustness.
   - Write unit tests to cover different scenarios, including success cases, error cases, and edge cases.
   - Use the `#[test]` attribute to define test functions and the `assert_ok!()`, `assert_noop!()`, and `assert_err!()` macros to make assertions about the expected behavior.

By following these guidelines and implementing the necessary functionalities, you can create well-structured and reliable pallet calls that provide a clear and secure interface for interacting with your pallet's functionality.

Let's get started by implementing the call to create a liquidity pool:
```rust
#[pallet::call]
impl<T: Config> Pallet<T> {
    // Dispatchable call to create a new liquidity pool
    #[pallet::weight(10_000)]
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
}
```

Make sure to update your pallet's `Error` enum to include the `LiquidityPoolAlreadyExists` variant, and add the `LiquidityPoolCreated` variant to your Event enum.
In a similar fashion, we could implement the call to `mint_liquidity`. We still have some parts that we have not
discussed yet, but don't worry, we'll be implementing those in later modules.
Here's how we can implement the `mint_liqudity` call:
```rust
#[pallet::weight(10_000)]
pub fn mint_liquidity(
    origin: OriginFor<T>,
    asset_a: AssetId,
    asset_b: AssetId,
    amount_a: Balance,
    amount_b: Balance,
    min_liquidity: Balance,
) -> DispatchResult {
    let sender = ensure_signed(origin)?;

    // Ensure consistent ordering of assets in the trading pair
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
```

We've abstracted away some functionality to helper methods. For example, the `calculate_liquidity_minted` function,
which calculates the amount of liquidity tokens to be minted based on the provided asset amounts and the current state
of the liquidity pool. When total liquidity is non-zero, `liquidity_minted` is calculated as
`(amount * total_liquidity) / reserve` for each asset. If there's no existing liquidity it calculates the geometric mean:
```rust
impl<T: Config> Pallet<T> {
    // ...

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

    // ...
}
```

Can you implement the call for burning the liquidity `burn_liquidity` next?
