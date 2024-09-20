
fn main() {
    // Increase global static counter 2 times
    library_a::increse_counter();
    library_a::increse_counter();

    // Increase global static counter 3 more times
    library_b::increse_counter();
    library_b::increse_counter();
    library_b::increse_counter();

    // Print "the" global static counter, as seen by ...
    library_a::print_counter();
    library_b::print_counter();
}
