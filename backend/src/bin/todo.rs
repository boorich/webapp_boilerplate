use std::env; // Import the env module so we can process cmd line args
use backend::db::{create_task, query_task, establish_connection};

fn help() {
    println!("subcommands:");
    println!("    new<title>: create a new task");
	println!("    show: show all tasks");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 { 				// not enough args
        help();							// print help
        return;
    }

    let subcommand = &args[1];
    match subcommand.as_ref() { 		// Match on the subcommand
        "new" => new_task(&args[2..]),	// create a new task
		"show" => show_tasks(&args[2..]),	// show all tasks
        _ => help(),					// print help
    }
}

fn new_task(args: &[String]) {			// create a new task
    if args.len() < 1 {					// not enough args
        println!("new: missing <title>");
        help();
        return;
    }

    let conn = establish_connection();	// establish a connection to the database
    create_task(&conn, &args[0]);		// create a new task
}

fn show_tasks(args: &[String]) {		// show all tasks
    if args.len() > 0 {					// too many args
        println!("show: unexpected argument");
        help();
        return;
    }

    let conn = establish_connection();	// establish a connection to the database
    println!("TASKS\n-----");			// print the header
    for task in query_task(&conn) {		// query the database
        println!("{}", task.title);		// print the title
    }
}

// Add tests to src/db/tests.rs


