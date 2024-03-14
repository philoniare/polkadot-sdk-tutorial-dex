# Initialize the pallet project

In this step, we will initialize the pallet project, where we can start building simple logic for the DEX pallet.

1. Clone the template branch by running:
    ```sh
    git clone --branch template git@github.com:shawntabrizi/polkadot-sdk-tutorial-dex.git pallet-dex
    ```
2. Now, that we have starter pallet. Let's break down what each file does:
- `Cargo.toml`:
    - **[dependencies]** section: Lists the dependencies required by the pallet.
        - codec: Specifies the encoding and decoding library used for serialization.
        - scale-info: Provides metadata information for the pallet's types.
        - frame-benchmarking: Allows benchmarking the pallet's performance.
        - frame-support: Provides the core framework for building Substrate pallets.
        - frame-system: Offers system-level functionality and types for the pallet.
    - **[dev-dependencies]** section: Lists the dependencies required only for development and testing purposes.
        - sp-core: Provides core primitives and types used in Substrate development.
        - sp-io: Offers I/O functionality for Substrate pallets.
        - sp-runtime: Provides runtime-related primitives and types.
- `lib.rs`: This file serves as the main entry point for the Substrate pallet. It defines the pallet's structure,
  configuration, storage, events, errors, and dispatchable functions. By organizing the pallet's code in this manner, it
  becomes easier to understand, test, and maintain the pallet's functionality within the Substrate framework. The `lib.rs`
  file in a Substrate pallet heavily relies on the use of macros to organize and simplify the code. Macros are a powerful feature in Rust that allow you to define reusable code templates and generate code based on those templates.
- `mock.rs` used to create a mock runtime environment for testing the functionality of the `pallet_dex` pallet. It sets up a minimal runtime configuration that includes the necessary modules and types required for testing the pallet.
  By creating a mock runtime, developers can write unit tests for the pallet's functionality, ensuring that it behaves as expected and catching potential issues early in the development process. The `new_test_ext` function is typically used in the test cases to initialize the mock runtime's storage before running the tests.
- `tests.rs` contains the unit tests for the `pallet_dex` pallet. It imports the necessary modules and types from the `mock.rs` file and the pallet itself, and defines test functions to verify the pallet's behavior.

## Useful Macro Magic
In the context of a Substrate pallet, macros provided by the FRAME framework are extensively used to define and structure various aspects of the pallet. Here are the primary macro usages that you'll see often:
1. Pallet Declaration:
    - The `#[frame_support::pallet]` macro is used to declare the pallet module, indicating that it represents a FRAME pallet.
    - This macro helps in organizing the pallet's code and provides a standard structure for defining the pallet's components.
2. Configuration Trait:
    - The `#[pallet::config]` macro is used to define the pallet's configuration trait, specifying the required types and constants for the pallet.
    - This macro ensures that the pallet's configuration is structured correctly and can be easily implemented by the runtime.
3. Storage Items:
    - The `#[pallet::storage]` macro is used to define the pallet's storage items, such as storage values and storage maps.
    - This macro simplifies the process of declaring storage items and generates the necessary code for accessing and modifying them.
4. Events:
    - The `#[pallet::event]` macro is used to define the events that the pallet can emit.
    - It provides a clean and organized way to declare event types and their associated data.
    - The `#[pallet::generate_deposit]` macro automatically generates the deposit_event function for emitting events, reducing boilerplate code.
5. Errors:
    - The `#[pallet::error]` macro is used to define the errors that the pallet can return.
    - It helps in organizing the error variants and their associated data in a structured manner.
6. Dispatchable Functions:
    - The `#[pallet::call]` macro is used to define the pallet's dispatchable functions (or Calls).
    - It provides a clear separation between the pallet's public interface and its internal implementation.
    - The macro also handles the necessary boilerplate code for dispatching and executing the functions.

By leveraging these macros, the Substrate pallet's code becomes more organized, readable, and maintainable. The macros abstract away many of the low-level details and provide a high-level, declarative way to define the pallet's components.

Furthermore, the use of macros helps in enforcing a consistent structure across different pallets. This consistency makes it easier for developers to understand and work with multiple pallets within a Substrate runtime.

Now that we've familiarized ourselves with the starter pallet code, let's dive into specifying the generic types necessary for our project.