// i want to build a simple todo cli app and have no idea what im doing.
// ctrl shift i to disable ctrl shift e to enable
// heres what i think i need to do:
// need struct to create todos
// need impl funcs to add, remove, list todos.
// todos should have unique ids for easy removal or updating (ltr on)
// i want to implement enums for todo status (e.g., Pending, Completed)

// still learning rust, dont judge me pls

#[derive(Debug)]

enum Status{
    Pending,
    Completed,
}

struct ToDo {
    note: String,
    timestamp: String,
    id: i32,
    status: Status, // either Pending or Completed
}

impl ToDo {
    fn add_todo(todos: &mut Vec<ToDo>, note: String, timestamp: String, id: i32, status: Status) {
        todos.push(ToDo { note, timestamp, id, status });
    }

    fn update_status(todos: &mut Vec<ToDo>, id: i32, new_status: Status) {
        for todo in todos.iter_mut() {
            if todo.id == id {
                todo.status = new_status; // errpr here...
            }
        }
    }

    fn remove_todo(todos: &mut Vec<ToDo>, id: i32) {
        todos.retain(|todo| todo.id != id);
    }

    fn list_todos(todos: &Vec<ToDo>) {
        for todo in todos {
            println!("{}: {} [{}] - {:?}", todo.id, todo.note, todo.timestamp, todo.status);
        }
    }
}

fn main() {
    let mut todos: Vec<ToDo> = vec![];

    ToDo::add_todo(&mut todos, "Learn Rust".to_string(), "2026-01-16".to_string(), 1, Status::Pending);
    ToDo::add_todo(&mut todos, "Build CLI app".to_string(), "2026-01-16".to_string(), 2, Status::Pending);
    println!("All todos:");
    ToDo::list_todos(&todos);

    ToDo::remove_todo(&mut todos, 1);

    println!("After removal:");
    ToDo::list_todos(&todos);

    ToDo::update_status(&mut todos, 2, Status::Completed);
    println!("After updating status:");
    ToDo::list_todos(&todos);
}
