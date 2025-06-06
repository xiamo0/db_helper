/*
处理命令参数
 */
use crate::database_type::DatabaseType;
use crate::task::Task;
use serde_json::Error;
use sqlparser::dialect::{Dialect, GenericDialect, MySqlDialect, PostgreSqlDialect};
use sqlparser::parser::Parser;
use std::fs;
use sqlparser::ast::Statement;

pub fn run() -> Result<Vec<Task>, String> {
    let args = std::env::args().collect::<Vec<String>>();
    if args.is_empty() {
        return Err("No arguments supplied".to_string());
    }
    if args.is_empty() {
        return Err("No arguments supplied".to_string());
    }
    let task_vec = read_command_file(&args);
    if task_vec.is_empty() {
        return Err("No task specified".to_string());
    }

    let task_vec = read_sql_file(&task_vec)?;

    for task in &task_vec {
        let database_type = &task.db_type;
        let is_ok = parse_sql(&database_type, &task.sql_ves);
        match is_ok {
            Ok(_) => {}
            Err(e) => {
                return Err(e);
            }
        }
        println!("sql:{:?}", &task.sql_ves)
    }
    Ok(task_vec)
}

pub fn read_command_file(args: &[String]) -> Vec<Task> {
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

pub fn parse_sql(db_type: &DatabaseType, sql_vec: &[String]) -> Result<(), String> {
    let dialect: Box<dyn Dialect> = match db_type {
        DatabaseType::MySQL => Box::new(MySqlDialect {}),
        DatabaseType::PostgreSQL => Box::new(PostgreSqlDialect {}),
        _ => Box::new(GenericDialect {}),
    };
    for sql in sql_vec {
        let result = Parser::parse_sql(&*dialect, sql);
        match result {
            Ok(_) => {}
            Err(e) => {
                return Err(format!("{},{}", sql, e));
            }
        }
    }
    Ok(())
}


pub fn parse_sql_file_to_vec(file_path: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    // 1. 读取文件内容
    let sql_content = fs::read_to_string(file_path)?;

    // 2. 使用 GenericDialect 解析（可替换为 MySqlDialect 等）
    let dialect = GenericDialect {};
    let statements = Parser::parse_sql(&dialect, &sql_content)?;

    // 3. 将解析后的 AST 转为 SQL 字符串
    let sql_vec = statements
        .into_iter()
        .map(|stmt: Statement| stmt.to_string())
        .collect::<Vec<_>>();

    Ok(sql_vec)
}

pub fn get_sql_vec(input: &[String]) -> Vec<String> {
    let mut sql_vec: Vec<String> = Vec::new();
    for x in input {
        if x.to_lowercase().ends_with(".sql") {
            let result1 = parse_sql_file_to_vec(x);
            if result1.is_err() {
                panic!("Error parsing file: {},{}", x, &result1.unwrap_err());
            } else {
                let vec = result1.unwrap();
                sql_vec.extend(vec);
            }
        } else {
            sql_vec.push(x.clone());
        }
    }
    sql_vec
}

pub fn read_sql_file(task: &[Task]) -> Result<Vec<Task>, String> {
    if task.is_empty() {
        return Err("No Task".to_string());
    }
    let new_task_vec: Vec<Task> = task
        .iter()
        .map(|t| Task {
            task_name: t.task_name.clone(),
            db_url: t.db_url.clone(),
            db_type: t.db_type.clone(),
            input: vec![],
            sql_ves: get_sql_vec(&t.input),
        })
        .collect();
    Ok(new_task_vec)
}
