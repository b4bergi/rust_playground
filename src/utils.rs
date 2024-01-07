use regex::Regex;

pub fn get_text_detection_score(text: &str) -> usize {
    let regex = Regex::new(r"([A-Z]|[a-z]|[.!?\-' ]|')").unwrap();
    let matching_chars = text.chars()
        .filter(|&c| regex.is_match(&c.to_string()))
        .count();

    return matching_chars;
}

pub fn get_hamming_distance(x: &[u8], y: &[u8]) -> u64 {
    x.iter().zip(y).fold(0, |a, (b, c)| a + (*b ^ *c).count_ones() as u64)
}