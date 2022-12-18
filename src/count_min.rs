// Copyright (c) 2022 Metehan Yıldırım

// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:

// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.

// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use rand::Rng;
use std::cmp::max;
use std::hash::{Hash, Hasher};

struct CountMinSketch<T> {
    counts: Vec<Vec<u64>>,
    hash_functions: Vec<Box<dyn Fn(&T) -> u64>>,
}

impl<T: Sized + Hash> CountMinSketch<T> {
    fn new(probability: f64, tolerance: f64) -> Self {
        let width = Self::optimal_width(tolerance);
        let depth = Self::optimal_depth(probability);
        let mut rng = rand::thread_rng();
        let mut hash_functions: Vec<Box<dyn Fn(&T) -> u64>> = Vec::new();
        for _ in 0..depth {
            let a: u64 = rng.gen_range(0..u64::pow(10, 9));
            let b: u64 = rng.gen_range(0..u64::pow(10, 9));
            let p: u64 = rng.gen_range(0..u64::pow(10, 9));
            let modulus: u64 = rng.gen_range(0..u64::pow(10, 9));
            hash_functions.push(Box::new(move |x| {
                let mut hasher = std::collections::hash_map::DefaultHasher::new();
                x.hash(&mut hasher);
                let h_modulus = hasher.finish() % modulus;
                ((a * h_modulus + b) % p) % width as u64
            }));
        }

        Self {
            counts: vec![vec![0; width]; depth],
            hash_functions,
        }
    }

    fn update(&mut self, item: T, count: u64) {
        for i in 0..self.hash_functions.len() {
            let hash_function = &self.hash_functions[i];
            let index = (hash_function)(&item) as usize;
            self.counts[i][index] += count;
        }
    }

    fn estimate(&self, item: T) -> u64 {
        self.hash_functions
            .iter()
            .zip(&self.counts)
            .map(|(hash_function, row)| row[(hash_function)(&item) as usize])
            .min()
            .unwrap_or(0)
    }

    fn optimal_width(tolerance: f64) -> usize {
        let e = tolerance;
        let width = (2.718 / e).round() as usize;
        max(2, width)
            .checked_next_power_of_two()
            .expect("Width would be way too large") as usize
    }

    fn optimal_depth(probability: f64) -> usize {
        max(
            1,
            ((1.0 - probability).ln() / 0.5_f64.ln()).floor() as usize,
        )
    }
}

#[test]
fn test_count_min_sketch() {
    let mut sketch = CountMinSketch::<&str>::new(0.95, 10.0 / 100.0);

    sketch.update("apple", 1);
    sketch.update("banana", 2);
    sketch.update("apple", 3);
    sketch.update("cherry", 4);

    assert_eq!(sketch.estimate("apple"), 4);
    assert_eq!(sketch.estimate("banana"), 2);
    assert_eq!(sketch.estimate("cherry"), 4);
    assert_eq!(sketch.estimate("durian"), 0);
}
