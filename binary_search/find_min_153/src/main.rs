struct Solution;

impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return nums[0];
        }
        let mut lower_bound: i32 = 0;
        let mut upper_bound: i32 = nums.len() as i32 - 1;

        while lower_bound < upper_bound {
            let middle = (upper_bound + lower_bound) / 2;

            let middle_value = nums[middle as usize];
            let left = nums[lower_bound as usize];
            let right = nums[upper_bound as usize];

            if left <= middle_value && middle_value <= right {
                return left;
            } else if left >= middle_value && middle_value >= right {
                return right;
            } else if left > middle_value || middle_value < right {
                upper_bound = middle;
                continue;
            } else {
                lower_bound = middle;
            }
        }
        return -1;
    }
}

fn main() {
    dbg!(Solution::find_min(vec![3, 4, 5, 1, 2]));
    dbg!(Solution::find_min(vec![4, 5, 6, 7, 0, 1, 2]));
    dbg!(Solution::find_min(vec![5, 1, 2, 3, 4]));
}
