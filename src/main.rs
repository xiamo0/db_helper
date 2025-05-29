use std::{fmt, fs};
use serde::{Deserialize, Deserializer};
use serde_json::Error;
use sqlparser::dialect::GenericDialect;
use sqlparser::parser::Parser;

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
fn parse_sql(db_type :&String,sql: &String)->bool {


    let dialect=GenericDialect{};
    let result = Parser::parse_sql(&dialect, sql);
    match result {
        Ok(statements) => {true}
        Err(_) => {false}
    }

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
    #[serde(rename = "dbType")]
    db_type:DatabaseType,
    input: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DatabaseType {
    MySQL,
    PostgreSQL,
    SQLite,
    MSSQL,
    Oracle,
    MariaDB,
}

impl<'de> Deserialize<'de> for DatabaseType {
    fn deserialize<D>(deserializer: D) -> Result<DatabaseType, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.to_lowercase().as_str() {
            "mysql" => Ok(DatabaseType::MySQL),
            "postgresql" | "postgres" => Ok(DatabaseType::PostgreSQL),
            "sqlite" => Ok(DatabaseType::SQLite),
            "mssql" | "sqlserver" => Ok(DatabaseType::MSSQL),
            "oracle" => Ok(DatabaseType::Oracle),
            "mariadb" => Ok(DatabaseType::MariaDB),
            _ => Err(serde::de::Error::custom(format!("未知的数据库类型: {}", s))),
        }
    }
}

impl fmt::Display for DatabaseType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            DatabaseType::MySQL => "MySQL",
            DatabaseType::PostgreSQL => "PostgreSQL",
            DatabaseType::SQLite => "SQLite",
            DatabaseType::MSSQL => "MSSQL",
            DatabaseType::Oracle => "Oracle",
            DatabaseType::MariaDB => "MariaDB",
        })
    }
}