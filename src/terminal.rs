use std::io::{self, stdout, Stdout, Write};

use crossterm::{cursor::{self, Hide, Show}, terminal::ClearType, ExecutableCommand};

pub struct Terminal {
    cursor_x: usize,
    cursor_y: usize,
    width: usize,
    height: usize,
    stdout: Stdout,
}

impl Terminal {
    
    pub fn new() -> io::Result<Self> {
        let size = crossterm::terminal::size()?;

        return Ok(Self { 
            cursor_x: 0,
            cursor_y: 0,
            width: size.0 as usize,
            height: size.1 as usize,
            stdout: stdout()
        })
    }

    pub fn move_cursor(&mut self, x: usize, y: usize) {
        self.cursor_x = x;
        self.cursor_y = y;
        self.stdout.execute(cursor::MoveTo(x as u16, y as u16));
    }

    pub fn get_cursor(&self) -> (usize, usize) {
        let pos =  cursor::position();
        match pos {
            Ok((x, y)) => { return (x as usize, y as usize); },
            Err(_) => todo!(),
        }
    }

    pub fn get_size(&self) -> (usize, usize) {
        return (self.width, self.height);
    }

    pub fn clear(&mut self) {
        let execute_status = self.stdout.execute(crossterm::terminal::Clear(ClearType::All));
        match execute_status {
            Ok(_) => {},
            Err(_) => todo!(),
        }
    }

    pub fn hide(&mut self) {
        self.stdout.execute(Hide);
    }
    pub fn show(&mut self) {
        self.stdout.execute(Show);
    }

    pub fn flush(&mut self) {
        let _ = self.stdout.flush();
    }

}