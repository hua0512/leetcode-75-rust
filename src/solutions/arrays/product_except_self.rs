struct Solution;

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut result = vec![1; n];

        // think about that for result[i] = products of all left elements of i * products of all right elements of i
        // for example, for [1, 2, 3], to determine result [1]
        // left side: result[1] = result[0]  * nums[0] = 1 * 1 = 1
        // right side: result[2] = result[2] * nums[2] = 1 * 3 = 3
        // result[1] = 1 * 3 = 3

        // we calculate prefix products
        for i in 1..n {
            // each of them is the previous * its value
            result[i] = result[i - 1] * nums[i - 1];
        }
        // calculate suffix products
        let mut suffix_product = 1;
        for i in (0..n).rev() {
            result[i] *= suffix_product;
            suffix_product *= nums[i];
        }

        

        result
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            Solution::product_except_self(vec![1, 2, 3, 4]),
            vec![24, 12, 8, 6]
        );
        assert_eq!(
            Solution::product_except_self(vec![-1, 1, 0, -3, 3]),
            vec![0, 0, 9, 0, 0]
        );
    }
}
