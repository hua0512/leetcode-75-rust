struct Solution;

// 334. Increasing Triplet Subsequence
// https://leetcode.com/problems/increasing-triplet-subsequence
impl Solution {
    pub fn increasing_triplet(nums: Vec<i32>) -> bool {
        let n = nums.len();

        // think:
        // we use two variables to keep track of the first and second minimum values
        // if we find a number that is greater than both of them, we return true
        let mut iv = i32::MAX;
        let mut jv = i32::MAX;

        // iterate through the array
        // T: O (n), S: O(1)
        for num in nums {
            // we have a first minimum
            if num <= iv {
                iv = num;
            // we have a second minimum
            } else if num <= jv {
                jv = num;
            // this number is greater than both the first and second minimum
            } else {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::increasing_triplet(vec![1, 2, 3, 4, 5]), true);
        assert_eq!(Solution::increasing_triplet(vec![5, 4, 3, 2, 1]), false);
        assert_eq!(Solution::increasing_triplet(vec![2, 1, 5, 0, 4, 6]), true);
        assert_eq!(
            Solution::increasing_triplet(vec![20, 100, 10, 12, 5, 13]),
            true
        );
    }
}
