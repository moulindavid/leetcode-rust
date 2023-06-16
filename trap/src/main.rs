struct Solution;

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        if height.is_empty() {
            return 0;
        }
        let (mut left, mut right) = (0, height.len() - 1);
        let (mut max_left, mut max_right) = (height[left], height[right]);
        let mut result = 0;
        while left < right {
            if max_left <= max_right {
                left += 1;
                max_left = max_left.max(height[left]);
                // if the center height is increasing we add 0.
                result += max_left - height[left];
            } else {
                right -= 1;
                max_right = max_right.max(height[right]);
                result += max_right - height[right];
            }
        }

        return result;
    }
}

fn main() {
    dbg!(Solution::trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]));
    dbg!(Solution::trap(vec![4, 2, 0, 3, 2, 5]));
}
