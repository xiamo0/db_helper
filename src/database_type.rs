use std::fmt;
use serde::{Deserialize, Deserializer};

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
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