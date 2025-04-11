# Space Mission Log Analysis

## Overview
This Rust application scans a large space mission log file (potentially gigabytes in size) to find the **longest successful Mars mission**. It prints the security code for that mission (in the format `ABC-123-XYZ`).

## Project Structure
```
space_mission_log_analysis/
├── .github/
│   └── workflows/
│       └── ci.yml          <-- GitHub Actions CI workflow
├── Makefile                <-- Build, test, lint, check, etc.
├── README.md               <-- Documentation: build/test instructions, usage, etc.
├── Cargo.toml              <-- Rust project manifest
├── src/
│   ├── main.rs             <-- High-level coordinator: CLI parsing and orchestration
│   ├── cli.rs              <-- CLI argument definition and handling
│   ├── parser.rs           <-- Module to parse lines of the space mission log
│   └── analyzer.rs         <-- Module to filter missions and determine the "longest successful Mars mission"
└── tests/
    └── integration_test.rs <-- End-to-end integration tests
```
---

### How it Works
- **Rayon** is used for parallel processing of large logs, which is generally faster for CPU-bound tasks.
- **CLI** is powered by [clap](https://crates.io/crates/clap). 
- The program ignores commented lines (`# ...`) and only considers missions with:
  1. Destination `Mars`
  2. Status `Completed`
  3. Properly parsed `duration`
- The mission with the largest `duration` is selected, and its security code is printed.

## Usage
1. **Build** the program:
    ```bash
    make build
    ```
2. **Run** on a log file:
    ```bash
    ./target/debug/space_mission_log_analysis --file ./space_missions.log
    ```
3. The output will be the security code of the longest successful Mars mission.

## Local Testing
- **Run all unit tests**:
    ```bash
    make test
    ```
- **Run integration test**:
    ```bash
    make integration
    ```
- **Check code format**:
    ```bash
    make fmt
    ```
- **Run lint**:
    ```bash
    make lint
    ```

## CI Explanation
- The GitHub Actions workflow (`.github/workflows/ci.yml`) checks out the repository, sets up Rust, and runs a **build**, **format check**, **lint** check, **unit tests**, and **integration tests**. 
- If any of these steps fail, the build is marked as failing.


## Concurrency vs Parallelism

Reasoning Why Choosing Rayon Over Tokio

Rayon excels at parallelizing CPU-bound computations. When processing a large text file and performing significant parsing logic, we are effectively CPU-bound for splitting and analyzing each line. Rayon’s data parallel approach (par_lines(), par_iter()) is straightforward and highly efficient for such scenarios.
Tokio is great for async I/O and concurrency when the program awaits many I/O-bound tasks. Since the challenge primarily involves local file reading plus intensive parsing, it’s more beneficial to parallelize the CPU-bound parsing using Rayon’s threads pool.