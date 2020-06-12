pub fn raindrops(n: u32) -> String {
    let mut result = vec![];

    if n % 3 == 0 {
        result.push("Pling");
    }

    if n % 5 == 0 {
        result.push("Plang");
    }

    if n % 7 == 0 {
        result.push("Plong");
    }


    if result.is_empty() {
        return n.to_string()
    } else {
        return result.join("")
    }
}
