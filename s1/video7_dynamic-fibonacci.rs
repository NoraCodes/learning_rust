// In this video, we'll implement a more efficient Fibonacci program.
// We'll do this using dynamic programming. Rather than a recursive call for every
// computation, we'll cache our results and look them up if possible in future
// calls.
// ST
// Hop over to the Playground, and let's get started.

// The best data structure for this is a HashMap, as it allows efficient lookups
// by arbitrary keys.
use std::collections::HashMap;


// We'll use our existing code, for convenience.
const FIB_ZERO: u64 = 1;
const FIB_ONE: u64 = 1;

fn fib(n: u64) -> u64 {
    if n == 0 {
       FIB_ZERO
    } else if n == 1 {
        FIB_ONE
    } else {
        fib(n - 1) + fib(n - 2)
    }
}

// We'll need to add one argument to the Fibonacci calculator.
// This type is a little more complex than you may have seen.
// First, the &mut means it's a mutable reference; more about that later.
// Second, the two types after the HashMap are telling the compiler that we
// want a hash map mapping from u64 to u64.
fn fib_dyn(n: u64, map: &mut HashMap<u64, u64>) -> u64 {
    // The internals of the function are a little more complex too.
    // Because of that, we'll use a match expression instead of if statements.
    // This is similar to a switch statement in C but far, far more powerful.
    match n {
        // The first case is that the value of n is either 0 or 1.
        // In that case we return 1.
        0 | 1 => 1,
        // In all other cases, the match expression will bind the value to
        // the name n, and let us run arbitrary code.
        n => {
            // We need to check if the hashmap contains the relevant key
            // It takes a reference, which we denote with the ampersand operator.
            if map.contains_key(&n) {
                // If the key _is_ there, we return it. The unwrap function here
                // relates to error handling, we'll discuss it later.
                *map.get(&n).unwrap()
            } else {
                // If the key isn't available, we have to compute it recursively.
                let val = fib_dyn(n-1, map) + fib_dyn(n-2, map);
                // Once computed, we put it into the map, as the value associated with the
                // key n
                map.insert(n, val);
                // Finally we return the value
                val
            }
       }
    }
}


// We'll use a similar main function
fn main() {
    // First though, we need to make a hash map
    let mut map = HashMap::new();
    for i in 1..41 {
        println!("{}:{}", i, fib_dyn(i, &mut map))
    }
}

// You should notice that this program runs far faster than the previous one,
// thanks to the dynamic programming approach we used.

// ST
// In the next section, we'll move beyond simple programs and see how the
// Rust toolchain is used to manage projects and dependencies.
