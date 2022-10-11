mod document;
mod editor;
mod row;
mod terminal;
pub use document::Document;
use editor::Editor;
pub use terminal::Terminal;
pub use editor::Position;
pub use row::Row;
fn main() {
    Editor::new().run();
}
