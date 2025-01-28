#![warn(clippy::all, clippy::pedantic)]
use hecto::editor::Editor;

fn main() {
    let editor = Editor::new();
    editor.run();
}
