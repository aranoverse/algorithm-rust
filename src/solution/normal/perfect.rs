use crate::solution::Solution;

impl Solution {
    pub fn is_perfect_square(num: i32) -> bool {
        let mut l = 1;
        let mut r = num;

        while l <= r && l > 0 && r > 0 {
            let mid = (l + r) / 2;
            let square = mid * mid;
            if square < num {
                l = mid + 1;
            } else if square > num {
                r = mid - 1;
            } else {
                return true;
            }
        }

        return false;
    }
}