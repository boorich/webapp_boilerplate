use diesel::{prelude::*, sqlite::SqliteConnection};                 // Import the diesel prelude and sqlite connection

pub mod models;
pub mod schema;

pub fn establish_connection() -> SqliteConnection {                 // establish a connection to the database
    let db = "./testdb.sqlite3";                                    // the path to the database
    SqliteConnection::establish(db)                                 // connect to the database
        .unwrap_or_else(|_| panic!("Error connecting to {}", db))   // panic if the connection failed
}

pub fn create_task(connection: &SqliteConnection, title: &str) {    // create a new task
    let task = models::NewTask { title };

    diesel::insert_into(schema::task::table)                        // insert the task into the database
        .values(&task)                                              // use the task
        .execute(connection)                                        // execute the query
        .expect("Error inserting new task");                        // panic if the query failed
}

pub fn query_task(connection: &SqliteConnection) -> Vec<models::Task> { // query the database
    schema::task::table                                                 // select the task table
        .load::<models::Task>(connection)                               // load the tasks
        .expect("Error loading tasks")                                  // panic if the query failed
}