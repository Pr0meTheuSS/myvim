use clap::Parser;
use crossterm::{
    ExecutableCommand, cursor,
    event::{Event, read},
    terminal::{ClearType, disable_raw_mode, enable_raw_mode},
};
use std::io::stdout;

mod editor;
mod mode_manager;
mod state_machine;

use editor::Editor;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    #[arg(short, long)]
    file: String,
}

use std::fs::File;
use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

use std::io::{self, Write};

fn write_buffer_to_file(buffer: &Vec<Vec<char>>, path: &str) -> io::Result<()> {
    let mut file = File::create(path)?;

    for line in buffer {
        let line_string: String = line.iter().collect();
        writeln!(file, "{}", line_string)?;
    }

    Ok(())
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let contents = read_lines(&args.file);

    enable_raw_mode()?;
    let mut stdout = stdout();
    let mut editor = Editor::from_strings(contents);

    stdout.execute(crossterm::terminal::Clear(ClearType::All))?;
    stdout.execute(cursor::MoveTo(0, 0))?;

    loop {
        stdout.execute(crossterm::terminal::Clear(ClearType::All))?;
        let content = editor.buffer.iter().enumerate();
        for (y, line) in content {
            stdout.execute(cursor::MoveTo(0, y as u16))?;
            for c in line {
                print!("{}", c);
            }
        }

        stdout.execute(cursor::MoveTo(
            editor.get_cursor_x() as u16,
            editor.get_cursor_y() as u16,
        ))?;
        stdout.flush()?;

        if let Event::Key(event) = read()? {
            editor.handle_key(event.code);
        }
        if editor.is_exiting() {
            break;
        }
    }
    write_buffer_to_file(&editor.get_content(), &args.file)?;
    disable_raw_mode()?;

    Ok(())
}

#[test]
fn test_read_lines() {
    let temp_file = "test_read_lines.txt";
    let mut file = File::create(temp_file).unwrap();
    writeln!(file, "Line 1").unwrap();
    writeln!(file, "Line 2").unwrap();

    let lines = read_lines(temp_file);
    assert_eq!(lines, vec!["Line 1", "Line 2"]);

    std::fs::remove_file(temp_file).unwrap();
}

#[test]
fn test_write_buffer_to_file() {
    let buffer = vec![vec!['H', 'e', 'l', 'l', 'o'], vec!['W', 'o', 'r', 'l', 'd']];

    let temp_file = "test_write_buffer.txt";
    write_buffer_to_file(&buffer, temp_file).unwrap();

    let content = std::fs::read_to_string(temp_file).unwrap();
    assert!(content.contains("Hello"));
    assert!(content.contains("World"));

    std::fs::remove_file(temp_file).unwrap();
}
