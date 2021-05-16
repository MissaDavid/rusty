/*
Variables are immutable by default
Use `let` keyword to define a variable (JS devs, this is a `const` equivalent)
 */

// snake_case for function names
// note the absence of `return` in the function
pub fn print_variables() {
    let mascot = "Ferris";
    let animal = "crab";

    println!("Hello, I am {} the {}", mascot, animal);

    // animal = "rustacean"
    // println!("Hello, I am {} the {}", mascot, crab);

    /* const keyword exists but not used as much */
    // const OFFICIAL: bool = false;

}
