use std::ops::Deref;
use crate::solution::Solution;

impl Solution {
     /**  [triangle](https://leetcode.com/problems/triangle/)
       * Given a triangle array, return the minimum path sum from top to bottom.
       *
       * For each step, you may move to an adjacent number of the row below. More formally, if you are on index i on the current row, you may move to either index i or index i + 1 on the next row.
       *
       * Example 1:
       *
       * Input: triangle = [[2],[3,4],[6,5,7],[4,1,8,3]]
       * Output: 11
       * Explanation: The triangle looks like:
       *    2
       *   3 4
       *  6 5 7
       * 4 1 8 3
       * The minimum path sum from top to bottom is 2 + 3 + 5 + 1 = 11 (underlined above).
       * Example 2:
       *
       * Input: triangle = [[-10]]
       * Output: -10
       *
       *
       * Constraints:
       *
       * 1 <= triangle.length <= 200
       * triangle[0].length == 1
       * triangle[i].length == triangle[i - 1].length + 1
       *  -104 <= triangle[i][j] <= 104
      */

    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        if triangle.len() == 0 || triangle[0].len() == 0 {
            return 0;
        }

        // Init first row
        let mut minimum_triangle = triangle.clone();

        for row in 1..triangle.len() {
            let cols: &Vec<i32> = &triangle[row];

            for col in 0..cols.len() {
                if col == 0 {
                    minimum_triangle[row][col] = minimum_triangle[row - 1][col] + cols[col];
                    continue;
                }

                if col >= triangle[row - 1].len() {
                    minimum_triangle[row][col] = minimum_triangle[row - 1][col - 1] + cols[col];
                    continue;
                }

                minimum_triangle[row][col] = minimum_triangle[row - 1][col].min(minimum_triangle[row - 1][col - 1]) + cols[col];
            }
        }

        *minimum_triangle[minimum_triangle.len() - 1].iter().min().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::solution::Solution;
    use test_case::test_case;

    #[test_case([[2].to_vec(), [3, 4].to_vec(), [6, 5, 7].to_vec(), [4, 1, 8, 3].to_vec()].to_vec(), 11; "case 1")]
    fn test_minimum_total(triangle: Vec<Vec<i32>>, minimum_total: i32) {
        assert_eq!(Solution::minimum_total(triangle), minimum_total);
    }
}