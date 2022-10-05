use crate::solution::Solution;

impl Solution {
    // https://leetcode.com/problems/unique-paths-ii/
    pub fn unique_paths_with_obstacles(mut obstacle_grid: Vec<Vec<i32>>) -> i32 {
        let m = obstacle_grid.len();
        let n = obstacle_grid.first().unwrap().len();
        let mut grid_ways: Vec<Vec<i32>> = vec![vec![0; n]; m];

        for idx in 0..n {
            if obstacle_grid[0][idx] == 1 {
                grid_ways[0][idx] = 0;
                continue;
            }

            if idx > 0 && grid_ways[0][idx - 1] == 0 {
                grid_ways[0][idx] = 0;
                continue;
            }

            grid_ways[0][idx] = 1;
        }

        for idx in 0..m {
            if obstacle_grid[idx][0] == 1 {
                grid_ways[idx][0] = 0;
                continue;
            }

            if idx > 0 && grid_ways[idx - 1][0] == 0 {
                grid_ways[idx][0] = 0;
                continue;
            }

            grid_ways[idx][0] = 1;
        }

        ///  formula: grid_ways[i][j] = grid_ways[i-1][j]  + grid_ways[i][j-1]
        for idx_m in 1..m {
            for idx_n in 1..n {
                if obstacle_grid[idx_m][idx_n] == 1 {
                    grid_ways[idx_m][idx_n] = 0;
                    continue;
                }

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

    #[test_case([[0, 1, 0, 0, 0].to_vec(), [1, 0, 0, 0, 0].to_vec(), [0, 0, 0, 0, 0].to_vec(), [0, 0, 0, 0, 0].to_vec()].to_vec(), 0; "case 2")]
    #[test_case([[0, 0, 0].to_vec(), [0, 1, 0].to_vec(), [0, 0, 0].to_vec()].to_vec(), 2; "case 1")]
    fn test_unique_paths(obstacle_grid: Vec<Vec<i32>>, ways: i32) {
        assert_eq!(Solution::unique_paths_with_obstacles(obstacle_grid), ways);
    }
}