struct Solution;

impl Solution {
    pub fn max_operations(mut nums: Vec<i32>, k: i32) -> i32 {
        let mut count = 0;

        if nums.is_empty() {
            return 0;
        }

        if nums.len() == 1 {
            return 0;
        }

        if nums.len() == 2 {
            return if nums[0] + nums[1] == k { 1 } else { 0 };
        }

        // sort the array
        // this is O (n log n) in the worst case
        // tip: use unstable sort to improve performance as we dont care about stability
        nums.sort_unstable();

        let mut i = 0;
        let mut j = nums.len() - 1;

        // think:
        // array is sorted now, so we have a pair of numbers that sum up to k
        // if sum is less than k, then we need to increase the left pointer obv
        // otherwise we need to decrease the right pointer
        // T: O(n log n) S: O(1)
        while i < j {
            let left = &nums[i];
            let right = &nums[j];
            let sum = left + right;

            if sum == k {
                count += 1;
                i += 1;
                j -= 1;
            } else if sum < k {
                i += 1;
            } else {
                j -= 1;
            }
        }

        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_operations() {
        assert_eq!(Solution::max_operations(vec![1, 2, 3, 4], 5), 2);
        assert_eq!(Solution::max_operations(vec![3, 1, 3, 4, 3], 6), 1);
        assert_eq!(Solution::max_operations(vec![2, 2, 2, 3, 1, 1, 4, 1], 4), 2);
        assert_eq!(
            Solution::max_operations(
                vec![4, 4, 1, 3, 1, 3, 2, 2, 5, 5, 1, 5, 2, 1, 2, 3, 5, 4],
                2
            ),
            2
        );
    }
}
