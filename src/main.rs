use iced::{Application, Font, Settings};
use crate::editor::Editor;

mod icons;
mod components;
mod file_handler;
mod editor;


fn main() -> iced::Result {
    Editor::run(
        Settings {
            default_font: Font::MONOSPACE,
            fonts: vec![
                include_bytes!("../fonts/editor-icons.ttf")
                    .as_slice()
                    .into()
            ],
            ..Settings::default()
        }
    )
}
