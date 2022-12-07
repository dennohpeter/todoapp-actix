use crate::models::TodoList;
use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;

pub async fn get_todos(client: &Client) -> Result<Vec<TodoList>, std::io::Error> {
    let stmt = client.prepare("SELECT * FROM todo_list").await.unwrap();

    let todos = client
        .query(&stmt, &[])
        .await
        .expect("Error getting todos")
        .iter()
        .map(|row| TodoList::from_row_ref(row).unwrap())
        .collect();

    Ok(todos)
}
