pub mod document;
pub mod terminal;

use document::{row::Row, Document};
use std::env;
use std::io::stdin;
use std::time::Duration;
use std::time::Instant;
use syntect::easy::HighlightLines;
use syntect::highlighting::{Color, Style, Theme, ThemeSet};
use syntect::parsing::SyntaxSet;
use syntect::util::as_24_bit_terminal_escaped;
use terminal::Terminal;
use termion::color;
use termion::event::Key;
use termion::input::TermRead;

pub const NUMBER_PRINT_OFFSET: usize = 6;
const STATUS_FG_COLOR: color::Rgb = color::Rgb(63, 63, 63);
const STATUS_BG_COLOR: color::Rgb = color::Rgb(239, 239, 239);
const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Default)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

struct StatusMessage {
    text: String,
    time: Instant,
}
impl StatusMessage {
    fn from(message: String) -> Self {
        Self {
            time: Instant::now(),
            text: message,
        }
    }
}

pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
    cursor_position: Position,
    offset: Position,
    document: Document,
    status_message: StatusMessage,
    background_color: Color,
    ts: Theme,
    ss: SyntaxSet,
}

impl Editor {
    pub fn run(&mut self) {
        loop {
            if let Err(error) = self.refresh_screen() {
                die(error);
            }
            if self.should_quit {
                break;
            }
            if let Err(error) = self.process_keypress() {
                die(error);
            }
        }
    }

