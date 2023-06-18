struct Solution;

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut output: Vec<i32> = vec![1; nums.len()];
        let mut prefix = 1;
        for index in 0..nums.len() {
            output[index] = prefix;
            prefix *= nums[index];
        }

        let mut postfix = 1;

        for index in (0..nums.len()).rev() {
            output[index] *= postfix;
            postfix *= nums[index];
        }
        return output;
    }
}

fn main() {
    dbg!(Solution::product_except_self(vec![1, 2, 3, 4]));
    dbg!(Solution::product_except_self(vec![
        1, 1, 1, 3, 3, 4, 3, 2, 4, 2
    ]));
}
