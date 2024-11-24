# Simple Rust Calculator

A simple console-based calculator application written in Rust. It accepts user input for two numbers and an operation, performs the calculation, and displays the result.

## Features

- Supports basic arithmetic operations: `+`, `-`, `*`, and `/`.
- Handles invalid input and division by zero gracefully.
- Interactive user input.

## Prerequisites

- Rust installed on your system. If Rust is not installed, you can install it using the following command:
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

## Usage

1. Clone this repository or create a new Rust project:
```bash
cargo new simple_calculator
cd simple_calculator
```

2. Build and run the application:
```bash
cargo run
```

## Example Output

```bash
Simple Calculator
==================
Enter the first number:
10
Enter the second number:
5
Enter the operation (+, -, *, /):
+
Result: 15
```

