# FRAME support for Floating Point Arithmetic

Substrate's `frame_support` module provides a set of types and methods for performing fixed-point arithmetic operations.
These types, such as `PerBill`, `PerThousand`, and `PerMillion`, allow you to represent fractions and perform
calculations with high precision. In this guide, we'll explore these types and their related methods.

- `PerThing` Trait:
    - The `PerThing` trait is the foundation for the fixed-point types in Substrate.
    - It defines a set of constants and methods for working with fractions.
    - The trait is implemented by types like `PerBill`, `PerThousand`, and `PerMillion`.
- `PerBill` Type:
    - `PerBill` represents fractions with a precision of one billionth (10^-9).
    - It is useful for representing very small fractions or percentages.
    - The range of possible values for `PerBill` is from 0 to 1 billion (inclusive).
- `PerThousand` Type:
    - `PerThousand` represents fractions with a precision of one thousandth (10^-3).
    - It is commonly used for representing percentages with one decimal place.
    - The range of possible values for `PerThousand` is from 0 to 1,000 (inclusive).
- `PerMillion` Type:
    - `PerMillion` represents fractions with a precision of one millionth (10^-6).
    - It offers a balance between precision and range for representing percentages.
    - The range of possible values for `PerMillion` is from 0 to 1 million (inclusive).
- Creating Instances:
    - You can create instances of `PerBill`, `PerThousand`, or `PerMillion` using the `from_parts` method.
    - For example: `let fraction = PerThousand::from_parts(250);` creates a `PerThousand` instance representing 25%.
- Arithmetic Operations:
    - The fixed-point types provide methods for performing arithmetic operations.
        - `mul_floor(value)`: Multiplies the fraction by value and rounds down the result.
        - `mul_ceil(value)`: Multiplies the fraction by value and rounds up the result.
        - `div_floor(value)`: Divides value by the fraction and rounds down the result.
        - `div_ceil(value)`: Divides value by the fraction and rounds up the result.
- Comparison Methods:
    - The fixed-point types implement comparison methods for checking equality and ordering.
        - `is_zero()`: Checks if the fraction is equal to zero.
        - `is_one()`: Checks if the fraction is equal to one.
        - `deconstruct()`: Returns the raw value of the fraction as an integer.
- Conversion Methods:
    - The fixed-point types provide methods for converting between different precisions.
        - `from_rational(numerator, denominator)`: Creates a fraction from a rational number.
        - `from_rational_approximation(numerator, denominator)`: Creates an approximation of a rational number.
        - `from_float(float)`: Creates a fraction from a floating-point number.
          These types and methods in Substrate's `frame_support` module provide a convenient way to work with fractions
          and perform precise arithmetic operations. They are particularly useful when dealing with percentages, fees,
          or any scenario that requires fixed-point calculations.

By leveraging the power of `PerBill`, `PerThousand`, and related methods, you can write more expressive and precise code
when working with fractions and percentages in your Substrate runtime.

Use one of the methods to refactor the `get_amount_out` method.