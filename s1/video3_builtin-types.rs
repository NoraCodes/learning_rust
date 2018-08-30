// In this video, we'll examine the built in types provided by Rust,
// using the Rust Playground, a web-based Rust development environment.
// ST
// We'll install Rust in the next section, but for now, go to the URL you see
// on the screen if you'd like to follow along with coding.
// ST
// Once you've loaded up the Rust Playground, you should see a large white
// text box with controls at the top.
// We'll write a simple hello world app, and run it.

// First, we'll write a simple main() function.
// This function will just print hello world.
// We use the fn keyword to denote a new function. The main function is special,
// in that it runs first in any program.
// The function body is denoted by curly braces.
fn main() {
    // Within the function, we'll simply print hello world
    // We use println followed by an exclaimation point or "bang"
    // to do this.
    // Println prints whatever it is given, followed by a newline.
    println!("Hello, world!")
}

// Upon running this program, you should see that the string Hello, world!
// is printed. Congratulations, you've just run your first Rust program!

// Now, let's do something a little more interesting - some simple arithmetic.
fn main() {
    // First, we'll define two immutable bindings, a and b
    let a = 1;
    let b = 2;
    // Then, we'll use a format string with println! to print them.
    // Each set of curly braces will be replaced with a value.
    println!("{} + {} is {}", a, b, a+b)
    // Println uses a bang in its name because it's a macro. This means
    // that the compiler can figure out the types of the values it's meant to 
    // format into the string at compile time, without being told. This lets
    // us avoid the clumsy syntax of C's printf without the runtime cost of 
    // python's reflection-based formatting.
}

// Upon running this code, you should see "1 + 2 is 3".

// ST
// Talk about type system
// We'll talk about slices and function types in a later video.
// ST
// Talk about bools
// ST
// Talk about chars
// ST
// Talk about ints
// ST
// Talk about floats
// ST
// Talk about tuples and arrays.
// ST
// Now, jump back to the Playground and let's use some of this!

// We'll define our own function, call it "add".


// We use the fn keyword followed by the function name, then the arguments.
// Their types are indicated with colons.
// Finally, the return type is indicated with an arrow.
fn add(a: i32, b: i32) -> i32 {
    // We just return a plus b
    return a + b;
}

// We can use the same main function, but replace the addition with a call to add.

// Now let's try something tricky. Replace the type of b with i64.

fn add(a: i32, b: i64) -> i32 {
    // We'll get an error, so we need to explicitly cast
    return a + b as i32;
    // This is what I mean when I say rust has strong types. You always have to
    // be explicit when changing types, either with an "As" cast or with a function.
}

// ST
// Now you know the building blocks of the Rust type system.
// In the next video, we'll talk about some of the more advanced types
// as we talk about imports and namespaces, and build a more complex
// program.