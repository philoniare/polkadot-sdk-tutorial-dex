# FRAME DEX Tutorial

Welcome to the FRAME DEX tutorial! This tutorial project is designed to introduce developers to building an DEX pallet using the FRAME framework. The DEX pallet enables the creation of liquidity pools, token swapping, and liquidity provision functionality on a Substrate-based blockchain.


## Topics Covered
1. **Pallet Structure**: Learn how to set up the basic structure of a Substrate pallet, including the pallet configuration trait, storage items, and dispatchable functions.
2. **Liquidity Pools**: Understand the concept of liquidity pools and how they facilitate token swapping. Implement functions to create and manage liquidity pools.
3. **Asset Management**: Explore asset management in Substrate using the `pallet_assets` or `pallet_balances`. Learn how to define and manage custom assets within the DEX pallet.
4. **Liquidity Provision**: Implement functionality for users to provide liquidity to the DEX by depositing token pairs into liquidity pools. Learn how to calculate and mint liquidity tokens.
5. **Token Swapping**: Develop functions for users to swap tokens through the DEX pallet. Understand the math behind calculating swap amounts and updating pool reserves.
6. **Error Handling**: Handle errors and edge cases gracefully within the pallet. Learn how to define and use custom error types and the DispatchResult type for error propagation.
7. **Events**: Emit events to notify users and off-chain systems about important actions and state changes within the DEX pallet.
8. **Optimization**: Explore ways to optimize the AMM pallet's performance, such as minimizing storage reads/writes, using efficient data structures, and implementing bounds checks.

## Getting Started
To get started with the tutorial, follow the steps at [dotcodeschool](https://dotcodeschool.com/courses/build-your-own-dex/).

## Resources

- Substrate Documentation: https://docs.substrate.io/
- Polkadot Wiki: https://wiki.polkadot.network/
- Rust Programming Language: https://www.rust-lang.org/

Feel free to reach out if you have any questions or need further assistance. Happy learning and building with Substrate!