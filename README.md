# Extrinsic Calls

Now, that we have the necessary methods implemented in our `liquidity_pool`. Let's define pallet calls that will call
these methods.

### What are pallet calls?

Pallet calls are the entry points for interacting with a pallet's functionality from the outside world. They define the
public API of the pallet and allow users or other pallets to trigger specific actions or updates within the pallet's
logic.

In Substrate, pallet calls are defined using the `#[pallet::call]` attribute macro. This macro is used to define a Rust
trait that represents the dispatchable functions of the pallet.

Necessary Functionalities of Pallet Calls:

#### 1. Authorization and Validation:

- Pallet calls should perform necessary authorization checks to ensure that the caller has the required permissions to
  execute the function.
- The origin parameter represents the source of the call (e.g., a user account, another pallet) and can be used for
  authentication and authorization purposes.
- The pallet should validate the input parameters to ensure they meet the expected constraints and requirements.

#### 2. State Modification:

- Pallet calls often involve modifying the pallet's storage items or interacting with other pallets' storage.
- The function should perform the necessary storage updates, such as inserting, updating, or removing data from storage
  maps or values.
- It's crucial to handle storage interactions safely and consistently, considering edge cases and potential conflicts.

#### 3. Event Emission:

- Pallet calls can emit events to notify users or other pallets about important actions or state changes.
- Events provide transparency and allow external entities to react to specific occurrences within the pallet.
- The pallet should define an event type using the `#[pallet::event]` attribute and emit relevant events using
  the `Self::deposit_event()` function.

#### 4. Error Handling:

- Pallet calls should handle potential errors that may occur during execution.
- The pallet should define a comprehensive set of error variants using the `#[pallet::error]` attribute.
- Functions should return a `DispatchResult` type, which is either `Ok(())` for success or `Err(Error::<T>::SomeError)`
  for specific error cases.

#### 5. Weight Annotation:

- Each pallet call should have a weight annotation that specifies the expected execution cost of the function.
- The weight represents the computational resources required to execute the call and is used by the Substrate runtime to
  manage and prioritize transactions.
- The `#[pallet::weight()]` attribute is used to assign a weight to a pallet call.

#### 6. Documentation and Comments:

- Pallet calls should be well-documented with clear explanations of their purpose, parameters, and expected behavior.
- Use comments and doc comments (`///`) to provide detailed information about each call, including any prerequisites,
  side effects, or important considerations.

#### 7. Testing:

- Pallet calls should be thoroughly tested to ensure their correctness and robustness.
- Write unit tests to cover different scenarios, including success cases, error cases, and edge cases.
- Use the `#[test]` attribute to define test functions and the `assert_ok!()`, `assert_noop!()`, and `assert_err!()`
  macros to make assertions about the expected behavior.