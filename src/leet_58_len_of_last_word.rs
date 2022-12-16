

// Given a string s consisting of words and spaces, return the length of the last word in the string.

// A word is a maximal substring consisting of non-space characters only.

 
pub fn length_of_last_word(s: String) -> i32 {
        let str: Vec<&str> = s.split_ascii_whitespace().collect();
        let length = str.get(str.len() -1).unwrap().len();
        length as i32
}


#[cfg(test)]
mod tests {
   use super::*;

    #[test]
    fn case1() { 
        assert_eq!( length_of_last_word("Hello World".to_string()), "World".len() as i32);
    }

    #[test]
    fn case2() { 
        assert_eq!( length_of_last_word("   fly me   to   the moon  ".to_string()), "moon".len() as i32);
    }

    #[test]
    fn case3() { 
        assert_eq!( length_of_last_word("luffy is still joyboy".to_string()), "joyboy".len() as i32);
    }
}