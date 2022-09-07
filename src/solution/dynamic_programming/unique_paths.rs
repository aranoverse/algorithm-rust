use crate::solution::Solution;

impl Solution {
    /// https://leetcode.com/problems/unique-paths/
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let m = m as usize;
        let n = n as usize;

        let mut grid_ways: Vec<Vec<i32>> = vec![vec![0; n]; m];

        for idx in 0..n {
            grid_ways[0][idx] = 1;
        }
        for idx in 0..m {
            grid_ways[idx][0] = 1;
        }

        ///  formula: grid_ways[i][j] = grid_ways[i-1][j]  + grid_ways[i][j-1]
        for idx_m in 1..m {
            for idx_n in 1..n {
                grid_ways[idx_m][idx_n] = grid_ways[idx_m - 1][idx_n] + grid_ways[idx_m][idx_n - 1];
            }
        }

        grid_ways[m - 1][n - 1]
    }
}


#[cfg(test)]
mod tests {
    use crate::solution::Solution;
    use test_case::test_case;

    #[test_case(3, 2, 3; "case 2")]
    #[test_case(3, 7, 28; "case 1")]
    fn test_unique_paths(m: i32, n: i32, ways: i32) {
        assert_eq!(Solution::unique_paths(m, n), ways);
    }
}