# Substrate Storage

Substrate provides a powerful and flexible storage system that allows you to store and retrieve data on the blockchain.

### The most commonly used storage types are:
- **StorageValue**: Used to store a single value of a specific type.
- **StorageMap**: Allows key-value pair storage, where each key is associated with a corresponding value.
- **StorageDoubleMap**: Enables storing values with two keys, providing a two-dimensional mapping.

These storage types are defined within the pallet using the `#[pallet::storage]` attribute and can be accessed using
the generated API.

Now, let's explore some naive storage primitives that can be used to represent liquidity pools and their associated data.