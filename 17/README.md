# Floating Point Arithmetic

Now that we have the `mint` and `burn` methods implemented, let's add the main functionality
to swap tokens in our DEX. DEX exchanges, such as Uniswap V2.0 charged a 0.3% fee per swap, so we'll want to implement
it as well.

### How are token balances stored in blockchain?

In blockchain systems, currency and token balances are typically stored as integer values rather than floating-point
numbers. This is because floating-point arithmetic can introduce rounding errors and lead to precision issues, which can
be problematic when dealing with financial transactions. Integer values, on the other hand, provide exact precision and
help maintain the integrity of the balances.

However, when calculating exchange rates or applying fees, it's common to use fixed-point arithmetic or decimal-like
calculations. In the case of the swap function, a 0.3% fee is being applied to the amount of tokens being swapped. To
calculate this fee while avoiding floating-point arithmetic, the function multiplies the input amount by 997 (
representing 99.7%) and performs integer division later to obtain the final amount after the fee deduction.
