mod editor;
mod terminal_control;
use editor::Editor;

fn main() {
    let mut editor = Editor::default();
    editor.run();
}