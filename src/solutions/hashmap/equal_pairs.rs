use std::collections::BTreeMap;

struct Solution;

// 2352. Equal Row and Column Pairs
// https://leetcode.com/problems/equal-row-and-column-pairs
impl Solution {
    pub fn equal_pairs(grid: Vec<Vec<i32>>) -> i32 {
        // same grid but transposed
        let mut cols = vec![vec![0i32; grid.len()]; grid.len()];

        // transpose grid
        for i in 0..grid.len() {
            for j in 0..grid.len() {
                cols[i][j] = grid[j][i];
            }
        }
        let mut map = BTreeMap::<Vec<i32>, usize>::new();

        // construct a map of rows
        // we use BTreeMap because it is sorted
        for row in grid.into_iter() {
            // insert the row into the map
            *map.entry(row).or_insert(0) += 1;
        }

        let mut result = 0;

        // here we check if the column exists in the map
        // if so, then we add the count to the result
        for col in cols {
            if let Some(count) = map.get(&col) {
                result += *count as i32;
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_equal_pairs() {
        let grid = vec![vec![3, 2, 1], vec![1, 7, 6], vec![2, 7, 7]];
        assert_eq!(Solution::equal_pairs(grid), 1);
    }
}
