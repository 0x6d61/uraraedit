

#[cfg(test)]
mod tests {
    use std::fs::{self,File};
    use std::io::Write;
    use uraraedit::document::Document;
    use uraraedit::document::row::Row;
    use uraraedit::Position;
    fn create_example_file(filename: &str) {
        let mut file = File::create(filename).unwrap();
        file.write_all(b"test\nfoo\nwei").unwrap();
    }
    fn remove_example_file(filename:&str) {
        fs::remove_file(filename).unwrap();
    }
    #[test]
    fn test_document_is_empty() {
        let filename = "test_document_is_empty.txt";
        create_example_file(&filename);
        let document = Document::open(&filename).unwrap();

        //空ファイルの場合,true 
        assert_eq!(document.is_empty(),false);
        remove_example_file(filename);
    }
    #[test]
    fn test_document_len() {
        let filename = "test_document_len.txt";
        create_example_file(&filename);
        let document = Document::open(&filename).unwrap();
        //行数を取得
        assert_eq!(document.len(),3);
        remove_example_file(filename);
    }
    #[test]
    fn test_document_insert_newline() {
        let filename = "test_document_insert_newline.txt";
        create_example_file(&filename);
        let mut document = Document::open(&filename).unwrap();
        //新しい行を追加
        let position = Position{
            x:0,
            y:4,
        };
        assert_eq!(document.insert_newline(&position),());
        let position = Position{
            x:0,
            y:3,
        };
        document.insert_newline(&position);
        assert_eq!(document.rows.len(),4);
        let position = Position{
            x:4,
            y:0,
        };
        document.insert_newline(&position);
        assert_eq!(document.rows.len(),5);
        remove_example_file(filename);

    }
    #[test]
    fn test_document_insert() {
        let filename = "test_document_insert.txt";
        create_example_file(&filename);
        let mut document = Document::open(&filename).unwrap();

    }
}