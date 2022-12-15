/*
Roman numerals are represented by seven different symbols: I, V, X, L, C, D and M.
Symbol       Value
I             1
V             5
X             10
L             50
C             100
D             500
M             1000

For example, 2 is written as II in Roman numeral, just two ones added together. 12 is written as XII, which is simply X + II. The number 27 is written as XXVII, which is XX + V + II.

Roman numerals are usually written largest to smallest from left to right. However, the numeral for four is not IIII. Instead, the number four is written as IV. Because the one is before the five we subtract it making four. The same principle applies to the number nine, which is written as IX. There are six instances where subtraction is used:

    I can be placed before V (5) and X (10) to make 4 and 9.
    X can be placed before L (50) and C (100) to make 40 and 90.
    C can be placed before D (500) and M (1000) to make 400 and 900.

Given a roman numeral, convert it to an integer.

 */

use std::collections::HashMap;

pub fn roman_to_int(s: String) -> i32 {
    let roman_lib: HashMap<char, i32> = HashMap::from([
        ('I', 1),
        ('V', 5),
        ('X', 10),
        ('L', 50),
        ('C', 100),
        ('D', 500),
        ('M', 1000),
    ]);
    //variable to store sum
    let mut total: i32 = 0;

    //store previous number
    let mut previous_number: i32 = 0;

    for char in s.to_uppercase().chars().rev() {
        if let Some(current_number) = roman_lib.get(&char) {
            if *current_number < previous_number {
                total -= current_number;
            } else {
                total += current_number;
            }
            previous_number = *current_number;
        } else {
            panic!("unexpected char found: [{}]", char)
        }
    }
    total
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(roman_to_int("III".to_string()), 3);
        assert_eq!(roman_to_int("IV".to_string()), 4);
    }
    #[test]
    fn test_2() {
        assert_eq!(roman_to_int("LVIII".to_string()), 58);
    }
    #[test]
    fn test_3() {
        assert_eq!(roman_to_int("MCMXCIV".to_string()), 1994);
    }
}
