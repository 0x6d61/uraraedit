

#[cfg(test)]
mod tests {
    use std::fs::{self,File};
    use std::io::Write;
    use uraraedit::document::Document;
    use uraraedit::document::row::Row;
    use uraraedit::Position;
    #[test]
    fn document_test() {
        let filename = "example.txt";
        let mut file = File::create(&filename).unwrap();
        file.write_all(b"test\nfoo\nwei").unwrap();
        let mut document = Document::open(&filename).unwrap();

        //空ファイルの場合,true 
        assert_eq!(document.is_empty(),false);
        //指定したindexの行を取得する
        if let Some(row) = document.row(0) {
            assert_eq!(row.string,"test");
            assert_eq!(row.len,4);
        }
        //行数を取得
        assert_eq!(document.len(),3);
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
    }
}