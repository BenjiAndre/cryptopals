use itertools::Itertools;

fn hex_char_to_u8(c: char) -> Option<u8> {
    match c {
        '0'..='9' => Some(c as u8 - b'0'),
        'a'..='f' => Some(c as u8 - b'a' + 10),
        'A'..='F' => Some(c as u8 - b'A' + 10),
        _ => None,
    }
}

fn hex_chars_to_bytes(high: char, low: char) -> Option<u8> {
    let high = hex_char_to_u8(high)?;
    let low = hex_char_to_u8(low)?;
    Some((high << 4) | low)
}

pub fn hex_str_to_bytes(hex: &str) -> Option<Vec<u8>> {
    hex.chars()
        .tuples()
        .map(|(h, l)| hex_chars_to_bytes(h, l))
        .collect()
}
