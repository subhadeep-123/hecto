#![warn(clippy::all, clippy::pedantic, clippy::print_stdout)]
use hecto::editor::Editor;

fn main() {
    Editor::new().run();
}
