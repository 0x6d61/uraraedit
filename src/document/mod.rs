pub mod row;
use crate::Position;
use row::Row;
use std::fs;
use std::io::{Error,Write};
use std::path::Path;


#[derive(Debug)]
#[derive(Default)]
pub struct Document {
    pub rows: Vec<Row>,
    pub file_name: Option<String>,
    pub extension:Option<String>,
    dirty:bool,
}

impl Document {
    pub fn open(filename: &str) -> Result<Self,std::io::Error> {
        let contents = fs::read_to_string(filename)?;
        let mut rows = vec!();
        for line in contents.lines() {
            rows.push(Row::from(line));
        }
        let extension = match Path::new(filename).extension() {
            Some(ext) => ext.to_str().unwrap(),
            None => "txt"
        };
        Ok(Self{
            rows,
            extension:Some(extension.to_string()),
            file_name: Some(filename.to_string()),
            dirty:false,
        })
    }
    pub fn row(&self,index: usize) -> Option<&Row> {
        self.rows.get(index)
    }
    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }

    pub fn len(&self) -> usize {
        self.rows.len()
    }
    pub fn insert_newline(&mut self,at:&Position) {
        if at.y == self.len()  {
            self.rows.push(Row::default());
            return;
        }
        let new_row = self.rows.get_mut(at.y).unwrap().split(at.x-crate::NUMBER_PRINT_OFFSET);
        self.rows.insert(at.y+1,new_row);
    }
    pub fn insert(&mut self,at: &Position, c:char) {
        if at.y > self.len() {
            return;
        }
        self.dirty = true;
        if c == '\n' {
            self.insert_newline(at);
            return;
        }
        if at.y == self.len() {
            let mut row = Row::default();
            row.insert(0,c);
            self.rows.push(row);
        }if at.y < self.len(){
            let row = self.rows.get_mut(at.y).unwrap();
            row.insert(at.x-crate::NUMBER_PRINT_OFFSET,c);

        }
    }
    pub fn delete(&mut self,at: &Position) {
        let len = self.len();
        if at.y >= len {
            return;
        }
        self.dirty = true;
        if at.x == self.rows.get_mut(at.y).unwrap().len() && at.y < len -1 {
            let next_row = self.rows.remove(at.y + 1);
            let row = self.rows.get_mut(at.y).unwrap();
            row.append(&next_row);
        }else{
            let row = self.rows.get_mut(at.y).unwrap();
            row.delete(at.x-crate::NUMBER_PRINT_OFFSET);
        }
    }
    pub fn save(&mut self) -> Result<(),Error> {
        if let Some(file_name) = &self.file_name {
            let mut file = fs::File::create(file_name)?;
            for row in &self.rows {
                file.write_all(row.as_bytes())?;
                file.write_all(b"\n")?;
            }
            self.dirty = false;
        }
        Ok(())
    }
    pub fn is_dirty(&self) -> bool {
        self.dirty
    }
    
}
