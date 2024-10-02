use iced::{executor, Font, theme};
use iced::widget::text_editor::Content;
use iced::widget::{button,tooltip, column, container, horizontal_space, row, text, text_editor};
use iced::{Application, Command, Element, Length, Settings, Theme};
use std::io;
use std::path::{Path, PathBuf};
use std::sync::Arc;

struct Editor {
    path: Option<PathBuf>,
    content: Content,
    error: Option<EditorError>,
}

#[derive(Debug, Clone)]
enum Message {
    Edit(text_editor::Action),
    FileOpened(Result<(PathBuf, Arc<String>), EditorError>),
    Open,
    New,
    Save,
    FileSaved(Result<PathBuf, EditorError>)
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

                Command::none()
            }
            Message::FileOpened(Err(e)) => {
                self.error = Some(e);

                Command::none()
            }
            Message::New => {
                self.path =  None;
                self.content = text_editor::Content::new();
                Command::none()
            }
            Message::Save => {
                let contents = self.content.text();

                Command::perform(save_file(contents), Message::FileSaved)
            }
            Message::FileSaved(Ok(path)) => {
                self.path = Some(path);

                Command::none()
            }
            Message::FileSaved(Err(e)) => {
                self.error = Some(e);

                Command::none()
            }
        }
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let controls = row![
            action(open_icon(), "Open File", Message::Open),
            action(new_icon(), "New File", Message::New),
            action(save_icon(), "Save File", Message::Save),
        ].spacing(10);
        let editor = text_editor(&self.content).on_edit(Message::Edit);

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

        container(column![controls, editor, status_bar].spacing(10))
            .padding(10)
            .into()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}

async fn load_file(path: PathBuf) -> Result<(PathBuf,Arc<String>), EditorError> {
    let contents = tokio::fs::read_to_string(&path)
        .await
        .map(Arc::new)
        .map_err(|e| EditorError::IO(e.kind()))?;
    Ok((path, contents))
}

#[derive(Debug, Clone)]
enum EditorError {
    DialogClose,
    IO(io::ErrorKind),
    IOFailed(io::ErrorKind),
}

async fn pick_file() -> Result<(PathBuf, Arc<String>), EditorError> {
    let handler = rfd::AsyncFileDialog::new()
        .set_title("Choose a text file ...")
        .set_directory(format!("{}",env!("CARGO_MANIFEST_DIR")))
        .pick_file()
        .await
        .ok_or(EditorError::DialogClose)?;
    load_file(handler.path().to_owned())
        .await
}

async fn save_file(file_text: String) -> Result<PathBuf, EditorError> {
    let handle = rfd::AsyncFileDialog::new()
        .set_title("Save file ...")
        .set_directory(format!("{}",env!("CARGO_MANIFEST_DIR")))
        .save_file()
        .await
        .ok_or(EditorError::DialogClose)?;
    tokio::fs::write(handle.path(), file_text)
        .await
        .map_err(|error| EditorError::IOFailed(error.kind()))?;
    Ok(PathBuf::from(handle.path()))
}

fn default_file() -> PathBuf {
    PathBuf::from(format!("{}\\README.md", env!("CARGO_MANIFEST_DIR")))
}

fn icon<'a, Message>(codepoint: char) -> Element<'a, Message> {
    const ICON_FONT: Font = Font::with_name("editor-icons");

    text(codepoint).font(ICON_FONT).into()
}

fn open_icon<'a>() -> Element<'a, Message> {
    icon('\u{E800}')
}

fn new_icon<'a>() -> Element<'a, Message> {
    icon('\u{E801}')
}

fn save_icon<'a>() -> Element<'a, Message> {
    icon('\u{E802}')
}

fn action<'a>(
    content: Element<'a, Message>,
    label: &str,
    on_press: Message
) -> Element<'a, Message> {
    tooltip(
        button(container(content).width(30).center_x())
            .padding([5, 10])
            .on_press(on_press),
        label,
        tooltip::Position::FollowCursor,
    )
        .style(theme::Container::Box)
        .into()
}

fn main() -> iced::Result {
    Editor::run(
        Settings {
            fonts: vec![
                include_bytes!("../fonts/editor-icons.ttf")
                    .as_slice()
                    .into()
            ],
            ..Settings::default()
        }
    )
}
