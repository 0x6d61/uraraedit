mod terminal;
use std::io::{stdout, Write};
use terminal::Terminal;
use termion::event::Key;
use termion::raw::IntoRawMode;

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Position {
    pub x: usize,
    pub y: usize,
}

pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
    cursor_position: Position,
}

impl Editor {
    pub fn run(&mut self) {
        let _stdout = stdout().into_raw_mode().unwrap();
        loop {
            if let Err(error) = self.refresh_screen() {
                die(error);
            }
            if self.should_quit {
                Terminal::clear_screen();
                break;
            }
            if let Err(error) = self.process_keypress() {
                die(error)
            }
        }
    }
    pub fn default() -> Self {
        Self {
            should_quit: false,
            terminal: Terminal::default().expect(":("),
            cursor_position: Position { x: 0, y: 0 },
        }
    }
    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        Terminal::cursor_hide();
        Terminal::clear_screen();
        Terminal::cursor_position(&Position { x: 0, y: 0 });
        if !self.should_quit {
            self.draw_rows();
            Terminal::cursor_position(&self.cursor_position);
        }
        Terminal::cursor_show();
        stdout().flush()
    }
    fn process_keypress(&mut self) -> Result<(), std::io::Error> {
        let pressed_key = Terminal::read_key()?;
        match pressed_key {
            Key::Ctrl('q') => self.should_quit = true,
            Key::Up | Key::Down | Key::Left | Key::Right => self.move_cursor(pressed_key),
            _ => (),
        }

        Ok(())
    }
    fn move_cursor(&mut self, key: Key) {
        let Position { mut x, mut y } = self.cursor_position;
        let size = self.terminal.size();
        let height = size.height.saturating_sub(1) as usize;
        let width = size.width.saturating_sub(1) as usize;
        match key {
            Key::Up => y = y.saturating_sub(1),
            Key::Down =>{
                y = if y < height {
                    y.saturating_add(1)
                }else{
                    y
                }
            }
            Key::Left => x = x.saturating_sub(1),
            Key::Right => {
                x = if x < width {
                    x.saturating_add(1)
                }else{
                    x
                }
            }
            _ => (),
        }
        self.cursor_position = Position { x, y }
    }
    fn draw_rows(&self) {
        let height = self.terminal.size().height;
        for row in 0..height - 1 {
            Terminal::clear_current_line();
            if row == height / 3 {
                self.draw_welcome_message();
            } else {
                println!("~\r");
            }
        }
    }
    fn draw_welcome_message(&self) {
        let mut welcome_message = format!("uraraedit -- versions {}\r", VERSION);
        let width = self.terminal.size().width as usize;
        let len = welcome_message.len();
        let padding = width.saturating_sub(len) / 2;
        let spaces = " ".repeat(padding.saturating_sub(1));
        welcome_message = format!("~{}{}", spaces, welcome_message);
        welcome_message.truncate(width);
        println!("{}\r", welcome_message);
    }
}

fn die(e: std::io::Error) {
    Terminal::clear_screen();
    panic!("{}", e);
}
