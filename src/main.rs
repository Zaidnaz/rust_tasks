use serde::{Serialize, Deserialize};
use reqwest::Client;
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    text: String,
    category: String,
}

// ... (Keep your GeminiResponse, Candidate, Content, etc. structs from before) ...

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = "YOUR_GEMINI_API_KEY"; 
    let client = Client::new();
    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.5-flash:generateContent?key={}",
        api_key
    );

    // 1. Read the file and split into lines
    let content = fs::read_to_string("tasks.txt").expect("Could not read tasks.txt");
    let mut final_tasks = Vec::new();

    println!("🚀 Starting Batch Processing...");

    for line in content.lines() {
        if line.trim().is_empty() { continue; }

        println!("🤖 Processing: '{}'", line);

        let prompt = format!(
            "Categorize this into one word: 'Work', 'Personal', or 'Coding'. Task: '{}'. Response should be only the word.",
            line
        );

        let request_body = serde_json::json!({
            "contents": [{ "parts": [{ "text": prompt }] }]
        });

        // 2. Make the API call
        let res = client.post(&url).json(&request_body).send().await?;
        let gemini_data: GeminiResponse = res.json().await?;

        if let Some(candidates) = gemini_data.candidates {
            let ai_category = candidates[0].content.parts[0].text.trim().to_string();
            
            final_tasks.push(Task {
                text: line.to_string(),
                category: ai_category,
            });
        }
        
        // Small sleep to avoid hitting the 429 rate limit again
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    }

    // 3. Save the entire list as a JSON array
    let final_json = serde_json::to_string_pretty(&final_tasks)?;
    fs::write("batch_results.json", final_json)?;

    println!("\n✅ All tasks processed! Check 'batch_results.json'.");
    Ok(())
}