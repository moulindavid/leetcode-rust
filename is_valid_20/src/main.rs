struct Stack<T> {
    data: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Stack { data: Vec::new() }
    }

    fn push(&mut self, item: T) {
        self.data.push(item);
    }

    fn pop(&mut self) -> Option<T> {
        self.data.pop()
    }

    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    fn len(&self) -> usize {
        self.data.len()
    }

    fn peek(&self) -> Option<&T> {
        self.data.last()
    }
}

struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack: Stack<char> = Stack::new();

        for c in s.chars() {
            if c == '{' || c == '[' || c == '(' {
                stack.push(c);
            }
            match c {
                '}' => {
                    if stack.peek() == Some(&'{') {
                        stack.pop();
                    } else {
                        return false;
                    }
                }
                ')' => {
                    if stack.peek() == Some(&'(') {
                        stack.pop();
                    } else {
                        return false;
                    }
                }
                ']' => {
                    if stack.peek() == Some(&'[') {
                        stack.pop();
                    } else {
                        return false;
                    }
                }
                _ => {}
            }
        }
        return stack.is_empty();
    }
}

fn main() {
    dbg!(Solution::is_valid("()[]".to_owned()));
}
