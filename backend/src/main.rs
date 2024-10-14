#[macro_use] extern crate rocket;

use mongodb::{Client, options::ClientOptions, bson::{doc, oid::ObjectId}, results::InsertOneResult};
use rocket::serde::{Serialize, Deserialize, json::Json};
use rocket::tokio;
use std::sync::Arc;
use rocket::State;
use mongodb::bson;

#[derive(Debug, Serialize, Deserialize)]
struct Todo {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    task: String,
    is_complete: bool,
}

struct MongoRepo {
    client: mongodb::Client,
}

#[post("/add", format = "json", data = "<todo>")]
async fn add(todo: Json<Todo>, db: &State<Arc<MongoRepo>>) -> Json<InsertOneResult> {
    let collection = db.client.database("todo_db").collection::<Todo>("todos");
    let new_todo = Todo {
        id: None,
        task: todo.task.clone(),
        is_complete: todo.is_complete,
    };
    let insert_result = collection.insert_one(new_todo, None).await.unwrap();
    Json(insert_result)
}

#[get("/list")]
async fn list(db: &State<Arc<MongoRepo>>) -> Json<Vec<Todo>> {
    let collection = db.client.database("todo_db").collection::<Todo>("todos");
    let mut cursor = collection.find(None, None).await.unwrap();
    let mut todos = vec![];
    while let Some(todo) = cursor.try_next().await.unwrap() {
        todos.push(todo);
    }
    Json(todos)
}

#[launch]
async fn rocket() -> _ {
    let client_options = ClientOptions::parse("mongodb://mongodb:27017").await.unwrap();
    let client = Client::with_options(client_options).unwrap();
    let db = Arc::new(MongoRepo { client });

    rocket::build()
        .manage(db)
        .mount("/", routes![add, list])
}
