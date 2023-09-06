# Data types

### Strings
#### String vs &str
##### String
- Vector of u8 data
- Mutable
- Stored on the heap

##### &str - a string slice
- Vector of u8 data
- Immutable
- Can be stored on the heap, stack or embedded in the compiled code

# Variables
- By default rust variables are immutable.
- Rust can infer variable types from the data that it was initialized with.
- Variable names starting with an underscore will not trigger warnings!

## Casting variables
- Rust does not support implicit casting of variables (as opposed to python).
If you try to divide float by integer the compiler will complain!
- Explicit casting happens using `as` keyword.

## Variable mutability
- Using keywoard `mut` after `let`.
- Why exactly?
- Benefits of immutable variables:
 - allow for concurrency without thread locking
 - the machine code produced by the compiler could be more optimized

## Scope and shadowing
- Variables defined within the code block are only accessible within the code block itself.
- Variables defined outside of a code block are accessible within the code blocke itself.
- Creating a variable with the same name as already created variable is called "shadowing"!
- "Shadowing" variables effectively overrides the value of that variable.
