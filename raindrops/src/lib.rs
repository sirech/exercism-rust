pub fn raindrops(n: u32) -> String {
    let mut result = vec![];

    let mut checker = |rem, word| {
        if n % rem == 0 {
            result.push(word);
        }
    };

    checker(3, "Pling");
    checker(5, "Plang");
    checker(7, "Plong");

    if result.is_empty() { n.to_string() } else { result.join("") }
}
