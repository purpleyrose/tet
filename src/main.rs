mod editor;
mod document;
mod row;
mod terminal;
pub use document::Document;
use editor::Editor;
pub use terminal::Terminal;
pub use row::Row;
pub use editor::Position;
/// A simple editor that can be used to write and edit text.
/// This runs in the terminal, and can be used to write text to a file.
/// 
/// 
fn main() {
    Editor::default().run();
}
