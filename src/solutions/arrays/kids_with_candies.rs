struct Solution;

impl Solution {
    pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
        let mut result = vec![];

        if candies.len() == 0 {
            return result;
        }

        let max = candies.iter().max().unwrap();

        for candy in &candies {
            let all_candies = candy + extra_candies;

            if all_candies >= *max {
                result.push(true);
            } else {
                result.push(false);
            }
        }

        result
    }
}
