# Fibonacci sequence
This mini-project calculates fibonacci succesion. This is a common project when starting with a new language.

The implementation is nothing new. But, it is notable how this implementation  behaves differently when running with and without the --develop flag.

## Overflows

The first notable difference is integer overflow behavior. As stated on the [Rust book](https://doc.rust-lang.org/book/ch03-02-data-types.html#integer-overflow), with cargo run, arithmetic operations exceeding the variable's range trigger a panic, halting program execution. This enforces strict overflow checks, safeguarding against potential errors by catching overflow issues early. On the other hand, when running "cargo run --develop" it instead removes the checks, allows the oveflow, and continues the execution.

It is important then to test without the develop flag.

## Compiler optimizations

The second notable difference is the optimization strategy. When running without develop each calculation of the fibonacci series takes around 500ns. The --develop flag dramatically reduces execution time by leveraging compiler optimizations. The compiler anticipates required calculations and pre-computes the entire Fibonacci sequence at compile time. This eliminates runtime calculations, effectively reducing execution time to zero seconds as pre-calculated values are simply retrieved. 

# Conclusions

This implementation showcases two key differences in program behavior when compiled with and without the --develop flag.  While exploring the code, I stumbled upon this unexpected behavior, highlighting the value of experimentation in development.


## Key takeaways

1. **Test with and Without --develop Flag**: Always test your code in both scenarios. The --develop flag can disable crucial checks, so verifying behavior without it ensures robustness.
2. **Number Type Documentation**: Consult the documentation for the chosen number type. This provides insights into its overflow behavior and helps you anticipate potential issues.
3. **Overflow-Aware Addition**: Consider using addition methods that explicitly detect overflows within the program. This approach offers more control and allows for graceful error handling compared to silent overflow.