    pub fn default() -> Self {
        let args: Vec<String> = env::args().collect();
        let mut initial_status = String::from("HELP: Ctrl-Q = quit|Ctrl-S = save");
        let document = if args.len() > 1 {
            let file_name = &args[1];
            let doc = Document::open(&file_name);
            if doc.is_ok() {
                doc.unwrap()
            } else {
                initial_status = format!("ERR: Could not open file: {}", file_name);
                Document::default()
            }
        } else {
            Document::default()
        };
        let timeout = std::time::Duration::from_millis(100);
        let rgb = termbg::rgb(timeout).unwrap();
        /*
         *color?????????????????????/?????????????????????????????????
         *document.Rows.Row(0)????????????/???????????????????????????
         */

        if let Some(_) = stdin().lock().keys().next() {}
        let background_color = Color {
            r: (rgb.r >> 8) as u8,
            b: (rgb.b >> 8) as u8,
            g: (rgb.g >> 8) as u8,
            a: 0,
        };
        Self {
            should_quit: false,
            terminal: Terminal::default().expect("Failed to initialize terminal"),
            document,
            cursor_position: Position {
                x: NUMBER_PRINT_OFFSET,
                y: 0,
            },
            offset: Position { x: 0, y: 0 },
            status_message: StatusMessage::from(initial_status),
            background_color,
            ss: SyntaxSet::load_defaults_newlines(),
            ts: ThemeSet::load_defaults().themes["base16-ocean.dark"].clone(),
        }
    }
    fn refresh_screen(&mut self) -> Result<(), std::io::Error> {
        Terminal::cursor_hide();
        Terminal::cursor_position(&Position::default());
        if !self.should_quit {
            self.draw_rows();
            self.draw_status_bar();
            self.draw_message_bar();
            Terminal::cursor_position(&Position {
                x: self.cursor_position.x.saturating_sub(self.offset.x),
                y: self.cursor_position.y.saturating_sub(self.offset.y),
            });
        } else {
            Terminal::clear_screen();
        }
        Terminal::cursor_show();
        Terminal::flush()
    }
    fn save(&mut self) {
        if self.document.file_name.is_none() {
            let new_name = self.prompt("Save as: ").unwrap_or(None);
            if new_name.is_none() {
                self.status_message = StatusMessage::from("Save aborted.".to_string());
                return;
            }
            self.document.file_name = new_name;
        }
        if self.document.save().is_ok() {
            self.status_message = StatusMessage::from("File saved successfully".to_string());
        } else {
            self.status_message = StatusMessage::from("Error writing file!".to_string());
        }
    }
    fn process_keypress(&mut self) -> Result<(), std::io::Error> {
        let pressed_key = Terminal::read_key()?;
        match pressed_key {
            Key::Ctrl('q') => {
                if self.document.is_dirty() {
                    let not_save = self.prompt("Exit without saving Y/n:").unwrap().unwrap();
                    if not_save == "Y".to_string() || not_save.to_string() == "y" {
                        self.should_quit = true;
                    } else {
                        return Ok(());
                    }
                }
                self.should_quit = true;
            }
            Key::Ctrl('s') => self.save(),
            Key::Ctrl('a') => {
                self.cursor_position.x = crate::NUMBER_PRINT_OFFSET;
            }
            Key::Ctrl('e') => {
                let end_line = self
                    .document
                    .row(self.cursor_position.y as usize)
                    .unwrap()
                    .len;
                self.cursor_position.x = end_line;
            }
            Key::Ctrl('h') => {
                self.cursor_position.y = 0;
                self.scroll();
            }
            Key::Ctrl('g') => {
                self.cursor_position.y = self.document.len();
                self.scroll();
            }
            Key::Char(c) => {
                self.document.insert(&self.cursor_position, c);
                /*
                 *tab == 4space
                 *??????????????????space4???????????????
                 */
                if c == '\t' {
                    self.cursor_position.x += 3;
                }
                self.move_cursor(Key::Right);
            }
            Key::Delete => self.document.delete(&self.cursor_position),
            Key::Backspace => {
                if self.cursor_position.x > 0 || self.cursor_position.y > 0 {
                    self.move_cursor(Key::Left);
                    self.document.delete(&self.cursor_position);
                }
            }
            Key::Up | Key::Down | Key::Left | Key::Right => self.move_cursor(pressed_key),
            _ => (),
        }
        self.scroll();
        Ok(())
    }
    fn scroll(&mut self) {
        let Position { x, y } = self.cursor_position;
        let width = self.terminal.size().width as usize;
        let height = self.terminal.size().height as usize;
        let mut offset = &mut self.offset;
        if y < offset.y {
            offset.y = y;
        } else if y >= offset.y.saturating_add(height) {
            offset.y = y.saturating_sub(height).saturating_add(1);
        }
        if x < offset.x {
            offset.x = x;
        } else if x >= offset.x.saturating_add(width) {
            offset.x = x.saturating_sub(width).saturating_add(1);
        }
    }
    fn move_cursor(&mut self, key: Key) {
        let Position { mut y, mut x } = self.cursor_position;
        let height = self.document.len();
        let mut width = if let Some(row) = self.document.row(y) {
            row.len()
        } else {
            0
        };
        match key {
            Key::Up => y = y.saturating_sub(1),
            Key::Down => {
                if y < height - 1 {
                    y = y.saturating_add(1);
                }
            }
            Key::Left => {
                if x > NUMBER_PRINT_OFFSET {
                    x -= 1;
                } else if y > 0 {
                    y -= 1;
                    if let Some(row) = self.document.row(y) {
                        x = row.len();
                    } else {
                        x = NUMBER_PRINT_OFFSET;
                    }
                }
            }
            Key::Right => {
                if x < width {
                    x += 1;
                } else if y < height {
                    y += 1;
                    x = NUMBER_PRINT_OFFSET;
                }
            }
            _ => (),
        }
        width = if let Some(row) = self.document.row(y) {
            row.len()
        } else {
            NUMBER_PRINT_OFFSET
        };
        if x > width {
            x = width;
        }

        self.cursor_position = Position { x, y }
    }

