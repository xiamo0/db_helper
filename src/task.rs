use crate::database_type::DatabaseType;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub(crate) struct Task {
    #[serde(rename = "taskName")]
    pub task_name: String,
    #[serde(rename = "dbUrl")]
    pub db_url: String,
    #[serde(rename = "dbType")]
    pub db_type: DatabaseType,
    pub input: Vec<String>,
    //反序列化时忽略
    #[serde(skip_deserializing)]
    pub sql_ves: Vec<String>,
}
