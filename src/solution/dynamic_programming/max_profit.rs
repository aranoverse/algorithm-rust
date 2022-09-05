use crate::solution::Solution;

impl Solution {
    /// https://leetcode.com/problems/best-time-to-buy-and-sell-stock/
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut max_profit: i32 = 0;
        let mut idx_buy: usize = 0;

        for (idx, price) in prices.iter().enumerate() {
            if prices[idx_buy] > prices[idx] {
                idx_buy = idx
            };

            max_profit = max_profit.max(price - prices[idx_buy]);
        }

        max_profit
    }
}

#[cfg(test)]
mod tests {
    use crate::solution::Solution;
    use test_case::test_case;

    #[test_case([7, 6, 4, 3, 1].to_vec(), 0; "case 2")]
    #[test_case([7, 1, 5, 3, 6, 4].to_vec(), 5; "case 1")]
    fn test_max_profit(prices: Vec<i32>, max_profit: i32) {
        assert_eq!(Solution::max_profit(prices), max_profit);
    }
}