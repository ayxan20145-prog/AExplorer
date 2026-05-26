use colored::Colorize;
use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{self, Event, KeyCode},
    execute,
    style::Print,
    terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode},
};
use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;
use std::time::Duration;

fn main() -> io::Result<()> {
    enable_raw_mode()?;

    let mut stdout = io::stdout();
    execute!(stdout, Hide, Clear(ClearType::All))?;

    let mut selected: usize = 0;

    let mut dir = env::current_dir()?;

    loop {
        let mut entries_list: Vec<(PathBuf, bool)> = Vec::new();

        for entry in fs::read_dir(&dir)? {
            if let Ok(entry) = entry {
                let path = entry.path();
                let is_dir = path.is_dir();
                entries_list.push((path, is_dir));
            }
        }

        if selected >= entries_list.len() && !entries_list.is_empty() {
            selected = entries_list.len() - 1;
        }

        execute!(stdout, MoveTo(0, 0), Clear(ClearType::All))?;

        execute!(stdout, Print(format!("{}\r\n", dir.display())))?;

        for (i, (path, is_dir)) in entries_list.iter().enumerate() {
            let name = path.file_name().unwrap_or_default().to_string_lossy();

            if i == selected {
                if *is_dir {
                    execute!(stdout, Print(format!("> {}/\r\n", name.blue())))?;
                } else {
                    execute!(stdout, Print(format!("> {}\r\n", name)))?;
                }
            } else {
                if *is_dir {
                    execute!(stdout, Print(format!("  {}/\r\n", name.blue())))?;
                } else {
                    execute!(stdout, Print(format!("  {}\r\n", name)))?;
                }
            }
        }

        stdout.flush()?;

        if event::poll(Duration::from_millis(200))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('j') => {
                        if selected + 1 < entries_list.len() {
                            selected += 1;
                        }
                    }
                    KeyCode::Char('k') => {
                        if selected > 0 {
                            selected -= 1;
                        }
                    }
                    KeyCode::Char('h') => {
                        dir.pop();
                    }
                    KeyCode::Char('l') => {
                        if let Some((path, is_dir)) = entries_list.get(selected) {
                            if *is_dir {
                                dir = path.clone();
                                selected = 0;
                            }
                        }
                    }
                    KeyCode::Char('q') => break,
                    _ => {}
                }
            }
        }
    }

    execute!(stdout, Show)?;
    disable_raw_mode()?;
    Ok(())
}
