use crossterm::event::KeyCode;

use crate::mode_manager::{ModeManager, State};

pub struct Editor {
    pub buffer: Vec<Vec<char>>,
    pub cursor_x: usize,
    pub cursor_y: usize,
    is_exiting: bool,
    mode_manager: ModeManager,
}

fn strings_to_char_buffer(lines: Vec<String>) -> Vec<Vec<char>> {
    lines
        .into_iter()
        .map(|line| line.chars().collect())
        .collect()
}

impl Editor {
    pub fn is_exiting(&self) -> bool {
        return self.is_exiting;
    }

    pub fn from_strings(content: Vec<String>) -> Editor {
        let mut editor = Editor {
            buffer: vec![vec![]],
            cursor_x: 0,
            cursor_y: 0,
            mode_manager: ModeManager::new(),
            is_exiting: false,
        };

        editor.buffer = strings_to_char_buffer(content);
        return editor;
    }

    pub fn handle_key(&mut self, key: KeyCode) {
        
        if self.mode_manager.current_state() == State::InsertMode {
            match key {
                KeyCode::Char(c) => self.insert_char(c),
                KeyCode::Backspace => self.backspace(),
                KeyCode::Enter => self.enter(),
                KeyCode::Up => self.move_cursor_up(),
                KeyCode::Down => self.move_cursor_down(),
                KeyCode::Left => self.move_cursor_left(),
                KeyCode::Right => self.move_cursor_right(),
                _ => {}
            }    
        }
        if self.mode_manager.current_state() == State::NormalMode {
            match key {
                KeyCode::Esc => self.is_exiting = true,
                KeyCode::Up => self.move_cursor_up(),
                KeyCode::Down => self.move_cursor_down(),
                KeyCode::Left => self.move_cursor_left(),
                KeyCode::Right => self.move_cursor_right(),
                _ => {}
            }    
        }

        self.mode_manager.handle_key(key);
    }
    
    pub fn insert_char(&mut self, c: char) {
        if self.mode_manager.current_state() == State::InsertMode {
            if self.cursor_y >= self.buffer.len() {
                self.buffer.push(vec![]);
            }
            self.buffer[self.cursor_y].insert(self.cursor_x, c);
            self.cursor_x += 1;
        }
    }

    pub fn backspace(&mut self) {
        if self.cursor_x > 0 {
            self.cursor_x -= 1;
            self.buffer[self.cursor_y].remove(self.cursor_x);
        }
    }

    pub fn enter(&mut self) {
        let new_line = self.buffer[self.cursor_y].split_off(self.cursor_x);
        self.cursor_y += 1;
        self.cursor_x = 0;
        self.buffer.insert(self.cursor_y, new_line);
    }

    pub fn move_cursor_up(&mut self) {
        if self.cursor_y > 0 {
            self.cursor_y -= 1;
            self.cursor_x = self.cursor_x.min(self.buffer[self.cursor_y].len());
        }
    }

    pub fn move_cursor_down(&mut self) {
        if self.cursor_y + 1 < self.buffer.len() {
            self.cursor_y += 1;
            self.cursor_x = self.cursor_x.min(self.buffer[self.cursor_y].len());
        }
    }

    pub fn move_cursor_left(&mut self) {
        if self.cursor_x > 0 {
            self.cursor_x -= 1;
        }
    }

    pub fn move_cursor_right(&mut self) {
        if self.cursor_x < self.buffer[self.cursor_y].len() {
            self.cursor_x += 1;
        }
    }

    pub fn get_cursor(&self) -> (usize, usize) {
        return (self.cursor_x, self.cursor_y);
    }

    pub fn get_content(&self) -> Vec<Vec<char>> {
        return self.buffer.clone();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_char() {
        let mut editor = Editor::from_strings([].to_vec());
        editor.handle_key(KeyCode::Char('i'));

        editor.insert_char('a');
        assert_eq!(editor.buffer[0], vec!['a']);
        assert_eq!(editor.cursor_x, 1);
    }

    #[test]
    fn test_backspace() {
        let mut editor = Editor::from_strings([].to_vec());
        editor.handle_key(KeyCode::Char('i'));

        editor.insert_char('a');
        editor.insert_char('b');
        editor.backspace();
        assert_eq!(editor.buffer[0], vec!['a']);
        assert_eq!(editor.cursor_x, 1);
    }

    #[test]
    fn test_enter() {
        let mut editor = Editor::from_strings([].to_vec());
        editor.handle_key(KeyCode::Char('i'));

        editor.insert_char('a');
        editor.insert_char('b');
        editor.enter();
        assert_eq!(editor.buffer.len(), 2);
        assert_eq!(editor.buffer[0], vec!['a', 'b']);
        assert_eq!(editor.buffer[1], vec![]);
        assert_eq!(editor.cursor_x, 0);
        assert_eq!(editor.cursor_y, 1);
    }

    #[test]
    fn test_move_cursor_up_down() {
        let mut editor = Editor::from_strings([].to_vec());
        editor.handle_key(KeyCode::Char('i'));

        editor.insert_char('x');
        editor.enter();
        editor.insert_char('y');

        editor.move_cursor_up();
        assert_eq!(editor.cursor_y, 0);
        assert_eq!(editor.cursor_x, 1);

        editor.move_cursor_down();
        assert_eq!(editor.cursor_y, 1);
        assert_eq!(editor.cursor_x, 1);
    }

    #[test]
    fn test_move_cursor_left_right() {
        let mut editor = Editor::from_strings([].to_vec());
        editor.handle_key(KeyCode::Char('i'));

        editor.insert_char('a');
        editor.insert_char('b');
        editor.move_cursor_left();
        assert_eq!(editor.cursor_x, 1);
        editor.move_cursor_right();
        assert_eq!(editor.cursor_x, 2);
        editor.move_cursor_right();
        assert_eq!(editor.cursor_x, 2);
    }

    #[test]
    fn test_backspace_on_empty_line() {
        let mut editor = Editor::from_strings([].to_vec());
        editor.handle_key(KeyCode::Char('i'));

        editor.backspace();
        assert_eq!(editor.cursor_x, 0);
    }
}
