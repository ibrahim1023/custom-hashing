# Custom Cryptographic Hash Functions

This project implements two custom cryptographic hash functions, `customhash1` and `customhash2`, utilizing advanced cryptographic techniques. The algorithms are designed to provide unique hash outputs for given inputs, featuring salting and a focus on security.

## Features

- **Custom Hash Functions**: Two distinct hashing algorithms (`customhash1` and `customhash2`) that utilize bitwise operations and modular arithmetic.
- **Salting**: Enhances security by appending a random salt to the input, making it resistant to rainbow table attacks.
- **Benchmarking**: Performance evaluation of the hash functions using the `criterion` crate.

## Usage
Run the tests
```
cargo test
```

## Modules

The project is organized into the following modules:

- **`hash`**: Contains the implementation of the custom hash functions.
  - **`customhash1`**: The first custom hashing algorithm.
  - **`customhash2`**: The second custom hashing algorithm, which includes salting.
  
- **`utils`**: Contains utility functions that support the main hashing algorithms, such as random salt generation.

- **`benchmarks`**: Contains benchmark tests to evaluate the performance of the hashing functions using the `criterion` crate.

- **`tests`**: Contains unit tests for the hashing algorithms to ensure correctness and performance.

## Dependencies

- [criterion](https://github.com/bheisler/criterion.rs): It helps you write fast code by detecting and measuring performance improvements or regressions, even small ones, quickly and accurately. You can optimize with confidence, knowing how each change affects the performance of your code.
