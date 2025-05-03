pub struct Editor {
    pub buffer: Vec<Vec<char>>,
    pub cursor_x: usize,
    pub cursor_y: usize,
}

fn strings_to_char_buffer(lines: Vec<String>) -> Vec<Vec<char>> {
    lines
        .into_iter()
        .map(|line| line.chars().collect())
        .collect()
}

impl Editor {
    pub fn new() -> Self {
        Self {
            buffer: vec![vec![]],
            cursor_x: 0,
            cursor_y: 0,
        }
    }

    pub fn from_strings(content: Vec<String>) -> Editor {
        let mut editor = Editor {
            buffer: vec![vec![]],
            cursor_x: 0,
            cursor_y: 0,
        };

        editor.buffer = strings_to_char_buffer(content);
        return editor;
    }

    pub fn insert_char(&mut self, c: char) {
        if self.cursor_y >= self.buffer.len() {
            self.buffer.push(vec![]);
        }
        self.buffer[self.cursor_y].insert(self.cursor_x, c);
        self.cursor_x += 1;
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

    pub fn get_cursor_x(&self) -> usize {
        return self.cursor_x;
    }

    pub fn get_cursor_y(&self) -> usize {
        return self.cursor_y;
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
        let mut editor = Editor::new();
        editor.insert_char('a');
        assert_eq!(editor.buffer[0], vec!['a']);
        assert_eq!(editor.cursor_x, 1);
    }

    #[test]
    fn test_backspace() {
        let mut editor = Editor::new();
        editor.insert_char('a');
        editor.insert_char('b');
        editor.backspace();
        assert_eq!(editor.buffer[0], vec!['a']);
        assert_eq!(editor.cursor_x, 1);
    }

    #[test]
    fn test_enter() {
        let mut editor = Editor::new();
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
        let mut editor = Editor::new();
        editor.insert_char('x');
        editor.enter();
        editor.insert_char('y');

        editor.move_cursor_up();
        assert_eq!(editor.cursor_y, 0);
        assert_eq!(editor.cursor_x, 1); // x length

        editor.move_cursor_down();
        assert_eq!(editor.cursor_y, 1);
        assert_eq!(editor.cursor_x, 1); // y length
    }

    #[test]
    fn test_move_cursor_left_right() {
        let mut editor = Editor::new();
        editor.insert_char('a');
        editor.insert_char('b');
        editor.move_cursor_left();
        assert_eq!(editor.cursor_x, 1);
        editor.move_cursor_right();
        assert_eq!(editor.cursor_x, 2);
        // Right beyond buffer should not increase cursor_x
        editor.move_cursor_right();
        assert_eq!(editor.cursor_x, 2);
    }

    #[test]
    fn test_backspace_on_empty_line() {
        let mut editor = Editor::new();
        editor.backspace(); // shouldn't panic
        assert_eq!(editor.cursor_x, 0);
    }
}
