mod terminal;
use std::io::{stdin, stdout,Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use terminal::Terminal;

pub struct Editor {
    should_quit: bool,
    terminal:Terminal,
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
        Self{
            should_quit:false,
            terminal:Terminal::default().expect(":("),
        }
    }
    fn refresh_screen(&self) -> Result<(),std::io::Error> {
        Terminal::cursor_hide();
        Terminal::clear_screen();
        Terminal::cursor_position(0,0);
        if !self.should_quit {
            self.draw_rows();
            Terminal::cursor_position(0,0);
        }
        Terminal::cursor_show();
        stdout().flush()
    }
    fn process_keypress(&mut self) -> Result<(),std::io::Error> {
        let pressed_key = Terminal::read_key()?;
        match pressed_key {
            Key::Ctrl('q') => self.should_quit = true,
            _ => (),
        }

        Ok(())
    }
    fn draw_rows(&self) {
        for _ in 0..self.terminal.size().height -1 {
            Terminal::clear_current_line();
            println!("~\r");
        }
    }
}

fn die(e: std::io::Error) {
    Terminal::clear_screen();
    panic!("{}", e);
}
