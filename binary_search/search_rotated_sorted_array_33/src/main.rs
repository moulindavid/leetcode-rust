struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut lower_bound: i32 = 0;
        let mut upper_bound: i32 = nums.len() as i32 - 1;
        while lower_bound <= upper_bound {
            let middle = (upper_bound + lower_bound) / 2;
            let middle_value = nums[middle as usize];

            if middle_value == target {
                return middle;
            }
            // left sorted portion
            if middle_value >= nums[lower_bound as usize] {
                if target > middle_value || target < nums[lower_bound as usize] {
                    lower_bound = middle + 1;
                } else {
                    upper_bound = middle - 1;
                }
                continue;
            }
            // right sorted portion
            else if target > nums[upper_bound as usize] || target < middle_value {
                upper_bound = middle - 1;
            } else {
                lower_bound = middle + 1;
            }
        }
        return -1;
    }
}

fn main() {
    dbg!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 0));
    dbg!(Solution::search(vec![1, 2], 2));
}
