struct Solution;

// 643. Maximum Average Subarray I
// https://leetcode.com/problems/maximum-average-subarray-i
impl Solution {
    pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
        let mut max = 0;

        if nums.len() == 0 || k == 0 {
            return 0.0;
        }
        let k = k as usize;

        if k > nums.len() {
            return 0.0;
        }

        // sliding window
        // computes sum of first k elements
        for i in 0..k {
            max += nums[i];
        }

        // this is the current max of the sliding window
        let mut current = max;
        // T: O(n) S: O(1)
        for i in k..nums.len() {
            let value = nums[i];
            // think:
            // when we slide, the current max is the previous minus the last value of the previous plus the current element value
            // this is a slide
            current = current - nums[i - k] + value;
            max = max.max(current);
        }

        max as f64 / k as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_max_average() {
        assert_eq!(
            Solution::find_max_average(vec![1, 12, -5, -6, 50, 3], 4),
            12.75
        );
        assert_eq!(Solution::find_max_average(vec![0, 4, 0, 3, 2], 1), 4.0);
    }
}
