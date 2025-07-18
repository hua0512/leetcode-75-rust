struct Solution;

// 283. Move Zeroes
// https://leetcode.com/problems/move-zeroes
impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        // last non zero position
        let mut j = 0;

        // iterate
        // T: O(n), S: O(1)
        for i in 0..nums.len() {
            let num = nums[i];

            if num != 0 {
                if i != j {
                    nums[j] = nums[i];
                    nums[i] = 0;
                }

                j += 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test() {
        Solution::move_zeroes(&mut vec![0, 1, 0, 3, 12]);
    }
}
