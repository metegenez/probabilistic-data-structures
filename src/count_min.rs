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

use std::cmp::max;
use std::hash::{Hash, Hasher};
use twox_hash::xxh3::Hash64;

struct CountMinSketch {
    counts: Vec<Vec<u64>>,
}

fn hash_once<T: Sized + Hash>(hasher: &mut Hash64, item: &T) -> u64 {
    item.hash(hasher);
    hasher.finish()
}

impl CountMinSketch {
    fn new(probability: f64, tolerance: f64) -> Self {
        let width = Self::optimal_width(tolerance);
        let depth = Self::optimal_depth(probability);
        Self {
            counts: vec![vec![0; width]; depth],
        }
    }

    fn update<T: Sized + Hash>(&mut self, item: T, count: u64) {
        for (seed, row) in self.counts.iter_mut().enumerate() {
            let mut hash_function = Hash64::with_seed(seed as u64);
            let index = hash_once(&mut hash_function, &item) as usize % row.len();
            row[index] += count;
        }
    }

    fn estimate<T: Sized + Hash + std::fmt::Debug>(&mut self, item: T) -> u64 {
        self.counts
            .iter()
            .enumerate()
            .map(|(seed, row)| {
                let mut hash_function = Hash64::with_seed(seed as u64);
                row[hash_once(&mut hash_function, &item) as usize % row.len()]
            })
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
    let mut sketch = CountMinSketch::new(0.90, 40.0 / 100.0);

    sketch.update("apple", 1);
    sketch.update("banana", 10);
    sketch.update("cherry", 5);
    sketch.update("apple", 3);

    assert_eq!(sketch.estimate("apple"), 4);
    assert_eq!(sketch.estimate("banana"), 10);
    assert_eq!(sketch.estimate("cherry"), 5);
    assert_eq!(sketch.estimate("durian"), 0);
}
