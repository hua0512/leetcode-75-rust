use core::num;

struct Solution;

// 724. Find Pivot Index
// https://leetcode.com/problems/find-pivot-index
impl Solution {
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        let mut pivot: i32 = -1;

        if nums.len() == 0 {
            return 0;
        }

        let mut acc = 0;

        // we get a total acc
        for i in 0..nums.len() {
            acc += nums[i];
        }

        let mut left = 0;

        // think:
        // we need to balance left and right
        // so, left is calculated by summing up left variables
        // to calculate i's right positions, we need to minus left acc and the current from total
        for i in 0..nums.len() {
            let current = nums[i];

            let right = acc - left - nums[i];

            if left == right {
                pivot = i as i32;
                break;
            }
            left += current;
        }

        pivot
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::pivot_index(vec![2, 1, -1]), 0)
    }
}
