use iced::Font;
use crate::editor::Editor;

mod icons;
mod components;
mod file_handler;
mod editor;


fn main() -> iced::Result {
    iced::application("Rheia - Iced", Editor::update, Editor::view)
        .theme(Editor::theme)
        .default_font(Font::MONOSPACE)
        .font(include_bytes!("../fonts/editor-icons.ttf"))
        .run_with(Editor::new)
}
