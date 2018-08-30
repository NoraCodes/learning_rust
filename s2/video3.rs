// In this video, we'll talk about creating projects with Cargo.
// ST
// Open up a shell and let's get started. On Mac OS, this will be terminal.app;
// on Windows, cmd.exe. On Linux, use the terminal of your choice.
// 
// cargo version
// You can see that it tells you the version, the commit, and the date.
//
// Now we'll make a new project. We'll call it guess
// cargo new guess
// Now you should see that inside that directory there is
//  Cargo.toml
//  and a directory called src
//  
//  Src contains our source code, and Cargo.toml contains info about the crate
// If we look at src/main.rs you can see it contains a simple hello world program
// We can run it with
// Cargo run
// note that cargo gives us some information about what it's doing
// now let's look at cargo.toml
//
// It's specified in TOML format, and it contains metadata like the package's name,
// authors, version, etc
// Let's get started with this project. Open up main.rs

use std::io;

fn main() {
    println!("Welcome to Guess a Number!");

    println!("Input guess:");

    // A string to hold our guess
    let mut guess = String::new();
    // Read into the buffer we created
    io::stdin().read_line(&mut guess);
    // This will cause an unused must use error
    // .expect("Unable to read from standard input.");
    println!("You guessed: {}", guess);
}

// Now we can run this but it's not very exciting.
// In the next video we'll see how to actually get random numbers
// using an external crate.