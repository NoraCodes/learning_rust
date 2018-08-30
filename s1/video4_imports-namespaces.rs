// In this video, we'll discuss how Rust code is organized, see some more types,
// and build some more substantial programs.
// ST
// Rust code is organized into crates and modules. Crates are built up of modules
// and one crate can rely on other crates.
// ST
// Pull up the Playground, and let's jump in!

// We use the use keyword to import things into the namespace. In this program,
// we'll import something from the standard library.
use std::collections::LinkedList;
// Here, std is the standard library crate, collections is a module, and LinkedList is the item
// we're importing.

// Now we can use LinkedList in our main function.
fn main() {
    // We'll create a new, mutable linked list.
    let mut ll = LinkedList::new();
    // Now we can add items to the linked list.
    ll.push_back(1);
    ll.push_back(2);
    ll.push_back(4);

    // Finally, let's print all the items in the list.
    // Rust's for loop uses the concept of an iterator, so we don't have to
    // write out all the conditionals.
    for item in ll {
        println!("{}", item);
    }
}

// There's actually a number of imports that come into every program, called
// the Prelude. We can use these without importing anything explicitly.
// For example, we have access to Vectors, heap-allocated dynamic arrays,
// in every program.

fn main() {
    let mut v = Vec::new();
    v.push('x');
    v.push('y');
    v.push('z');

    for item in v {
        println!("{}", item);
    }
}

// Let's try making our own module.
// First, we can define a function:

fn say_hello() {
    println!("Hello, world!");
}

// Now we can wrap that in a mod block

mod hello {
    fn say_hello() {
        println!("Hello, world!");
    }
}

// Finally, we can import this and use it:

use hello::say_hello;

fn main() {
    say_hello();
}


// ST
// Now that you know how to import items into your code, move on to the next
// video to learn about all the functionality that comes with Rust by default.