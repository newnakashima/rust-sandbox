pub fn convert(raw: String) -> String {
    let first = match raw.chars().next() {
        Some(c) => c,
        None => return String::from(""),
    };

    let remaining = &raw[1..raw.len()].to_string();
    return match first {
        'a'|'i'|'u'|'e'|'o' => format!("{}-hay", raw),
        _ => format!("{}-{}ay", remaining, first),
    }
}

#[cfg(test)]
mod tests {
    use super::convert;

    #[test]
    fn test_convert() {
        let result = convert(String::from("first"));
        assert_eq!(result, "irst-fay");

        let result = convert(String::from("apple"));
        assert_eq!(result, "apple-hay");
    }
}
