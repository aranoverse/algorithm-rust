use crate::solution::Solution;

impl Solution {
    /// https://leetcode.com/problems/climbing-stairs/
    pub fn climb_stairs(n: i32) -> i32 {
        if n < 3 {
            return n;
        }

        let mut step_ways = Vec::with_capacity(n as usize + 1);

        for idx in 0..step_ways.capacity() {
            if idx < 3 {
                step_ways.push(idx);
                continue;
            }

            step_ways.push(step_ways[idx - 1] + step_ways[idx - 2]);
        }


        step_ways[n as usize] as i32
    }

    pub fn climb_stairs_v2(n: i32) -> i32 {
        let mut step_ways = vec![0, 1, 2];

        for idx in 3..=n as usize {
            step_ways.push(step_ways[idx - 1] + step_ways[idx - 2]);
        }


        step_ways[n as usize] as i32
    }

    pub fn climb_stairs_recurse(n: i32) -> i32 {
        if 0 <= n && n <= 2 {
            return n;
        }

        Self::climb_stairs(n - 1) + Self::climb_stairs(n - 2)
    }
}

#[cfg(test)]
mod tests {
    use crate::solution::Solution;
    use test_case::test_case;


    #[test_case(2, 2;)]
    #[test_case(3, 3;)]
    #[test_case(4, 5;)]
    fn test_climb_stairs(steps: i32, ways: i32) {
        assert_eq!(Solution::climb_stairs(steps), ways);
    }

    #[test_case(2, 2;)]
    #[test_case(3, 3;)]
    #[test_case(4, 5;)]
    fn test_climb_stairs_v2(steps: i32, ways: i32) {
        assert_eq!(Solution::climb_stairs_v2(steps), ways);
    }
}


