pub struct CombinationsWithReplacement<'a, T> {
    items: &'a [T],
    n: usize,
    indices: Vec<usize>,
    done: bool,
}

impl<'a, T: Clone> CombinationsWithReplacement<'a, T> {
    pub fn new(items: &'a [T], n: usize) -> Self {
        if items.is_empty() {
            return Self {
                items,
                n,
                indices: vec![],
                done: true,
            };
        }

        Self {
            items,
            n,
            indices: vec![0; n], // start at [0,0,...,0]
            done: n == 0,        // n=0 => one empty combination
        }
    }
}

impl<'a, T: Clone> Iterator for CombinationsWithReplacement<'a, T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }

        // Emit current combination
        let result = self
            .indices
            .iter()
            .map(|&i| self.items[i].clone())
            .collect::<Vec<_>>();

        let k = self.items.len();

        // Find rightmost index that can be incremented
        let mut pos = self.n;
        while pos > 0 {
            pos -= 1;

            if self.indices[pos] < k - 1 {
                // Increase this index
                let new_val = self.indices[pos] + 1;
                self.indices[pos] = new_val;

                // And propagate the value to the right (non-decreasing)
                for i in pos + 1..self.n {
                    self.indices[i] = new_val;
                }

                return Some(result);
            }
        }

        // All indices were at max → finished
        self.done = true;
        Some(result)
    }
}
