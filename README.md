# Multiple Mutable Borrows in Rust

This example demonstrates a common error in Rust related to mutable borrowing. Rust's borrow checker prevents data races by enforcing strict rules on mutable references.  This example showcases how attempting to have multiple mutable borrows to the same data simultaneously results in a compile-time error.

The `bug.rs` file contains the erroneous code, and `bugSolution.rs` provides a corrected version.

This example highlights a key aspect of Rust's memory safety features.