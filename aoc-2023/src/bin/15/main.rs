#![allow(dead_code, unused)]

use rayon::iter::{IntoParallelIterator, ParallelBridge, ParallelIterator};

fn main() {
    let ex1 = include_str!("ex1.txt");
    let in1 = include_str!("in1.txt");
    println!("p1ex1 = {}", init_seq(ex1));
    println!("p1in1 = {}", init_seq(in1));
}

fn init_seq(input: &str) -> u64 {
    parse(input)
        .iter()
        .par_bridge()
        .into_par_iter()
        .map(|s| hash(s))
        .sum()
}

fn hash(s: &str) -> u64 {
    let mut val = 0;
    for ch in s.chars() {
        let code = ch as u32;
        val += code;
        val *= 17;
        val = val % 256;
    }
    val.into()
}

fn parse(input: &str) -> Vec<String> {
    input.trim().split(",").map(|s| s.to_string()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pt1_ex1() {
        let ex1 = include_str!("ex1.txt");
        let seq = init_seq(ex1);
        assert_eq!(seq, 1320);
    }

    #[test]
    fn test_hash() {
        assert_eq!(hash("HASH"), 52)
    }
}
