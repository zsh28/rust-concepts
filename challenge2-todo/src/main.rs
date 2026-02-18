use std::env;

use challenge2_todo::TodoApp;

/// CLI entry point for the persistent todo queue app.
fn main() {
    let mut args = env::args().skip(1);
    let command = match args.next() {
        Some(cmd) => cmd,
        None => {
            print_usage();
            return;
        }
    };

    let mut app = match TodoApp::load_or_new("todos.bin") {
        Ok(app) => app,
        Err(err) => {
            eprintln!("Failed to load todo queue: {err}");
            std::process::exit(1);
        }
    };

    match command.as_str() {
        "add" => {
            let description = args.collect::<Vec<String>>().join(" ");
            if description.is_empty() {
                eprintln!("Usage: todo add \"task description\"");
                std::process::exit(1);
            }

            match app.add_task(description) {
                Ok(todo) => println!("Added task #{}: {}", todo.id, todo.description),
                Err(err) => {
                    eprintln!("Failed to add task: {err}");
                    std::process::exit(1);
                }
            }
        }
        "list" => {
            if app.len() == 0 {
                println!("No pending tasks.");
                return;
            }

            for todo in app.list_tasks() {
                println!("#{} [{}] {}", todo.id, todo.created_at, todo.description);
            }
        }
        "done" => match app.complete_next() {
            Ok(Some(todo)) => println!("Completed task #{}: {}", todo.id, todo.description),
            Ok(None) => println!("No tasks to complete."),
            Err(err) => {
                eprintln!("Failed to complete task: {err}");
                std::process::exit(1);
            }
        },
        _ => {
            eprintln!("Unknown command: {command}");
            print_usage();
            std::process::exit(1);
        }
    }
}

fn print_usage() {
    println!("Todo CLI");
    println!("  todo add \"Buy groceries\"");
    println!("  todo list");
    println!("  todo done");
}
