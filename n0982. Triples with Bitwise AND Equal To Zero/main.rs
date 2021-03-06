// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

impl Solution {
    pub fn count_triplets(nums: Vec<i32>) -> i32 {
        let max_num = nums.iter().skip(1).fold(nums[0], |a, b| { a.max(*b) } ) as usize;
        // dp[n] 为A[i] & A[j] = n的个数
        let mut dp = vec![0; max_num + 1];
        for ai in &nums {
            for aj in &nums { dp[(ai & aj) as usize] += 1; }
        }

        let mut ans = 0;
        for a in &nums {
            for b in (0..=max_num as i32) {
                if a & b == 0 {
                    ans += dp[b as usize];
                }
            }
        }

        ans
    }
}

