#![allow(dead_code, unused)]

fn main() {
    println!("{}", hash("HASH"));
}

fn hash(s: &str) -> u32 {
    let mut val = 0;
    for ch in s.chars() {
        let code = ch as u32;
        val += code;
        val *= 17;
        val = val % 256;
    }
    val
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash() {
        assert_eq!(hash("HASH"), 52)
    }
}
