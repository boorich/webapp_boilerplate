#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde;


use rocket_contrib::json::Json;
use mytodo::db::models::Task;
use mytodo::db::{query_task, establish_connection};

#[derive(Serialize)] 						// Serialize the struct
struct JsonApiResponse { 					// Create a struct to hold the response
    data: Vec<Task>, 						// Hold the data
}

#[get("/tasks")]							// Define the route
fn tasks_get() -> Json<JsonApiResponse> {	// Return the response
    let mut response = JsonApiResponse { data: vec![], };	// Create a new response

    let conn = establish_connection();		// Establish a connection to the database
    for task in query_task(&conn) {			// Query the database
        response.data.push(task);			// Add the task to the response
    }

    Json(response)							// Return the response
}


fn main() {
    rocket::ignite()						// Start the rocket server
        .mount("/", routes![tasks_get])		// Mount the routes
        .launch();							// Launch the server
}

// Add tests to src/db/tests.rs
