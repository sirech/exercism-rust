pub fn reverse(input: &str) -> String {
    let mut chars = vec![];
    chars.extend(input.chars());
    chars.reverse();

    chars.into_iter().collect()
}
