struct MinStack {
   container: Vec<i32>,
   prefix_container: Vec<i32>,
}

impl MinStack {
    pub fn new() -> Self {
        let mut prefix_container = Vec::new();
        prefix_container.push(i32::MAX);

        MinStack {
            container: Vec::new(),
            prefix_container
        }
    }

    pub fn push(&mut self, val: i32) {
        self.container.push(val);
        let current_min = *self.prefix_container.last().unwrap();
        self.prefix_container.push(current_min.min(val));
    }

    pub fn pop(&mut self) {
        self.container.pop();
        self.prefix_container.pop();
    }

    pub fn top(&self) -> i32 {
        *self.container.last().unwrap()
    }

    pub fn get_min(&self) -> i32 {
        *self.prefix_container.last().unwrap()
    }
}
