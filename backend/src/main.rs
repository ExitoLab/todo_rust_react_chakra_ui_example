use warp::Filter;
use mongodb::{Client, options::ClientOptions, bson::{doc, oid::ObjectId}};
use serde::{Serialize, Deserialize};
use std::sync::Arc;
use warp::reject::{self, Reject};
use warp::http::StatusCode;
use futures_util::stream::StreamExt;

// Struct to hold error types for custom rejections
#[derive(Debug)]
struct DatabaseError;
impl Reject for DatabaseError {}

// Struct to represent a Todo item
#[derive(Debug, Serialize, Deserialize)]
struct Todo {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    task: String,
    is_complete: bool,
}

// Struct to hold the MongoDB client
struct MongoRepo {
    client: mongodb::Client,
}

// Health check route handler
async fn health_check() -> Result<impl warp::Reply, warp::Rejection> {
    Ok(warp::reply::with_status("Server is healthy", StatusCode::OK))
}

// Add a new Todo to the database
async fn add(todo: Todo, db: Arc<MongoRepo>) -> Result<impl warp::Reply, warp::Rejection> {
    let collection = db.client.database("todo_db").collection::<Todo>("todos");
    let new_todo = Todo {
        id: None,
        task: todo.task,
        is_complete: todo.is_complete,
    };

    match collection.insert_one(new_todo, None).await {
        Ok(insert_result) => Ok(warp::reply::with_status(
            warp::reply::json(&insert_result),
            StatusCode::CREATED,
        )),
        Err(_) => Err(reject::custom(DatabaseError)),  // Custom rejection for db errors
    }
}

// List all Todos in the database
async fn list(db: Arc<MongoRepo>) -> Result<impl warp::Reply, warp::Rejection> {
    let collection = db.client.database("todo_db").collection::<Todo>("todos");

    let mut todos = vec![];
    let mut cursor = match collection.find(None, None).await {
        Ok(cursor) => cursor,
        Err(_) => return Err(reject::custom(DatabaseError)), // Handle db find error
    };

    while let Some(todo) = cursor.next().await {
        if let Ok(todo) = todo {
            todos.push(todo);
        }
    }

    Ok(warp::reply::json(&todos))
}

// Function to handle rejections (including DatabaseError)
async fn handle_rejection(err: warp::Rejection) -> Result<impl warp::Reply, warp::Rejection> {
    if let Some(_) = err.find::<DatabaseError>() {
        Ok(warp::reply::with_status(
            "Internal Server Error".to_string(),
            StatusCode::INTERNAL_SERVER_ERROR,
        ))
    } else {
        // If the error is not a DatabaseError, return a generic 404 error
        Ok(warp::reply::with_status(
            "Not Found".to_string(),
            StatusCode::NOT_FOUND,
        ))
    }
}

#[tokio::main]
async fn main() {
    // Set up MongoDB client
    let client_options = ClientOptions::parse("mongodb://mongodb:27017").await.unwrap();
    let client = Client::with_options(client_options).unwrap();
    let db = Arc::new(MongoRepo { client });

    // Define filters for routing
    let db_filter = warp::any().map(move || db.clone());

    // Health check route
    let health_route = warp::path("health")
        .and(warp::get())
        .and_then(health_check);

    // Add new Todo route
    let add_route = warp::path("add")
        .and(warp::post())
        .and(warp::body::json())
        .and(db_filter.clone())
        .and_then(add);

    // List all Todos route
    let list_route = warp::path("list")
        .and(warp::get())
        .and(db_filter.clone())
        .and_then(list);

    // Combine all routes and set up error handling
    let routes = health_route
        .or(add_route)
        .or(list_route)
        .recover(handle_rejection);

    // Set up CORS
    let cors = warp::cors()
        .allow_any_origin() // Allow requests from any origin
        .allow_methods(vec!["GET", "POST"]) // Allow GET and POST methods
        .allow_headers(vec!["content-type"]); // Allow the Content-Type header

    // Start the Warp server with CORS enabled
    warp::serve(routes.with(cors))
        .run(([0, 0, 0, 0], 8000))
        .await;
}
