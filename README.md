# Safe Math

In Substrate runtime pallet development, using safe math is crucial to ensure the stability and security of your
blockchain network. When dealing with arithmetic operations, especially those involving user input or storage values,
it's important to handle potential overflow or underflow situations gracefully.

### Why do we need to use Safe Math?

If a panic occurs within your runtime pallet due to an arithmetic overflow or underflow, it can lead to severe
consequences:

- **Node Crash**: A panic in the runtime can cause the node to crash, leading to downtime and disrupting the
  availability of your blockchain network.
- **Inconsistent State**: If a panic occurs during a state update, it can leave your blockchain in an inconsistent
  state, corrupting the data and compromising the integrity of your system.
- **Security Vulnerabilities**: Panics caused by arithmetic issues can be exploited by malicious actors to perform
  attacks, such as draining funds or manipulating the system's behavior.

To mitigate these risks, Rust provides safe math utilities that handle arithmetic operations in a safe manner. The two
commonly used options for safe math are:

- `saturating_sub`: This method performs a saturating subtraction. If the result of the subtraction would be negative,
  it returns zero instead of panicking. The benefit of using saturating_sub is that it prevents underflow and ensures
  that the result is always within the valid range. However, it's important to note that it silently returns zero if the
  subtraction would result in a negative value, which may not always be the desired behavior.
- `checked_sub`: This method performs a checked subtraction. It returns an `Option` type, where `Some(result)` is
  returned if the subtraction is successful, and `None` is returned if an overflow or underflow occurs. The benefit of
  using `checked_sub` is that it allows you to explicitly handle the case when the subtraction fails.
  You can choose to return an `Error` with a specific error message or take alternative actions based on your
  requirements. However, it requires additional error handling logic compared to `saturating_sub`.
  The choice between `saturating_sub` and `checked_sub` depends on your specific use case and error handling strategy.
  If you want to ensure that the result is always within the valid range and silently handle underflow by returning
  zero, `saturating_sub` is a good choice. If you want to explicitly handle the case when the subtraction fails and have
  more control over the error handling, `checked_sub` is a better option.