    fn draw_welcome_message(&self) {
        let mut welcome_message = format!("urara editor -- version {}", VERSION);
        let width = self.terminal.size().width as usize;
        let len = welcome_message.len();
        let padding = width.saturating_sub(len) / 2;
        let spaces = " ".repeat(padding.saturating_sub(1));
        welcome_message = format!("~{}{}", spaces, welcome_message);
        welcome_message.truncate(width);
        println!("{}\r", welcome_message);
    }
    pub fn draw_row(&self, row: &Row, terminal_row: usize) -> String {
        let width = self.terminal.size().width as usize;
        let start = self.offset.x;
        let end = self.offset.x + width;
        let row = row.render(start, end);
        Terminal::set_fg_color(color::Rgb(255, 215, 0));
        print!("{0:>5} ", terminal_row as usize + self.offset.y + 1);
        Terminal::reset_fg_color();
        row
    }
    fn draw_rows(&mut self) {
        let ext = self.document.extension.clone().unwrap_or("txt".to_string());
        let syntax = self.ss.find_syntax_by_extension(&ext).unwrap();
        self.ts.settings.background = Some(self.background_color);
        let mut highlight = HighlightLines::new(syntax, &self.ts);
        let height = self.terminal.size().height;
        for terminal_row in 0..height {
            Terminal::clear_current_line();
            if let Some(row) = self.document.row(terminal_row as usize + self.offset.y) {
                let drow = self.draw_row(row, terminal_row as usize);
                let ranges: Vec<(Style, &str)> = highlight.highlight(&drow, &self.ss);
                let escaped = as_24_bit_terminal_escaped(&ranges[..], true);
                println!("{}\r", escaped);
            } else if self.document.is_empty() && terminal_row == height / 3 {
                self.draw_welcome_message();
            } else {
                println!("~\r");
            }
        }
    }
    fn draw_status_bar(&self) {
        let mut status;
        let width = self.terminal.size().width as usize;
        let modified_indicator = if self.document.is_dirty() {
            " (modified)"
        } else {
            ""
        };
        let mut file_name = "[No Name]".to_string();
        if let Some(name) = &self.document.file_name {
            file_name = name.clone();
            file_name.truncate(20);
        }
        status = format!(
            "{} - {} lines{}",
            file_name,
            self.document.len(),
            modified_indicator
        );

        let line_indicator = format!(
            "{}/{}",
            self.cursor_position.y.saturating_add(1),
            self.document.len()
        );
        let len = status.len() + line_indicator.len();
        if width > len {
            status.push_str(&" ".repeat(width - len));
        }
        status = format!("{}{}", status, line_indicator);
        status.truncate(width);
        Terminal::set_bg_color(STATUS_BG_COLOR);
        Terminal::set_fg_color(STATUS_FG_COLOR);
        println!("{}\r", status);
        Terminal::reset_fg_color();
        Terminal::reset_bg_color();
    }
    fn draw_message_bar(&self) {
        Terminal::clear_current_line();
        let message = &self.status_message;
        if Instant::now() - message.time < Duration::new(5, 0) {
            let mut text = message.text.clone();
            text.truncate(self.terminal.size().width as usize);
            print!("{}", text);
        }
    }
    fn prompt(&mut self, prompt: &str) -> Result<Option<String>, std::io::Error> {
        let mut result = String::new();
        loop {
            self.status_message = StatusMessage::from(format!("{}{}", prompt, result));
            self.refresh_screen()?;
            match Terminal::read_key()? {
                Key::Backspace => {
                    if !result.is_empty() {
                        result.truncate(result.len() - 1);
                    }
                }
                Key::Char('\n') => break,
                Key::Char(c) => {
                    if !c.is_control() {
                        result.push(c);
                    }
                }
                Key::Esc => {
                    result.truncate(0);
                    break;
                }
                _ => (),
            }
        }
        self.status_message = StatusMessage::from(String::new());
        if result.is_empty() {
            return Ok(None);
        }
        Ok(Some(result))
    }
}
fn die(e: std::io::Error) {
    Terminal::clear_screen();
    panic!("{}", e);
}
