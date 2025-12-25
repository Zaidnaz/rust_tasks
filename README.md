Rust AI Task Categorizer
A high-performance command-line interface tool built in Rust that utilizes the Gemini 2.5 Flash API to classify unstructured text tasks into structured categories. This project demonstrates the implementation of asynchronous networking, safe JSON serialization, and robust error handling in a systems programming environment.

Project Overview
The Rust AI Task Categorizer bridges the gap between local data management and cloud-based Large Language Models. By leveraging Rust's memory safety and the Tokio runtime, the application processes batch tasks efficiently without the overhead of a garbage collector.

Technical Features
Asynchronous Architecture: Utilizes the Tokio runtime for non-blocking I/O operations, ensuring high throughput during network requests.

Type-Safe Serialization: Employs the Serde framework to guarantee that data structures remain consistent between the application and the JSON output.

API Resiliency: Includes logic to detect and report API quota limitations and network timeouts.

Batch Processing: Capable of reading multi-line input files and aggregating categorized results into a single formatted document.

Prerequisites
Rust Toolchain: Version 1.92 or higher.

Google AI Studio Credentials: A valid API key for the Gemini 2.5 Flash model.

Installation and Configuration
1. Repository Setup
Clone the repository to your local machine:

Bash

git clone https://github.com/Zaidnaz/rust_tasks.git
cd rust_tasks
2. Dependency Resolution
Ensure all necessary crates are fetched and compiled:

Bash

cargo build
3. Environment Configuration
Update the api_key variable in src/main.rs with your Gemini API credentials.

Usage Guide
Preparing Input
Create a file named tasks.txt in the root directory with one task per line:

Plaintext

Refactor the authentication logic in the backend
Buy groceries for the weekend
Develop a smart contract for a crowdfunding dApp
Executing the Program
Initiate the categorization process:

Bash

cargo run
Output Format
The application generates batch_results.json in the following format:

JSON

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
Dependency Information
tokio: Multi-threaded runtime for async functions.

reqwest: HTTP client for API communication.

serde / serde_json: Framework for data serialization.

License
This project is licensed under the MIT License. See the LICENSE file for details.