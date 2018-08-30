// In this section, we'll discuss the basics of Rust and what makes it such
// a powerful language.
// ST
// In this video, we'll examine the basic syntax of Rust and how its variables
// (called bindings) work.
// ST
// Rust is similar to C or JavaScript in syntax. Like most programming languages, 
// it has variables. Rust calls them bindings.
// You declare a binding with the let keyword. For instance, the code on the
// left is creating a binding to the string literal value, called var.
// ST
// Unlike many programming languages, Rust bindings are not mutable by default.
// This means that the code here will not compile.
// ST
// Fortunately, the Rust compiler gives us excellent error messages. For instance,
// the error message produced by the code on the previous slide shows exactly where
// the erroneous code is. These error messages often provide info on how to fix the
// offending code as well.
// ST
// Unlike some languages, Rust does actually support mutable bindings.
// We can mark a binding as mutable with the mut keyword.
// In this code, we've created a variable with let mut, and it can be changed
// at will.
// ST
// Another feature of the Rust binding model is that the compiler will prevent
// you from using a variable you haven't initialized. Uninitialized variables
// generally contain garbage values and using them can lead to bugs, so the
// Rust compiler just won't consider the code shown here to be valid.
// ST
// In the next video, we'll see how to run Rust code without installing the
// Rust compiler, and examine some of the built-in types provided
// by the Rust language and standard library.

fn main() {
    let var = "value";
    var = "other value";
}