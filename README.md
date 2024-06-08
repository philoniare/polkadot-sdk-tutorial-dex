# Using structs

In Rust, tuples are a useful way to group multiple values together. However, when dealing with complex data or when readability and maintainability are priorities, creating specific structs is often a better choice. Remember we had the following method:
```
// Helper function to get the consistently ordered trading pair
fn get_trading_pair(
    asset_a: AssetIdOf<T>,
    asset_b: AssetIdOf<T>,
) -> (AssetIdOf<T>, AssetIdOf<T>) {
    if asset_a < asset_b {
        (asset_a, asset_b)
    } else {
        (asset_b, asset_a)
    }
}
```
While this approach works, it has some drawbacks:

1. **Lack of clarity**: Tuples don't provide any semantic meaning to the values they contain. It's not immediately clear what the first and second elements of the tuple represent without looking at the function signature or documentation.
2. **Prone to errors**: When using tuples, it's easy to accidentally swap the order of the elements or misinterpret their meaning, leading to bugs in the code.
3. **Limited functionality**: Tuples don't allow you to define associated functions or methods, which can make the code less expressive and harder to maintain.

Try refactoring this code to create a specific struct for representing the trading pair