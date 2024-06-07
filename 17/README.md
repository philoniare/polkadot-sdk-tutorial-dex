# Solution

Here's a simple helper function:

```rust
// Helper function to get the consistently ordered trading pair
fn get_trading_pair(asset_a: AssetId, asset_b: AssetId) -> (AssetId, AssetId) {
    if asset_a < asset_b {
        (asset_a, asset_b)
    } else {
        (asset_b, asset_a)
    }
}
```