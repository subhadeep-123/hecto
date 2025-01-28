#![warn(clippy::all, clippy::pedantic)]
use hecto::editor::Editor;

fn main() {
    Editor::new().run();
}
