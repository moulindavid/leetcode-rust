struct Solution;

impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        //Using a stack that will store temperatures in a decreasing, or equal order.
        let mut temp_stack: Vec<(i32, usize)> = Vec::new();
        let mut day_to_wait: Vec<i32> = vec![Default::default(); temperatures.len()];

        for (i, temp) in temperatures.into_iter().enumerate() {
            //If we met a temperatures which is superior to the ones in the stack we can update the
            //number of day between these two temperatures.
            while !temp_stack.is_empty() && temp > temp_stack.last().unwrap().0 {
                let (_stack_temp, stack_index) = temp_stack.pop().unwrap();
                day_to_wait[stack_index] = (i - stack_index) as i32;
            }
            temp_stack.push((temp, i));
        }
        //This is not needed as we init the vec at 0.
        //
        //while !temp_stack.is_empty() {
        //    let (_stack_temp, stack_index) = temp_stack.pop().unwrap();
        //    day_to_wait[stack_index] = 0;
        //}

        return day_to_wait;
    }
}

fn main() {
    dbg!(Solution::daily_temperatures(vec![
        73, 74, 75, 71, 69, 72, 76, 73
    ]));
    dbg!(Solution::daily_temperatures(vec![
        1, 1, 1, 3, 3, 4, 3, 2, 4, 2
    ]));
}
