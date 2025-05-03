use crossterm::{
    ExecutableCommand, cursor,
    event::{Event, KeyCode, read},
    terminal::{ClearType, disable_raw_mode, enable_raw_mode},
};
use std::io::{Write, stdout};

mod editor;
use editor::Editor;

fn main() -> std::io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = stdout();
    let mut editor = Editor::new();

    stdout.execute(crossterm::terminal::Clear(ClearType::All))?;
    stdout.execute(cursor::MoveTo(0, 0))?;
    println!("Enter 'Esc' to exit...");

    loop {
        stdout.execute(cursor::MoveTo(editor.get_cursor_x() as u16, editor.get_cursor_y() as u16))?;
        stdout.flush()?;

        if let Event::Key(event) = read()? {
            match event.code {
                KeyCode::Esc => break,
                KeyCode::Char(c) => editor.insert_char(c),
                KeyCode::Backspace => editor.backspace(),
                KeyCode::Enter => editor.enter(),
                KeyCode::Up => editor.move_cursor_up(),
                KeyCode::Down => editor.move_cursor_down(),
                KeyCode::Left => editor.move_cursor_left(),
                KeyCode::Right => editor.move_cursor_right(),
                _ => {}
            }
        }

        stdout.execute(crossterm::terminal::Clear(ClearType::All))?;
        for (y, line) in editor.buffer.iter().enumerate() {
            stdout.execute(cursor::MoveTo(0, y as u16))?;
            for c in line {
                print!("{}", c);
            }
        }
    }

    disable_raw_mode()?;
    Ok(())
}

