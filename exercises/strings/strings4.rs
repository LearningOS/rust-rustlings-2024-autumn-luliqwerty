// strings4.rs
//
// Ok, here are a bunch of values-- some are `String`s, some are `&str`s. Your
// task is to call one of these two functions on each value depending on what
// you think each value is. That is, add either `string_slice` or `string`
// before the parentheses on each line. If you're right, it will compile!
//
// No hints this time!


fn string_slice(arg: &str) {
    println!("{}", arg);
}
fn string(arg: String) {
    println!("{}", arg);
}

fn main() {
    // &str
    string_slice("blue");
    // String
    string("red".to_string());
    // String
    string(String::from("hi"));
    // String
    string("rust is fun!".to_owned());
    // String
    string("nice weather".into());
    // String
    string(format!("Interpolation {}", "Station"));
    // &str
    string_slice(&String::from("abc")[0..1]);
    // &str | &str.trim() -> remove the whitespace in the String
    string_slice("  hello there ".trim());
    // String
    string("Happy Monday!".to_string().replace("Mon", "Tues"));
    string("mY sHiFt KeY iS sTiCkY".to_lowercase());
}
