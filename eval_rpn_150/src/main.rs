struct Solution;

impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut number_stack: Vec<i32> = Vec::new();

        for token in tokens {
            if let Ok(number) = token.parse::<i32>() {
                number_stack.push(number);
            } else {
                //Operator
                let second_operand = number_stack.pop().unwrap();
                let first_operand = number_stack.pop().unwrap();
                let result = match token.as_str() {
                    "+" => first_operand + second_operand,
                    "-" => first_operand - second_operand,
                    "*" => first_operand * second_operand,
                    "/" => first_operand / second_operand,
                    _ => 0,
                };
                number_stack.push(result);
            }
        }
        print!("{}", number_stack.len());
        return number_stack.pop().unwrap();
    }
}

fn main() {
    dbg!(Solution::eval_rpn(vec![
        "2".to_owned(),
        "13".to_owned(),
        "5".to_owned(),
        "/".to_owned(),
        "+".to_owned(),
    ]));
}
