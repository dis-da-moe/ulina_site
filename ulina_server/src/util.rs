pub const ZERO_WIDTH: &str = "\u{200B}";

pub fn capitalise(string: &str) -> String {
    string
        .trim()
        .split(" ")
        .filter_map(|word| {
            let mut chars = word.chars().collect::<Vec<_>>();
            if let Some(first) = chars.get_mut(0) {
                *first = first.to_ascii_uppercase();

                Some(chars.iter().collect::<String>())
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}
