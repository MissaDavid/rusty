mod variables;
mod types;


fn main() {
    variables::print_variables();
    types::discover_ints();

    // let name = "Ferris".to_string(); // or String::from_str("Ferris")
    // types::gimme_your_name(&name);

    /* When the compiler sees a &String being passed to a function that takes &str,
       it coerces the &String into a &str
    */
    let string = "Ferris";
    types::gimme_your_name(string);
}
