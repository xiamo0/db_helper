use std::fs;
use serde_json::Error;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let task_vec = read_command_file(args);
    if task_vec.is_empty() {
        println!("No task specified");
        return;
    }
    task_vec.iter().for_each(|t| {
        println!("begin task {}", t.task_name);
    })
}

fn read_command_file(args: Vec<String>) -> Vec<Task> {
    if args.len() < 2 {
        println!("Usage: cargo run --release --example sql");
        return vec![];
    }
    let file_path = &args[1];

    let content = fs::read_to_string(file_path);
    if content.is_err() {
        println!("Error reading file: {}", content.unwrap_err());
        return Vec::new();
    }
    let content = content.unwrap();
    let result: Result<Vec<Task>, Error> = serde_json::from_str(&content);
    if result.is_err() {
        println!("Error deserializing file: {}", result.unwrap_err());
        return Vec::new();
    }
    result.unwrap()
}
#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct Task {
    #[serde(rename = "taskName")]
    task_name: String,
    #[serde(rename = "dbUrl")]
    db_url: String,
    output: String,
    #[serde(rename = "taskType")]
    task_type: String,
    input: Vec<String>,
}