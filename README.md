# Probabilistic Data Structures in Streaming

This repository contains an implementation of the **[Count-Min Sketch](https://en.wikipedia.org/wiki/Count%E2%80%93min_sketch)** data structure (currently, will be added more) in Rust.

# ****Example****

```rust
// Create a new Count-Min Sketch with a confidence of 0.9 and a relative error of 0.01. 
let mut sketch = CountMinSketch::<&str>::new(0.9, 0.01);

sketch.update("apple", 1);
sketch.update("banana", 2);
sketch.update("apple", 3);
sketch.update("cherry", 4);

assert_eq!(sketch.estimate("apple"), 4);
assert_eq!(sketch.estimate("banana"), 2);
assert_eq!(sketch.estimate("cherry"), 4);
assert_eq!(sketch.estimate("durian"), 0);
```

## **Contributions**

We welcome contributions to this repository. If you have an idea for a new feature or have found a bug, please open an issue or submit a pull request.

## **License**

This project is licensed under the MIT License - see the **[LICENSE](https://chat.openai.com/LICENSE)** file for details.
