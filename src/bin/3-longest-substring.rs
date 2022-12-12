
// 3. Longest Substring Without Repeating Characters
// Given a string s, find the length of the longest 
// substring without repeating characters.
use std::cmp;
use std::{collections::HashMap};

fn main() {

    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];

    let s1 = String::from("");
    let r = length_of_longest_substring(s); 
    println!("substr length of non repeating sub-string: {}",r);
}

fn length_of_longest_substring(s: String) -> i32 {
    let mut h: HashMap<char, usize> = HashMap::new();
    let mut length = 0;
    let mut start = 0;
    println!("start : {}", start);
    // for (i,c) in s.chars().enumerate() {
    //     if h.contains_key(&c) {
    //         break;
    //     }
    //     else {
    //         h.insert(c, i);
    //     }
    // }
    // h.keys().len() as i32
    // }

    for (end,c) in s.char_indices() {
        if let Some(&n) = h.get(&c) {
            start = cmp::max(start, n);
            println!("start : {start}, n: {n}");
        }
        length = cmp::max(length, end - start + 1);
        println!("length : {length} compared to end - start + 1: {}",end - start + 1 );
        h.insert(c, end+1);
        println!("hashmap: {:?}", h);
    }
    length as i32
}

 


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_length_of_longest_substring() {
        let s = "pwwkew".to_string();
        let result = length_of_longest_substring(s);
        assert_eq!(result, 3);
    }
}
