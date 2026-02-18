use challenge2_todo::TodoApp;
use std::{
    env, fs,
    time::{SystemTime, UNIX_EPOCH},
};

fn unique_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |duration| duration.as_secs())
}

#[test]
fn todo_persists_between_restarts() {
    let file_path = env::temp_dir().join(format!("challenge2_todos_{}.bin", unique_timestamp()));

    let mut app = TodoApp::load_or_new(&file_path).expect("load should succeed");
    app.add_task("Buy groceries".to_string())
        .expect("first add should succeed");
    app.add_task("Pay bills".to_string())
        .expect("second add should succeed");

    let reloaded = TodoApp::load_or_new(&file_path).expect("reload should succeed");
    let descriptions: Vec<&str> = reloaded
        .list_tasks()
        .map(|todo| todo.description.as_str())
        .collect();

    assert_eq!(descriptions, vec!["Buy groceries", "Pay bills"]);

    let _ = fs::remove_file(file_path);
}
