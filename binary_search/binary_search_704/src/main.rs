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
            if middle_value < target {
                lower_bound = middle + 1;
                continue;
            }

            if middle_value > target {
                upper_bound = middle - 1;
            }
        }
        return -1;
    }
}

fn main() {
    dbg!(Solution::search(vec![-1, 0, 3, 5, 9, 12], 9));
    dbg!(Solution::search(vec![-1, 0, 3, 5, 9, 12], 2));
}
