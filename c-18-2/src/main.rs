fn main() {
    let some_option_value = Some("some");
    if let Some(x) = some_option_value {
        println!("{x}");
    }
}
