use crate::components::action;
use crate::file_handler::{default_file, load_file, open_file, save_file};
use crate::icons::{new_icon, open_icon, save_icon};
use iced::event::{self, Event};
use iced::widget::{self, column, horizontal_space, pick_list, row, text, text_editor};
use iced::{highlighter, keyboard, window, Center, Element, Fill, Subscription, Task, Theme};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::{ffi, io};

pub struct Editor {
    file: Option<PathBuf>,
    content: text_editor::Content,
    theme: highlighter::Theme,
    is_dirty: bool,
    is_loading: bool,
}

#[derive(Debug, Clone)]
pub enum EditorError {
    DialogClose,
    IoError(io::ErrorKind),
}

#[derive(Debug, Clone)]
pub enum EditorMessage {
    ActionPerformed(text_editor::Action),
    FileOpened(Result<(PathBuf, Arc<String>), EditorError>),
    OpenFile,
    NewFile,
    SaveFile,
    FileDropped(PathBuf),
    FileSaved(Result<PathBuf, EditorError>),
    ThemeChanged(highlighter::Theme),
}

impl Editor {
    pub fn new() -> (Self, Task<EditorMessage>) {
        (
            Self {
                file: None,
                content: text_editor::Content::new(),
                theme: highlighter::Theme::SolarizedDark,
                is_dirty: false,
                is_loading: true,
            },
            Task::batch([
                Task::perform(load_file(default_file()), EditorMessage::FileOpened),
                widget::focus_next(),
            ]),
        )
    }

    pub fn update(&mut self, message: EditorMessage) -> Task<EditorMessage> {
        match message {
            EditorMessage::ActionPerformed(action) => {
                self.is_dirty = self.is_dirty || action.is_edit();
                self.content.perform(action);

                Task::none()
            }
            EditorMessage::FileOpened(result) => {
                self.is_loading = false;
                self.is_dirty = false;

                if let Ok((path, contents)) = result {
                    self.file = Some(path);
                    self.content = text_editor::Content::with_text(&contents);
                }

                Task::none()
            }
            EditorMessage::OpenFile => {
                if self.is_loading {
                    Task::none()
                } else {
                    self.is_loading = true;

                    Task::perform(open_file(), EditorMessage::FileOpened)
                }
            }
            EditorMessage::NewFile => {
                if !self.is_loading {
                    self.file = None;
                    self.content = text_editor::Content::new();
                }

                Task::none()
            }
            EditorMessage::SaveFile => {
                if self.is_loading {
                    Task::none()
                } else {
                    self.is_loading = true;

                    Task::perform(
                        save_file(self.file.clone(), self.content.text()),
                        EditorMessage::FileSaved,
                    )
                }
            }
            EditorMessage::FileSaved(result) => {
                self.is_loading = false;

                if let Ok(path) = result {
                    self.file = Some(path);
                    self.is_dirty = false;
                }

                Task::none()
            }
            EditorMessage::ThemeChanged(theme) => {
                self.theme = theme;

                Task::none()
            }

            EditorMessage::FileDropped(path) => Task::batch([
                Task::perform(load_file(path), EditorMessage::FileOpened),
                widget::focus_next(),
            ]),
        }
    }

    pub fn view(&self) -> Element<EditorMessage> {
        let controls = row![
            action(new_icon(), "New file", Some(EditorMessage::NewFile)),
            action(
                open_icon(),
                "Open file",
                (!self.is_loading).then_some(EditorMessage::OpenFile)
            ),
            action(
                save_icon(),
                "Save file",
                self.is_dirty.then_some(EditorMessage::SaveFile)
            ),
            horizontal_space(),
            pick_list(
                highlighter::Theme::ALL,
                Some(self.theme),
                EditorMessage::ThemeChanged
            )
            .text_size(14)
            .padding([5, 10])
        ]
        .spacing(10)
        .align_y(Center);

        let status_bar = {
            let file_status = if let Some(path) = &self.file {
                let path = path.display().to_string();
                if path.len() > 60 {
                    text(format!("...{}", &path[path.len() - 40..]))
                } else {
                    text(path)
                }
            } else {
                text("New File")
            };

            let position = {
                let (line, column) = self.content.cursor_position();
                let select = self.content.selection();

                let base_info = format!("{}:{}", line + 1, column + 1);

                match select {
                    None => text(base_info),
                    Some(select) => {
                        let breaks = select.matches('\n').count();
                        let extra_info = if breaks > 0 {
                            format!(" ({} chars, {} line breaks)", select.len(), breaks)
                        } else {
                            format!(" ({} chars)", select.len())
                        };
                        text(format!("{}{}", base_info, extra_info))
                    }
                }
            };
            row![file_status, horizontal_space(), position]
        };

        let editor = text_editor(&self.content)
            .height(Fill)
            .on_action(EditorMessage::ActionPerformed)
            .highlight(
                self.file
                    .as_deref()
                    .and_then(Path::extension)
                    .and_then(ffi::OsStr::to_str)
                    .unwrap_or("md"),
                self.theme,
            )
            .key_binding(|key_press| match key_press.key.as_ref() {
                keyboard::Key::Character("s") if key_press.modifiers.command() => {
                    Some(text_editor::Binding::Custom(EditorMessage::SaveFile))
                }
                _ => text_editor::Binding::from_key_press(key_press),
            });

        column![controls, editor, status_bar]
            .spacing(10)
            .padding(10)
            .into()
    }

    pub fn theme(&self) -> Theme {
        if self.theme.is_dark() {
            Theme::Dark
        } else {
            Theme::Light
        }
    }

    pub fn subscription(&self) -> Subscription<EditorMessage> {
        event::listen_with(|event, _status, _windows| match event {
            Event::Window(window::Event::FileDropped(path)) => {
                Some(EditorMessage::FileDropped(path))
            }
            _ => None,
        })
    }
}
