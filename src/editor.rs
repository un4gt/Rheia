use std::io;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use iced::{Application, Command, Element, executor, highlighter, keyboard, Length, Subscription, Theme};
use iced::highlighter::Highlighter;
use iced::widget::{container, horizontal_space, pick_list, row, text, text_editor};
use iced::widget::text_editor::Content;
use crate::file_handler::{load_file, save_file, pick_file, default_file};
use crate::components::action;
use crate::icons::{
    open_icon,
    save_icon,
    new_icon
};

pub struct Editor {
    path: Option<PathBuf>,
    content: Content,
    error: Option<EditorError>,
    theme: highlighter::Theme,
    is_dirty: bool,
}

#[derive(Debug, Clone)]
pub enum EditorError {
    DialogClose,
    IO(io::ErrorKind),
    IOFailed(io::ErrorKind),
}

#[derive(Debug, Clone)]
pub enum Message {
    Edit(text_editor::Action),
    FileOpened(Result<(PathBuf, Arc<String>), EditorError>),
    Open,
    New,
    Save,
    FileSaved(Result<PathBuf, EditorError>),
    ThemeChanged(highlighter::Theme)
}

impl Application for Editor {
    type Executor = executor::Default; // `Fn` traits
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Message>) {
        (
            Self {
                content: Content::with(include_str!("main.rs")),
                error: None,
                path: None,
                theme: highlighter::Theme::SolarizedDark,
                is_dirty: true,
            },
            Command::perform(
                load_file(default_file()),
                Message::FileOpened,
            ),
        )
    }

    fn title(&self) -> String {
        String::from("Rheia")
    }

    fn update(&mut self, message: Self::Message) -> Command<Message> {
        match message {
            Message::Edit(action) => {
                self.is_dirty = self.is_dirty || action.is_edit();
                self.content.edit(action);

                self.error = None;

                Command::none()
            }
            Message::Open => {
                Command::perform(pick_file(), Message::FileOpened)
            }
            Message::FileOpened(Ok((path, content))) => {
                self.path = Some(path);
                self.content = text_editor::Content::with(&content);
                self.is_dirty = false;
                Command::none()
            }
            Message::FileOpened(Err(e)) => {
                self.error = Some(e);

                Command::none()
            }
            Message::New => {
                self.path =  None;
                self.content = text_editor::Content::new();
                self.is_dirty = true;
                Command::none()
            }
            Message::Save => {
                let contents = self.content.text();

                Command::perform(save_file(contents), Message::FileSaved)
            }
            Message::FileSaved(Ok(path)) => {
                self.path = Some(path);
                self.is_dirty = false;

                Command::none()
            }
            Message::FileSaved(Err(e)) => {
                self.error = Some(e);

                Command::none()
            }
            Message::ThemeChanged(new_theme) => {
                self.theme = new_theme;
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let controls = row![
            action(open_icon(), "Open File", Some(Message::Open)),
            action(new_icon(), "New File", Some(Message::New)),
            action(save_icon(), "Save File", self.is_dirty.then_some(Message::Save)),
            horizontal_space(Length::Fill),
            pick_list(highlighter::Theme::ALL, Some(self.theme), Message::ThemeChanged)
        ].spacing(10);
        let editor = text_editor(&self.content)
            .on_edit(Message::Edit)
            .highlight::<Highlighter>(highlighter::Settings {
                theme: self.theme,
                extension: self.path
                    .as_ref()
                    .and_then(|path| path.extension()?.to_str())
                    .unwrap_or("rs")
                    .to_string(),
            }, |highlight, _theme| {
                highlight.to_format()
            }
            );

        let status_bar = {
            let status = if let Some(EditorError::IO(error)) = self.error.as_ref() {
                text(error.to_string())
            }else {
                match self.path.as_deref().and_then(Path::to_str) {
                    None => text("New file"),
                    Some(path) => text(path).size(14)
                }
            };

            let position = {
                let (line, column) = self.content.cursor_position();
                let select = self.content.selection();

                let base_info = format!("{}:{}", line + 1, column + 1);

                match select {
                    None => text(base_info),
                    Some(select) => {
                        // line breaks
                        let breaks = select.matches('\n').count();
                        //  char selection
                        let extra_info = if breaks > 0 {
                            format!(" ({} chars, {} line breaks)", select.len(), breaks)
                        } else {
                            format!(" ({} chars)", select.len())
                        };
                        text(format!("{}{}", base_info, extra_info))
                    }
                }
            };
            row![status, horizontal_space(Length::Fill), position]
        };

        container(iced::widget::column![controls, editor, status_bar].spacing(10))
            .padding(10)
            .into()
    }

    fn theme(&self) -> Theme {
        if self.theme.is_dark() {
            Theme::Dark
        } else {
            Theme::Light
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        keyboard::on_key_press(|code, modifiers| {
            match code {
                keyboard::KeyCode::S if modifiers.command() => Some(Message::Save),
                _ => None
            }
        })
    }
}