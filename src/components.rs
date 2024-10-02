use iced::{Element, theme};
use iced::widget::{button, container, tooltip};
use crate::editor::Message;

// some custom components
pub fn action<'a>(
    content: Element<'a, Message>,
    label: &str,
    on_press: Option<Message>
) -> Element<'a, Message> {
    let is_disabled = on_press.is_none();
    tooltip(
        button(container(content).width(30).center_x())
            .padding([5, 10])
            .style(
                if is_disabled {
                    theme::Button::Secondary
                } else {
                    theme::Button::Primary
                }
            )
            .on_press_maybe(on_press),
        label,
        tooltip::Position::FollowCursor,
    )
        .style(theme::Container::Box)
        .into()
}