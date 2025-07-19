struct Solution;

// 1493. Longest Subarray of 1's After Deleting One Element
// https://leetcode.com/problems/longest-subarray-of-1s-after-deleting-one-element
impl Solution {
    pub fn longest_subarray(nums: Vec<i32>) -> i32 {
        // this is a specialization of 1004. Max Consecutive Ones III
        // with k = 1 and deletes one element

        let mut k = 1;

        let mut i = 0;
        let mut j = 0;

        let n = nums.len();
        while j < n {
            if nums[j] == 0 {
                k -= 1;
            }
            if k < 0 {
                if nums[i] == 0 {
                    k += 1;
                }
                i += 1;
            }
            j += 1;
        }

        // -1 because we delete one element
        (j - i - 1) as i32
    }
}
