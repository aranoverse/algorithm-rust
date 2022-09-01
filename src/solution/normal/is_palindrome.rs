use crate::solution::Solution;

impl Solution {
    /// https://leetcode.com/problems/palindrome-number/
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 || (x % 10 == 0 && x != 0) {
            return false;
        }

        let mut source = x;
        let mut reversed_num: i32 = 0;
        while source > 0 {
            reversed_num = reversed_num * 10 + source % 10;
            source /= 10;
        }

        x == reversed_num
    }

    pub fn is_palindrome_v2(x: i32) -> bool {
        let str: String = x.to_string();

        str.chars().rev().collect::<String>() == str
    }
}