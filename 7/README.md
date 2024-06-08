# Single Liquidity Pool
To store a single liquidity pool, we can use a `StorageValue`:
```rust
#[pallet::storage]
pub type LiquidityPoolBalance<T: Config> = StorageValue<_, Balance, ValueQuery>;
```
In this example, we define a `StorageValue` named `LiquidityPoolBalance` that stores a single `Balance` value. With database interactions, storage operation return is always an option. There is either `Some` value at a key or there is `None` value at a specific key. The default behavior for accessing storage is `OptionQuery`, which exposes exactly that information back. However, in practice, there are some scenarios where you would like `None` to actually represent some value. The common example is a `Balance` where `None` is exactly the same as having a zero balance. In this case, you would rather have the storage return a `0` rather than an `Option` each time you query and use the balance. So for convenience, `ValueQuery` will return a default value for a type rather than having you deal with an optional value. The default value for a `None` can be modified by using the `DefaultValue`.

Drawbacks:
- Limited to storing only one liquidity pool
- Cannot handle multiple trading pairs or liquidity pools