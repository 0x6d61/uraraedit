use std::cmp;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug, Default)]
pub struct Row {
    pub string: String,
    pub len: usize,
}

impl From<&str> for Row {
    fn from(slice: &str) -> Self {
        let mut row = Self {
            string: String::from(slice),
            len: 0,
        };
        row.update_len();
        row
    }
}

impl Row {
    pub fn render(&self, start: usize, end: usize) -> String {
        let end = cmp::min(end, self.string.len());
        let start = cmp::min(start, end);
        let mut result = String::new();
        for grapheme in self.string[..]
            .graphemes(true)
            .skip(start)
            .take(end - start)
        {
            if grapheme == "\t" {
                result.push_str(" ");
            } else {
                result.push_str(grapheme);
            }
        }
        result
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    fn update_len(&mut self) {
        self.len = self.string[..].graphemes(true).count() + 6;
    }
    pub fn insert(&mut self, at: usize, c: char) {
        let c = if c == '\t' {
            " ".repeat(4)
        }else{
            c.to_string()
        };

        if at >= self.len() {
            self.string.push_str(&c);
        } else {
            let mut result: String = self.string[..].graphemes(true).take(at).collect();
            let remainder: String = self.string[..].graphemes(true).skip(at).collect();
            result.push_str(&c);
            result.push_str(&remainder);
            self.string = result;
        }
        self.update_len();
        if c == "{".to_string() {
            return self.insert(at+1,'}');
        }else if c == "'".to_string() {
            return self.insert(at+1, '\'');
        }
    }
    pub fn delete(&mut self, at: usize) {
        if at >= self.len() {
            return;
        } else {
            let mut result: String = self.string[..].graphemes(true).take(at).collect();
            let remainder: String = self.string[..].graphemes(true).skip(at + 1).collect();
            result.push_str(&remainder);
            self.string = result;
        }
        self.update_len();
    }
    pub fn append(&mut self, new: &Self) {
        self.string = format!("{}{}", self.string, new.string);
        self.update_len();
    }
    pub fn split(&mut self, at: usize) -> Self {
        let beginning: String = self.string[..].graphemes(true).take(at).collect();
        let remainder: String = self.string[..].graphemes(true).skip(at).collect();
        self.string = beginning;
        self.update_len();
        Self::from(&remainder[..])
    }
    pub fn as_bytes(&self) -> &[u8] {
        self.string.as_bytes()
    }
}
