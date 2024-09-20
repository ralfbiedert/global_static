```rust
// Increase global static counter 2 times
library_a::increase_counter();
library_a::increase_counter();

// Increase global static counter 3 more times
library_b::increase_counter();
library_b::increase_counter();
library_b::increase_counter();

// Print "the" global static counter, as seen by ...
library_a::print_counter();
library_b::print_counter();
```

produces

```
[crates\library_a\src\lib.rs:8:5] &some_crate::GLOBAL_COUNTER = 2
[crates\library_b\src\lib.rs:8:5] &some_crate::GLOBAL_COUNTER = 3
```