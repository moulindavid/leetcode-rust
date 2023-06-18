struct Solution;

impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        //Stack containing index and height
        let mut height_stack: Vec<(i32, usize)> = Vec::new();
        let mut max_area = 0;
        let heights_len = heights.len();
        for (i, height) in heights.into_iter().enumerate() {
            let mut extended_index = i;
            //If the height is less than the previous one we should pop the previous one and add to the stack
            //the current one.
            while !height_stack.is_empty() && height < height_stack.last().unwrap().0 {
                let (stack_height, stack_index) = height_stack.pop().unwrap();
                if max_area < ((i - stack_index) as i32) * stack_height {
                    max_area = ((i - stack_index) as i32) * stack_height;
                }
                extended_index = stack_index;
                //Moving the index to where this rectangle start.
            }
            height_stack.push((height, extended_index));
        }
        //We need to pop the stack to the end to be sure to have the max area.
        while !height_stack.is_empty() {
            let (stack_height, stack_index) = height_stack.pop().unwrap();
            if max_area < ((heights_len - stack_index) as i32) * stack_height {
                max_area = ((heights_len - stack_index) as i32) * stack_height;
            }
        }
        return max_area;
    }
}

fn main() {
    dbg!(Solution::largest_rectangle_area(vec![2, 1, 5, 6, 2, 3]));
    dbg!(Solution::largest_rectangle_area(vec![
        2, 1, 5, 6, 2, 8, 8, 8
    ]));
    dbg!(Solution::largest_rectangle_area(vec![
        2, 1, 5, 6, 2, 2, 2, 2, 2, 2
    ]));
}
