use iced::{Element, Font};
use iced::widget::text;
use crate::editor::Message;

// global icon handlers
const ICON_FONT: Font = Font::with_name("editor-icons");

fn icon<'a, Message>(codepoint: char) -> Element<'a, Message> {
    text(codepoint).font(ICON_FONT).into()
}

pub fn open_icon<'a>() -> Element<'a, Message> {
    icon('\u{E800}')
}

pub fn new_icon<'a>() -> Element<'a, Message> {
    icon('\u{E801}')
}

pub fn save_icon<'a>() -> Element<'a, Message> {
    icon('\u{E802}')
}