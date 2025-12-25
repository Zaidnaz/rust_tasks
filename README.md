#  Rust AI Task Categorizer

A high-performance CLI tool built with **Rust** that leverages the **Gemini 2.5 Flash API** to automatically categorize tasks into categories like `Coding`, `Work`, or `Personal`.

## Features
- **Modern Tech Stack**: Built with Rust 1.92+ and the Tokio async runtime.
- **AI Integration**: Uses Google's `gemini-2.5-flash` model for intelligent text classification.
- **Batch Processing**: Reads a list of tasks from a local text file and processes them sequentially.
- **Type Safety**: Utilizes Rust's powerful `struct` and `enum` system for safe JSON serialization/deserialization with `serde`.
- **Error Handling**: Robust handling for API rate limits (429 errors) and network failures.

##  Prerequisites
- **Rust & Cargo**: [Install Rust](https://www.rust-lang.org/tools/install)
- **Google AI Studio API Key**: Get it from [Google AI Studio](https://aistudio.google.com/)

## Installation & Setup

1. **Clone the repository:**
   ```bash
   git clone [https://github.com/Zaidnaz/rust_tasks.git](https://github.com/Zaidnaz/rust_tasks.git)
   cd rust_tasks