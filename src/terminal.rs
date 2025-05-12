use std::io::{self, stdout, Stdout, Write};

use crossterm::{cursor, terminal::ClearType, ExecutableCommand};

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
        return (self.cursor_x, self.cursor_y);
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

    pub fn scroll_down(&mut self, offset: i64) {
        self.stdout.execute(crossterm::terminal::ScrollDown(offset.abs() as u16));
    }

    pub fn scroll_up(&mut self, offset: i64) {
        self.stdout.execute(crossterm::terminal::ScrollUp(offset.abs() as u16));
    }

    pub fn flush(&mut self) {
        let _ = self.stdout.flush();
    }

}