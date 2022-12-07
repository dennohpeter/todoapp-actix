use tokio_pg_mapper_derive::PostgresMapper;

#[derive(serde::Serialize)]
pub struct Status {
    pub status: String,
}

#[derive(serde::Serialize, serde::Deserialize, PostgresMapper)]
#[pg_mapper(table = "todo_list")]
pub struct TodoList {
    pub id: i32,
    pub title: String,
}

#[derive(serde::Deserialize, serde::Serialize, PostgresMapper)]
#[pg_mapper(table = "todo_item")]
pub struct TodoItem {
    pub id: i32,
    pub title: String,
    pub completed: bool,
    pub list_id: i32,
}
