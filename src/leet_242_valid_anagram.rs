// https://leetcode.com/problems/valid-anagram/

/*
Given two strings s and t, return true if t is an anagram of s, and false otherwise.

An Anagram is a word or phrase formed by rearranging the letters of a different word or phrase,
typically using all the original letters exactly once.
*/

use std::collections::HashMap;

pub fn is_anagram(s: String, t: String) -> bool {
    string_to_hashmap(s) == string_to_hashmap(t)
}

fn string_to_hashmap(s: String) -> HashMap<char, i32> {
    let mut map: HashMap<char, i32> = HashMap::new();
    for c in s.chars() {
        map.entry(c).and_modify(|e| *e += 1).or_insert(1);
    }
    map
}

// fn keys_match<T: Eq + Hash, U, V>(
//     map1: &HashMap<T, U>,
//     map2: &HashMap<T, V>,
// ) -> bool {
//     map1.len() == map2.len() && map1.keys().all(|k| map2.contains_key(k))
// }

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            true,
            is_anagram("anagram".to_string(), "nagaram".to_string())
        );
    }

    #[test]
    fn case2() {
        assert_eq!(false, is_anagram("rat".to_string(), "car".to_string()));
    }
}
