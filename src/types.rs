/*
Primitive and Custom Types
Integers
    U = unsigned (no negetive numbers)
    I = Signed
    F = floats
    Then any combination of a letter + bit size
    8-bit, 16-bit, 32-bit, 64-bit, 128-bit
Boolean bool
Characters char - single unicode character
Tuples, Arrays (fixed length), Vectors (resizable arrays)
Struct, Enum
...
 */

pub fn discover_ints() {
    let x = 1; // compiler is using the default type of i32!

    println!("Max value for an i32: {}", i32::MAX);
    println!("Max value for an i64: {}", i64::MAX);
}

// String VS str
// Functions are one place where the compiler will not work out types for you
pub fn gimme_your_name(name: &str) {
    println!("Your name is {}", name);

    let mut hello = String::new();
    // there is a difference between single and double quotes !!
    hello.push('H');
    hello.push_str("ello");

    // only prints to console if failing
    assert_eq!(hello, "Hello");
}