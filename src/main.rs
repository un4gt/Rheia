use crate::editor::Editor;
use iced::Font;

mod components;
mod editor;
mod file_handler;
mod icons;

fn main() -> iced::Result {
    iced::application("Rheia - Iced", Editor::update, Editor::view)
        .theme(Editor::theme)
        .default_font(Font::MONOSPACE)
        .font(include_bytes!("../fonts/editor-icons.ttf"))
        .subscription(Editor::subscription)
        .run_with(Editor::new)
}
