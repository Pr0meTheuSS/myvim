use clap::Parser;
use crossterm::{
    event::{poll, read, Event},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use terminal::Terminal;

mod editor;
mod mode_manager;
mod state_machine;
mod terminal;

use editor::Editor;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    #[arg(short, long)]
    file: String,
}

use std::{fs::File, time::Duration};
use std::fs::read_to_string;
use std::io::{self, Write};

fn read_lines(filename: &str) -> Vec<String> {
    let content = match read_to_string(filename) {
        Ok(content) => content,
        Err(_) => {
            File::create(filename).expect("Create new file.");
            String::new()
        }
    };

    content.lines().map(String::from).collect()
}

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
    let mut editor = Editor::from_strings(contents);
    let mut terminal = Terminal::new()?;
    terminal.clear();

    let mut offset = 0;

    loop {
        // Обработка ввода
        if poll(Duration::from_millis(16))? {
            if let Event::Key(event) = read()? {
                editor.handle_key(event.code);
            }
        }

        if editor.is_exiting() {
            break;
        }

        let (terminal_width, terminal_height) = terminal.get_size();
        let height = terminal_height as usize;

        let (cursor_x, cursor_y) = editor.get_cursor();

        // Обновляем offset, если курсор выходит за экран
        if cursor_y < offset {
            offset = cursor_y;
        } else if cursor_y >= offset + height {
            offset = cursor_y - height + 1;
        }

        // Получаем контент
        let mut content = editor.get_content();

        // Добавляем статусную строку
        let status_string = format!("x: {}, y: {} offset: {}", cursor_x, cursor_y, offset);
        let status_line = status_string.chars().collect();
        content.push(status_line);

        // Отрисовка
        terminal.hide();
        for i in 0..height {
            terminal.move_cursor(0, i);
            if let Some(line) = content.get(offset + i) {
                let display_line: String = line.iter().collect();
                print!("{:width$}", display_line, width = terminal_width as usize);
            } else {
                print!("{:width$}", "", width = terminal_width as usize);
            }
        }
        terminal.show();

        // Перемещаем курсор (на экране — с учетом offset)
        terminal.move_cursor(cursor_x, cursor_y - offset);
        terminal.flush();
    }

    write_buffer_to_file(&editor.get_content(), &args.file)?;
    disable_raw_mode()?;

    Ok(())
}
