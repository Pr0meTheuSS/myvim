use std::io::{stdout, Write};
use crossterm::{
    cursor,
    event::{read, Event, KeyCode},
    terminal::{enable_raw_mode, disable_raw_mode, ClearType},
    ExecutableCommand,
};

fn main() -> std::io::Result<()>  {
    enable_raw_mode()?;

    let mut stdout = stdout();
    let mut buffer: Vec<Vec<char>> = vec![vec![]];
    let mut cursor_x = 0;
    let mut cursor_y = 0;

    stdout.execute(crossterm::terminal::Clear(ClearType::All))?;
    stdout.execute(cursor::MoveTo(0, 0))?;
    println!("Enter 'q' to exit...");
    

    loop {
        stdout.execute(cursor::MoveTo(cursor_x as u16, cursor_y as u16));
        stdout.flush()?;
        
        if let Event::Key(event) = read()? {
            match event.code {
                KeyCode::Char('q') => break,
                KeyCode::Char(c) => {
                    buffer[cursor_y].insert(cursor_x, c);
                    cursor_x+=1;
                },
                _ => todo!()
            }
        }

        stdout.execute(crossterm::terminal::Clear(ClearType::All))?;
        for (y, line) in buffer.iter().enumerate() {
            stdout.execute(cursor::MoveTo(0, y as u16))?;
            for c in line {
                print!("{}", c);
            }
        }
    }

    disable_raw_mode()?;
    Ok(())
}

