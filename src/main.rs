use actix_web::{ get, web, App, HttpServer };
use serde::{ Deserialize, Serialize };
use std::sync::Mutex;


struct AppState {
    todolist_entries: Mutex<Vec<TodoListEntry>>
}

#[derive(Serialize, Deserialize, Clone)]
struct TodoListEntry {
    id: i32,
    date: i64,
    title: String,
}

#[get("/")]
async fn index() -> String {
    "This is a health check".to_string()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_data = web::Data::new(AppState {
        todolist_entries: Mutex::new(vec![])
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .service(index)
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}