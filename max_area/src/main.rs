use std::cmp::{max, min};

struct Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = height.len() - 1;
        let mut result = 0;

        while left < right {
            let area = (right - left) as i32 * min(height[left], height[right]);
            result = max(area, result);
            if height[left] < height[right] {
                left += 1;
            } else if height[left] > height[right] {
                right -= 1;
            } else {
                right -= 1;
                left += 1;
            }
        }
        return result;
    }
}

fn main() {
    dbg!(Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]));
    dbg!(Solution::max_area(vec![1, 2, 3, 4, 3, 2, 1]));
    dbg!(Solution::max_area(vec![1, 1]));
}
