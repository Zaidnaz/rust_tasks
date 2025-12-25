# Rust AI Task Categorizer

A high-performance command-line interface (CLI) tool built in Rust that uses the Gemini 2.5 Flash API to classify unstructured text tasks into structured categories.

This project demonstrates asynchronous networking, type-safe JSON serialization, and robust error handling in a systems programming environment.

---

## Project Overview

The Rust AI Task Categorizer bridges local batch processing with cloud-based Large Language Models (LLMs). Leveraging Rust’s memory safety guarantees and the Tokio async runtime, the application efficiently processes task lists without the overhead of a garbage collector.

It is designed for developers who want a fast, reliable, and scalable way to categorize text-based tasks using modern AI APIs.

---

## Technical Features

- Asynchronous Architecture  
  Uses the Tokio runtime for non-blocking I/O, enabling high throughput during API requests.

- Type-Safe Serialization  
  Implements serde and serde_json to ensure strict consistency between Rust data structures and JSON output.

- API Resiliency  
  Detects and reports API quota limits, invalid responses, and network timeouts gracefully.

- Batch Processing  
  Reads multi-line input files and aggregates categorized results into a single formatted JSON output.

---

## Prerequisites

- Rust Toolchain: Version 1.92 or higher  
- Google AI Studio Credentials: A valid API key for the Gemini 2.5 Flash model

---

## Installation and Configuration

### 1. Clone the Repository

```bash
git clone https://github.com/Zaidnaz/rust_tasks.git
cd rust_tasks
````

### 2. Build Dependencies

Fetch and compile all required crates:

```bash
cargo build
```

### 3. Configure API Key

Update the `api_key` variable in `src/main.rs` with your Gemini API key:

```rust
let api_key = "YOUR_API_KEY_HERE";
```

For production use, storing API keys in environment variables is strongly recommended.

---

## Usage Guide

### Preparing Input

Create a file named `tasks.txt` in the project root directory. Each line should contain one task:

```txt
Refactor the authentication logic in the backend
Buy groceries for the weekend
Develop a smart contract for a crowdfunding dApp
```

### Running the Application

Execute the categorization process:

```bash
cargo run
```

---

## Output Format

The application generates a file named `batch_results.json` with categorized results:

```json
[
  {
    "text": "Refactor the authentication logic",
    "category": "Coding"
  },
  {
    "text": "Buy groceries",
    "category": "Personal"
  }
]
```

---

## Dependencies

* tokio – Multi-threaded async runtime
* reqwest – HTTP client for API communication
* serde / serde_json – Safe and efficient data serialization

---

## License

This project is licensed under the MIT License. See the LICENSE file for details.

---

## Author

Mahamad Zaid
Rust, Systems Programming, AI Integration


