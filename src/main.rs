#![warn(clippy::all, clippy::pedantic)]
mod editor;
use editor::Editor;

fn main() {

    let editor = Editor::new();
    editor.run();

}