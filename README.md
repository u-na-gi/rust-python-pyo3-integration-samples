# Project Overview

This project demonstrates how to call Python code from Rust using the [PyO3](https://pyo3.rs/v0.22.3/) library. It showcases an integration where Rust executes embedded Python code, enabling the use of Python libraries, such as `numpy`, from Rust. The primary goal is to provide a sample for developers looking to harness Python's capabilities within a Rust project.

## Motivation

The motivation behind this project was the lack of detailed examples showing how to call Python from Rust, particularly for using Python's unique libraries that have no Rust equivalents. By creating this example, we aim to bridge the gap between Rust’s system programming strengths and Python’s rich ecosystem in data science and machine learning.

## Development Setup

### Step 1: Clone the Repository

Clone the repository to your local machine:

```bash
git clone https://github.com/your-username/your-repository.git
cd your-repository
```

### Step 2: Build and Set Up the Environment

Use the following command to build the Docker environment and set up the necessary dependencies:

```bash
make build
```

This command will:
- Build Docker images for the project.
- Force container recreation, ensuring a fresh setup every time.
- Remove orphaned containers to keep the environment clean.

### Step 3: Run the Tests

After setting up the environment, you can run the tests that demonstrate the interaction between Rust and Python. These tests verify that the Python code is properly executed from Rust, ensuring that the integration works as expected.

To run the tests, use:

```bash
cargo test
```

This command will execute the Rust unit tests, which include calls to Python functions embedded in the project. The tests cover scenarios such as:

- Loading a Python script at runtime.
- Calling a Python function (`my_numpy_function`) that performs numerical calculations using `numpy`.
- Extracting the result of the Python function back into Rust and validating the output.

### Running Tests in Docker

If you are running the environment inside Docker, make sure to use the following command to run tests in the containerized environment:

```bash
docker compose exec <container_name> cargo test
```

Replace `<container_name>` with the name of the container running your Rust application.

## Key Considerations

### Global Interpreter Lock (GIL)

One critical aspect of interacting with Python from Rust is managing Python's Global Interpreter Lock (GIL). Since only one thread can execute Python code at a time, ensure that you do not attempt to acquire the GIL in multiple threads simultaneously. In the tests, the GIL is managed carefully using `Python::with_gil()` to avoid any conflicts.

### Test Reliability

Ensure that each test interacts with Python in a single-threaded manner when working with Python libraries such as `numpy`. This will prevent race conditions or deadlocks, which could arise due to improper GIL handling.

## Reference

For more detailed information about using PyO3 to embed Python code in Rust, please refer to the official [PyO3 documentation](https://pyo3.rs/v0.22.3/). This will guide you through advanced features such as GIL handling, object extraction, and calling Python functions from Rust.