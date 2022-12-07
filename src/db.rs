use crate::models::{TodoItem, TodoList};
use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;

pub async fn get_todos(client: &Client) -> Result<Vec<TodoList>, std::io::Error> {
    let stmt = client
        .prepare("SELECT * FROM todo_list order by id desc")
        .await
        .unwrap();

    let todos = client
        .query(&stmt, &[])
        .await
        .expect("Error getting todos")
        .iter()
        .map(|row| TodoList::from_row_ref(row).unwrap())
        .collect();

    Ok(todos)
}

pub async fn get_items(client: &Client, list_id: i32) -> Result<Vec<TodoItem>, std::io::Error> {
    let stmt = client
        .prepare("SELECT * FROM todo_item where list_id = $1 order by id")
        .await
        .unwrap();

    let items = client
        .query(&stmt, &[&list_id])
        .await
        .expect("Error getting items")
        .iter()
        .map(|row| TodoItem::from_row_ref(row).unwrap())
        .collect();

    Ok(items)
}

pub async fn create_todo(client: &Client, title: String) -> Result<TodoList, std::io::Error> {
    let stmt = client
        .prepare("INSERT INTO todo_list (title) VALUES ($1) returning id, title")
        .await
        .unwrap();

    client
        .query(&stmt, &[&title])
        .await
        .expect("Error creating todo list")
        .iter()
        .map(|row| TodoList::from_row_ref(row).unwrap())
        .collect::<Vec<TodoList>>()
        .pop()
        .ok_or(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Error creating todo list",
        ))
}

pub async fn check_item(client: &Client, list_id: i32, item_id: i32) -> Result<(), std::io::Error> {
    let stmt = client
        .prepare("UPDATE todo_item SET completed = true WHERE list_id = $1 AND id = $2 and completed = false")
        .await
        .unwrap();

    let result = client
        .execute(&stmt, &[&list_id, &item_id])
        .await
        .expect("Error checking todo item");

    match result {
        ref updated if *updated == 1 => Ok(()),
        _ => Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Failed to check todo item",
        )),
    }
}
