struct Solution;

impl Solution {
    pub fn search_matrix(nums: Vec<Vec<i32>>, target: i32) -> bool {
        let rows = nums.len();
        let cols = nums[0].len();

        let mut left: i32 = 0;
        let mut right: i32 = (rows * cols - 1) as i32;

        while left <= right {
            let middle = left + (right - left) / 2;
            let middle_row = middle / cols as i32;
            let middle_col = middle % cols as i32;
            let middle_value = nums[middle_row as usize][middle_col as usize];

            if middle_value == target {
                return true;
            }
            if middle_value < target {
                left = middle + 1;
                continue;
            }
            if middle_value > target {
                right = middle - 1;
            }
        }
        return false;
    }
}

fn main() {
    dbg!(Solution::search_matrix(
        vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]],
        3
    ));
    dbg!(Solution::search_matrix(
        vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]],
        13
    ));

    dbg!(Solution::search_matrix(vec![vec![1]], 0));
}
