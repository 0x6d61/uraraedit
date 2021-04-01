mod lib;
use uraraedit::Editor;

fn die(e: std::io::Error) {
    panic!("{}", e);
}

fn main() {
    let editor = Editor{};
    editor.run()
}
