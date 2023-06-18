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
    pub fn search_matrix_binary_search(nums: Vec<Vec<i32>>, target: i32) -> bool {
        let rows: i32 = nums.len() as i32;
        let cols: i32 = nums[0].len() as i32;

        let mut top: i32 = 0;
        let mut bot: i32 = rows - 1;
        let mut row = 0;
        while top <= bot {
            row = (top + bot) / 2;
            if *nums[row as usize].last().unwrap() < target {
                top = row + 1;
                continue;
            }
            if nums[row as usize][0] > target {
                bot = row - 1;
            } else {
                break;
            }
        }
        if top > bot {
            return false;
        };
        let mut lower_bound: i32 = 0;
        let mut upper_bound: i32 = cols - 1;

        while lower_bound <= upper_bound {
            let col = (upper_bound + lower_bound) / 2;
            let middle_value = nums[row as usize][col as usize];

            if middle_value == target {
                return true;
            }
            if middle_value < target {
                lower_bound = col + 1;
                continue;
            }

            if middle_value > target {
                upper_bound = col - 1;
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
    dbg!(Solution::search_matrix_binary_search(vec![vec![1]], 0));
    dbg!(Solution::search_matrix_binary_search(
        vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]],
        3
    ));
    dbg!(Solution::search_matrix_binary_search(
        vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]],
        13
    ));
    dbg!(Solution::search_matrix(vec![vec![1]], 0));
}
