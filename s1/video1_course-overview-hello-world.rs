// Hello and welcome to Learning Rust. I'm Leo Tindall, and I'll be your instructor
// for this course.
// ST
// I'm a software engineer and a longtime Rust user. I've built many applications,
// both for commercial entities and as open source projects, and I maintain several
// open source libraries and applications built with Rust.
// ST
// In this video, we'll run through a quick overview of the course and see a first glimpse of Rust
// code.
// ST
// This course is broken up into 7 sections. In the first section, we'll run
// through the basics of Rust. Then, we'll dive into the Rust ecosystem and
// toolchain, look at the unique features of Rust, examine the type system,
// and finally learn about the functional features of Rust and how they lead
// to idiomatic, safe, and blazing fast code.
// ST
// The requirements for this course are pretty minimal. You need a computer, with
// pretty much any operating system, a text editor (I use VSCode but you can
// use whatever you want) and a basic understanding of how to program, in any
// language.
// ST
// So, I promised you some Rust. Here is your first taste!
// This is a small but complete program which prints Hello, world!, written in
// a modular and extensible way, with a main() function and an additional user-
// defined function that takes a parameter for who to say hello to.
// Believe it or not, this program actually takes advantage of Rust's advanced
// type system in a way I'll explain later.
// ST
// Now that you've gotten a taste of Rust, move on the the next video and I'll
// show you how to use it in your own projects.

fn main() {
    say_hello("world")
}

fn say_hello(name: &str) {
   println!("Hello, {}!", name) 
}