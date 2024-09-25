use iced::executor;
use iced::widget::text_editor::Content;
use iced::widget::{button, column, container, horizontal_space, row, text, text_editor};
use iced::{Application, Command, Element, Length, Settings, Theme};
use std::io;
use std::path::Path;
use std::sync::Arc;

struct Editor {
    content: Content,
    error: Option<EditorError>,
}

#[derive(Debug, Clone)]
enum Message {
    Edit(text_editor::Action),
    FileOpened(Result<Arc<String>, EditorError>),
    Open,
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
            },
            Command::perform(
                load_file(format!("{}/src/main.rs", env!("CARGO_MANIFEST_DIR"))),
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
                self.content.edit(action);

                Command::none()
            }
            Message::Open => {
                Command::perform(pick_file(), Message::FileOpened)
            }
            Message::FileOpened(Ok(content)) => {
                self.content = text_editor::Content::with(&content);

                Command::none()
            }
            Message::FileOpened(Err(e)) => {
                self.error = Some(e);

                Command::none()
            }
        }
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let open_file_btn = button("Open ...").on_press(Message::Open);
        let controls = row![open_file_btn];
        let editor = text_editor(&self.content).on_edit(Message::Edit);

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

        let status_bar = row![horizontal_space(Length::Fill), position];
        container(column![controls, editor, status_bar].spacing(10))
            .padding(10)
            .into()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}

async fn load_file(path: impl AsRef<Path>) -> Result<Arc<String>, EditorError> {
    tokio::fs::read_to_string(path)
        .await
        .map(Arc::new)
        .map_err(|e| EditorError::IO(e.kind()))
}

#[derive(Debug, Clone)]
enum EditorError {
    DialogClose,
    IO(io::ErrorKind),
}

async fn pick_file() -> Result<Arc<String>, EditorError> {
    let handler = rfd::AsyncFileDialog::new()
        .set_title("Choose a text file ...")
        .pick_file()
        .await
        .ok_or(EditorError::DialogClose)?;
    load_file(handler.path())
        .await
}

fn main() -> iced::Result {
    Editor::run(Settings::default())
}
