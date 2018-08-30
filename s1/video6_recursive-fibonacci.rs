// In this video, we'll implement a simple recursive Fibonacci calculator,
// which we'll improve in the next video.
// ST
// As a quick refresher, to compute a Fibonacci number, we compute the last two
// Fibonacci numbers and add them, starting with 1 and 1.
// ST
// Hop into the playground an let's get coding!

// First things first, we want to define our first and second fibonacci
// numbers. We'll use a feature called constants.
// Consts are bindings defined outside of any function, whose type you must specify,
// and which can't be mutable.

// We want to use u64 because it's large and we don't care about negative numbers.
const FIB_ZERO: u64 = 1;
const FIB_ONE: u64 = 1;

// Now we'll define a simple recursive Fibonacci function.
// It takes a u64 for the Fibonacci number to generate and returns a u64.
fn fib(n: u64) -> u64 {
    // Our base cases are 0 and 1.
    if n == 0 {
        // Here we don't have to write return.
        // That's because Rust is expression oriented: this expression
        // returns up to the if, which in turn returns up to the function,
        // and since there's no semicolon, that causes the function to return
        // this value.
        FIB_ZERO
    } else if n == 1 {
        FIB_ONE
    } else {
        // Finally, our actual computation.
        fib(n - 1) + fib(n - 2)
    }
}


// Now the main function 
fn main() {
    // We'll generate a bunch of fibonacci numbers
    // For a for loop over numbers, we'll use range syntax
    for i in 1..41 {
        println!("{}:{}", i, fib(i))
    }
}

// Running this code, you'll see the computation performed exactly as expected.
// Congratulations!
// However, you might notice that the later computations took a while.

//ST
// Unfortunately, because it is purely recursive, large values of n require very many
// computations. While Rust code is fast, writing a poor algorithm like this one can still make
// programs slow, so in the next video we'll build a better fibonacci algorithm using a 
// dynamic programming technique.