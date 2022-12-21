// 3. Longest Substring Without Repeating Characters
// Given a string s, find the length of the longest
// substring without repeating characters.

use std::cmp;
use std::collections::HashMap;

fn main() {
    let s = String::from("helloworld");

    let r = length_of_longest_substring(s);
    println!("substr length of non repeating sub-string: {}", r);
}

fn length_of_longest_substring(s: String) -> i32 {
    let mut h: HashMap<char, usize> = HashMap::new();
    let mut length = 0;
    let mut start = 0;
    //println!("start : {}", start);
    for (end, c) in s.char_indices() {
        if let Some(&n) = h.get(&c) {
            start = cmp::max(start, n);
        }
        length = cmp::max(length, end - start + 1);
        println!(
            "length : {length} compared to end - start + 1: {}",
            end - start + 1
        );
        h.insert(c, end + 1);
    }
    length as i32
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_length_of_longest_substring() {
        assert_eq!(3, length_of_longest_substring("pwwkew".to_string()));
    }
}
