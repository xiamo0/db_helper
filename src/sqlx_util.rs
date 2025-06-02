/*
使用SQLx 操作数据库
 */
use crate::database_type::DatabaseType;
use crate::task::Task;
use sqlx::mysql::MySqlPoolOptions;

#[allow(dead_code)]
pub fn run(task_vec: &[Task]) -> Result<(), String> {
    for task in task_vec {
        match task.db_type {
            DatabaseType::MySQL => {
                // let mysql = operate_mysql(task);
            }
            _ => {
                println!("{}", task.db_type);
            }
        }
    }
    Ok(())
}
pub async fn operate_mysql(task: &Task) -> Result<(), sqlx::Error> {
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(task.db_url.as_str())
        .await?;
    for x in &task.sql_ves {
        let result = sqlx::query(x.as_str()).execute(&pool).await?;
        let i = result.rows_affected();
    }
    Ok(())
}
