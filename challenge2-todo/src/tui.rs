use std::io;
use std::time::Duration;

use challenge2_todo::{Todo, TodoApp};
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Style};
use ratatui::text::Line;
use ratatui::widgets::{Block, Borders, List, ListItem, ListState, Paragraph};
use ratatui::Terminal;

enum Mode {
    // Browse list + run shortcuts.
    Normal,
    // Type into input buffer to create a new task.
    Adding,
}

struct UiState {
    mode: Mode,
    input: String,
    status: String,
    list_state: ListState,
}

impl UiState {
    fn new() -> Self {
        let mut list_state = ListState::default();
        list_state.select(None);

        Self {
            mode: Mode::Normal,
            input: String::new(),
            status: String::from("Ready. Select a task with arrows, then use shortcuts below."),
            list_state,
        }
    }
}

pub fn run(app: &mut TodoApp) -> io::Result<()> {
    // Terminal setup for full-screen TUI.
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let result = run_loop(&mut terminal, app);

    // Always restore terminal state before returning.
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    result
}

fn run_loop(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    app: &mut TodoApp,
) -> io::Result<()> {
    let mut state = UiState::new();

    loop {
        // Snapshot tasks for this frame render.
        let tasks: Vec<Todo> = app.list_tasks().cloned().collect();
        clamp_selection(&mut state, tasks.len());

        terminal.draw(|frame| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(4),
                    Constraint::Min(10),
                    Constraint::Length(3),
                    Constraint::Length(4),
                ])
                .split(frame.area());

            let body_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
                .split(chunks[1]);

            let mode_label = match state.mode {
                Mode::Normal => "Normal",
                Mode::Adding => "Adding",
            };

            let title_text = format!(
                "Todo Queue (Ratatui) | Mode: {mode_label} | Pending: {}",
                tasks.len()
            );
            let title = Paragraph::new(title_text)
                .block(Block::default().borders(Borders::ALL).title("Challenge 2"));
            frame.render_widget(title, chunks[0]);

            let items = if tasks.is_empty() {
                vec![ListItem::new("No pending tasks")]
            } else {
                tasks
                    .iter()
                    .enumerate()
                    .map(|(idx, todo)| {
                        ListItem::new(format!(
                            "{:>2}. #{} [{}] {}",
                            idx + 1,
                            todo.id,
                            todo.created_at,
                            todo.description
                        ))
                    })
                    .collect()
            };

            let list = List::new(items)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title("Pending Tasks (j/k or arrows)"),
                )
                .highlight_style(Style::default().fg(Color::Black).bg(Color::Cyan))
                .highlight_symbol("> ");
            frame.render_stateful_widget(list, body_chunks[0], &mut state.list_state);

            let detail_text = selected_detail_text(&tasks, &state);
            let detail = Paragraph::new(detail_text).block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Selected Task"),
            );
            frame.render_widget(detail, body_chunks[1]);

            let input_title = match state.mode {
                Mode::Normal => "Input (press 'a' to add)",
                Mode::Adding => "Add Task (type and press Enter)",
            };
            let input_text = match state.mode {
                Mode::Normal => String::from(""),
                Mode::Adding => state.input.clone(),
            };
            let input = Paragraph::new(input_text)
                .block(Block::default().borders(Borders::ALL).title(input_title));
            frame.render_widget(input, chunks[2]);

            let help = Paragraph::new(vec![
                Line::from("a: add  d: complete next  x/Delete: delete selected  q: quit"),
                Line::from(format!("Status: {}", state.status)),
            ])
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Help / Status"),
            );
            frame.render_widget(help, chunks[3]);
        })?;

        if event::poll(Duration::from_millis(200))? {
            let Event::Key(key) = event::read()? else {
                continue;
            };

            if key.kind != KeyEventKind::Press {
                continue;
            }

            match state.mode {
                Mode::Normal => {
                    if handle_normal_mode(key.code, &mut state, app)? {
                        break;
                    }
                }
                Mode::Adding => handle_add_mode(key.code, &mut state, app)?,
            }
        }
    }

    Ok(())
}

fn handle_normal_mode(key: KeyCode, state: &mut UiState, app: &mut TodoApp) -> io::Result<bool> {
    match key {
        KeyCode::Char('q') => return Ok(true),
        KeyCode::Char('a') => {
            state.mode = Mode::Adding;
            state.input.clear();
            state.status = String::from("Adding mode: type task and press Enter. Esc cancels.");
        }
        KeyCode::Char('d') => match app.complete_next()? {
            Some(todo) => {
                state.status = format!("Completed task #{}: {}", todo.id, todo.description);
            }
            None => {
                state.status = String::from("No tasks to complete.");
            }
        },
        KeyCode::Delete | KeyCode::Char('x') => {
            // Delete the highlighted item by logical queue index.
            if let Some(index) = state.list_state.selected() {
                match app.delete_at(index)? {
                    Some(todo) => {
                        state.status = format!("Deleted task #{}: {}", todo.id, todo.description);
                    }
                    None => {
                        state.status = String::from("No selected task to delete.");
                    }
                }
            } else {
                state.status = String::from("No selected task to delete.");
            }
        }
        KeyCode::Up | KeyCode::Char('k') => {
            let current = state.list_state.selected().unwrap_or(0);
            state.list_state.select(Some(current.saturating_sub(1)));
        }
        KeyCode::Down | KeyCode::Char('j') => {
            let current = state.list_state.selected().unwrap_or(0);
            state.list_state.select(Some(current.saturating_add(1)));
        }
        _ => {}
    }

    Ok(false)
}

fn handle_add_mode(key: KeyCode, state: &mut UiState, app: &mut TodoApp) -> io::Result<()> {
    match key {
        KeyCode::Esc => {
            state.mode = Mode::Normal;
            state.input.clear();
            state.status = String::from("Add cancelled.");
        }
        KeyCode::Enter => {
            let value = state.input.trim();
            if value.is_empty() {
                state.status = String::from("Task description cannot be empty.");
            } else {
                // Commit typed value into queue + persistence layer.
                let todo = app.add_task(value.to_string())?;
                state.status = format!("Added task #{}: {}", todo.id, todo.description);
            }
            state.input.clear();
            state.mode = Mode::Normal;
        }
        KeyCode::Backspace => {
            state.input.pop();
        }
        KeyCode::Char(ch) => {
            state.input.push(ch);
        }
        _ => {}
    }

    Ok(())
}

fn clamp_selection(state: &mut UiState, len: usize) {
    if len == 0 {
        // Nothing to highlight when list is empty.
        state.list_state.select(None);
        return;
    }

    let current = state.list_state.selected().unwrap_or(0);
    let max_index = len.saturating_sub(1);
    state.list_state.select(Some(current.min(max_index)));
}

fn selected_detail_text(tasks: &[Todo], state: &UiState) -> String {
    if tasks.is_empty() {
        return String::from("No task selected");
    }

    // Derive a safe index from current cursor position.
    let index = state
        .list_state
        .selected()
        .unwrap_or(0)
        .min(tasks.len() - 1);
    let todo = &tasks[index];
    format!(
        "Index: {}\nID: {}\nCreated: {}\n\n{}",
        index + 1,
        todo.id,
        todo.created_at,
        todo.description
    )
}
