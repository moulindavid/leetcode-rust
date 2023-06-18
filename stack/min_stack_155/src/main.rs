struct MinStack {
    vec: Vec<i32>,
    min: Vec<i32>,
}

impl MinStack {
    fn new() -> Self {
        Self {
            vec: Vec::new(),
            min: Vec::new(),
        }
    }

    fn push(&mut self, val: i32) {
        if let Some(num) = self.min.last() {
            if num >= &val {
                self.min.push(val);
            }
        } else {
            self.min.push(val);
        }
        self.vec.push(val);
    }

    fn pop(&mut self) {
        let val = self.vec.pop().unwrap();
        if &val == self.min.last().unwrap() {
            self.min.pop();
        }
    }

    fn top(&self) -> i32 {
        *self.vec.last().unwrap()
    }

    fn get_min(&self) -> i32 {
        *self.min.last().unwrap()
    }
}
