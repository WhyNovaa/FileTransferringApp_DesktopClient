use iced::{window, Length, Padding, Size, Vector};
use iced::{Alignment, Background, Border, Element, Sandbox, Settings, Shadow};
use iced::alignment::{Horizontal, Vertical};
use iced::theme::Theme;
use iced::widget::{button, container, TextInput, text, Button, Column, Container, Row, Scrollable, Space, Image, Checkbox};
use reqwest::blocking::Client;
use std::collections::HashMap;
use dotenv::dotenv;
use std::env;
use iced::window::Icon;
use serde_json;

fn main() -> iced::Result {

    let mut settings = Settings::default();

    let window_settings = window::Settings {
        min_size: Some(Size::new(700.0, 600.0)),
        icon: Some(load_icon("src/resources/icon.ico")),
        ..window::Settings::default()
    };


    settings.window = window_settings;
    App::run(settings)
}

fn load_icon(path: &str) -> Icon {
    let result = window::icon::from_file(path);

    match result {
        Ok(icon) => {
            icon.into()
        }
        Err(e) => {
            println!("Load icon error");
            let rgba: Vec<u8> = create_rgba_image(16, 16);
            let width = 16;
            let height = 16;
            window::icon::from_rgba(rgba, width, height).expect("Failed to create icon")
        }
    }
}


fn create_rgba_image(width: u32, height: u32) -> Vec<u8> {
    let mut image = vec![0; (width * height * 4) as usize];

    for y in 0..height {
        for x in 0..width {
            let index = (y * width + x) as usize * 4;
            image[index] = 135;
            image[index + 1] = 206;
            image[index + 2] = 250;
            image[index + 3] = 255;
        }
    }

    image
}


struct App {
    theme: Theme,
    page: Page,
    login_field: LoginField,
    token: String,
    client: Client,
    login_error: Option<String>,
    packages: Vec<PackageRow>,
    server: Server
}

struct LoginField {
    login: String,
    password: String
}

struct Server {
    URL: String,
    grant_type: String,
}
#[derive(Debug, Clone, PartialEq, Eq)]
enum Page{
    Login,
    Main
}

#[derive(Debug, Clone)]
enum Message {
    ToggleTheme,
    LoginSubmit,
    LoginFieldChanged(String, String),
    DeleteFileClicked(String),
    EditFileClicked(String),
    ToggleCheck(usize),
}


#[derive(Debug, Clone)]
struct PackageRow {
    checked: bool,
    filename: String,
}

impl PackageRow {
    fn new(filename: String) -> Self {
        PackageRow {
            checked: false,
            filename,
        }
    }
    fn view(&self, index: usize) -> Container<'static, Message> {
        let row = Row::new()
            .push(Space::with_width(10))
            .push(Space::with_width(20))
            .push(Checkbox::new("", self.checked).on_toggle(move |_| Message::ToggleCheck(index)))
            .push(text(self.filename.to_string()).size(20))
            .push(Space::with_width(Length::Fill))
            .push(edit_btn(Message::EditFileClicked(self.filename.to_string())))
            .push(Space::with_width(20))
            .push(del_btn(Message::DeleteFileClicked(self.filename.to_string())))
            .push(Space::with_width(10))
            //.height(Length::Fill)
            .align_items(Alignment::Center);

        container(row)
            .style(iced::theme::Container::Custom(Box::new(FileStyle)))
    }
}

impl Sandbox for App {
    type Message = Message;

    fn new() -> Self {
        dotenv().ok();
        Self {
            theme: Theme::Dark,
            page: Page::Login,
            login_field: LoginField {
                login: String::new(),
                password: String::new()
            },
            token: String::new(),
            client: Client::new(),
            login_error: None,
            packages: (1..=100)
                .map(|i| PackageRow::new("Aboba".to_string()))
                .collect(),
            server: Server {
                URL: env::var("SERVER_URL").expect("SERVER_URL must be set").to_string(),
                grant_type: "urn:ietf:params:oauth:grant-type:jwt-bearer".to_owned()
            }
        }
    }

    fn title(&self) -> String {
        String::from("FTA")
    }

    fn theme(&self) -> Theme {
        self.theme.clone()
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::ToggleTheme => {
                self.theme = if self.theme == Theme::Light {
                    Theme::Dark
                } else {
                    Theme::Light
                };
            }


            Message::LoginSubmit => {
                log_in_request(self);
            }


            Message::LoginFieldChanged(login, password) => {
                self.login_field.login = login;
                self.login_field.password = password;
            }


            Message::DeleteFileClicked(filename) => {
                // Реализация удаления файла
                println!("{}", filename);
            }


            Message::EditFileClicked(filename) => {
                // Реализация редактирования файла
                println!("{}", filename);
            }

            Message::ToggleCheck(index) => {
                if let Some(row) = self.packages.get_mut(index) {
                    row.checked = !row.checked;
                }
            }
        }
    }
    fn view(&self) -> Element<Message> {
        let content =
            match self.page {
            Page::Login => log_in_page(&self.login_field, self.login_error.clone()),
            Page::Main => main_page(&self.client ,&self.token, &self.packages)
        };


        let wrapper =  Column::new();

        let wrapper =
            match self.page {
            Page::Login => wrapper.spacing(10)
                .width(Length::Fill)
                .align_items(Alignment::Center)
                .push(content)
                .push(page_footer()),

            Page::Main => wrapper.push(page_footer())
                .spacing(10)
                .width(Length::Fill)
                .align_items(Alignment::Center)
                .push(content),
        };

        let temp_container = container(wrapper)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .style(iced::theme::Container::Custom(Box::new(ContainerStyle)));

        let container = match self.page {
                Page::Login => temp_container.center_y(),
                Page::Main => temp_container.align_y(Vertical::Top),
        };
        container.width(Length::Fill).height(Length::Fill).into()



    }


}





fn log_in_request(app: &mut App) {
    let mut params = HashMap::new();
    params.insert("username", app.login_field.login.to_string());
    params.insert("password", app.login_field.password.to_string());

    let result = app.client.post(app.server.URL.to_string() + "/login")
        .form(&params)
        .send();

    match result {
        Ok(response) => {
            let json_result: Result<HashMap<String, String>, _> = response.json();
            match json_result {
                Ok(json) => {
                    if let Some(token) = json.get("token") {
                        println!("{}", token);
                        app.token = token.clone();
                        app.page = Page::Main;
                    }
                    else {
                        app.login_error = Some("Wrong username or password".to_string())
                    }
                },

                Err(e) => {
                    app.login_error= Some("Wrong username or password".to_string());
                    eprintln!("Error parsing JSON: {}", e)
                },

            }
        }
        Err(e) => {
            app.login_error = Some("Internet connection error".to_string());
            eprintln!("Error sending request: {}", e)
        },

    }
}

fn page_footer() -> Container<'static, Message> {
    let footer = Row::new()
        .push(
            button("Toggle Theme")
                .on_press(Message::ToggleTheme)
                .style(iced::theme::Button::Custom(Box::new(ButtonStyle::ThemeButton)),
            )
        )
        .align_items(Alignment::Center)
        .spacing(10);

    container(footer).center_x().center_y()

}
fn log_in_page(login_field: &LoginField, login_error: Option<String>) -> Container<Message> {
    let mut column = Column::new()
        .push(text("File Transferring App"))
        .push(
            input_field("Login", &login_field.login)
            .on_input(
                |login| {
                Message::LoginFieldChanged(login, login_field.password.clone())
                }
            )
        )
        .push(
            input_field("Password", &login_field.password)
            .on_input(
                |password| {
                    Message::LoginFieldChanged(login_field.login.clone(), password)
                }
            )
        )
        .push(submit_btn("Login", Message::LoginSubmit))
        .padding(Padding::from([50, 20]))
        .align_items(Alignment::Center)
        .spacing(40);

    if let Some(error) = login_error {
        column = column.push(
            text(error)
                .size(16)
                .style(iced::theme::Text::Color(iced::Color::from_rgb(1.0, 0.0, 0.0)))
        );
        container(column)
            .padding(Padding::from([20, 20, 0, 20]))
            .style(iced::theme::Container::Custom(Box::new(ContainerStyle)))
    }
    else {
        container(column)
            .padding(Padding::from(20))
            .style(iced::theme::Container::Custom(Box::new(ContainerStyle)))
    }
}

/*fn create_row() -> Container<'static, Message> {
    let row = Row::new()
        .push(Space::with_width(10))
        .push(text("text1").size(20))
        .push(Space::with_width(Length::Fill))
        .push(edit_btn(Message::EditFileClicked("TEMP1".to_string())))
        .push(Space::with_width(20))
        .push(del_btn(Message::DeleteFileClicked("TEMP2".to_string())))
        .push(Space::with_width(10))
        .height(Length::Fill)
        .align_items(Alignment::Center);

    container(row)
        .style(iced::theme::Container::Custom(Box::new(FileStyle)))
}*/

fn main_page(client: &Client, token: &str, packages: &Vec<PackageRow>) -> Container<'static, Message> {
    let mut column = Column::new()
        .width(Length::Fill)
        .spacing(15);

    column = column.push(Space::with_height(0));


    for (index, package) in packages.iter().enumerate() {
        column = column.push(package.view(index));
    }

    column = column.push(Space::with_height(0));


    let scrollable = Scrollable::new(column)
        .width(Length::Fill)
        .height(Length::Fill);


    container(scrollable)
        .style(iced::theme::Container::Custom(Box::new(ContainerStyle)))
        .align_y(Vertical::Top)

}

fn input_field(_placeholder: &str, _value: &str, ) -> TextInput<'static, Message> {
    TextInput::new(_placeholder, _value)
        .width(Length::Fixed(500.0))
        .padding(Padding::from(10))
        .line_height(text::LineHeight::Relative(1.75))
}

fn del_btn(event: Message) -> Button<'static, Message> {
    let image = Image::new("src/resources/delete.png");

    Button::new(image)
        .on_press(event)
        .width(32)
        .height(32)
        .style(iced::theme::Button::Custom(Box::new(ButtonStyle::Transparent)))
}

fn edit_btn(event: Message) -> Button<'static, Message> {
    let image = Image::new("src/resources/edit.png");

    Button::new(image)
        .on_press(event)
        .width(32)
        .height(32)
        .style(iced::theme::Button::Custom(Box::new(ButtonStyle::Transparent)))
}


fn submit_btn(name: &str, event: Message) -> Button<Message> {
    Button::new(
        text(name)
        .horizontal_alignment(Horizontal::Center)
        .vertical_alignment(Vertical::Center)
        .size(21)
    )
    .on_press(event)
    .width(Length::Fixed(500.0))
    .height(Length::Fixed(45.0))
    .style(iced::theme::Button::Custom(Box::new(ButtonStyle::Standard)))
}



enum ButtonStyle {
    Standard,
    ThemeButton,
    Transparent, // Новый стиль
}

impl button::StyleSheet for ButtonStyle {
    type Style = Theme;

    fn active(&self, theme: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: match self {
                Self::Standard => Some(Background::Color(iced::Color::from_rgb(0.059, 0.463, 0.702))),
                Self::ThemeButton => Some(Background::Color(iced::Color::default())),
                Self::Transparent => None,
            },
            border: match self {
                Self::Standard => Border::with_radius(5),
                Self::ThemeButton => Border::default(),
                Self::Transparent => Border::default(),
            },
            shadow_offset: match self {
                Self::Standard => Vector::new(0.0, 2.0),
                Self::ThemeButton => Vector::new(0.0, 0.0),
                Self::Transparent => Vector::new(0.0, 0.0),
            },
            shadow: match self {
                Self::Standard => Shadow {
                    color: iced::Color::BLACK,
                    offset: Vector::new(0.0, 4.0),
                    blur_radius: 20.0,
                },
                Self::ThemeButton => Shadow::default(),
                Self::Transparent => Shadow::default(),
            },
            text_color: {
                if theme == &Theme::Light {
                    match self {
                        Self::Standard => iced::Color::WHITE,
                        Self::ThemeButton => iced::Color::BLACK,
                        Self::Transparent => iced::Color::BLACK,
                    }
                } else {
                    match self {
                        Self::Standard => iced::Color::WHITE,
                        Self::ThemeButton => iced::Color::WHITE,
                        Self::Transparent => iced::Color::WHITE,
                    }
                }
            },
        }
    }
}

struct ContainerStyle;

impl container::StyleSheet for ContainerStyle {
    type Style = Theme;

    fn appearance(&self, _: &Self::Style) -> container::Appearance {
        container::Appearance {
            text_color: Default::default(),
            border: Border::with_radius(5),
            background: None,
            shadow: Shadow {
                color: iced::Color::BLACK,
                offset: Vector::new(0.0, 2.0),
                blur_radius: 40.0,
            },
        }
    }
}

struct FileStyle;
impl container::StyleSheet for FileStyle {
    type Style = Theme;

    fn appearance(&self, _: &Self::Style) -> container::Appearance {
        container::Appearance {
            text_color: Default::default(),
            border: Border::with_radius(50),
            background: None,
            shadow: Shadow {
                color: iced::Color::BLACK,
                offset: Vector::new(0.0, 2.0),
                blur_radius: 40.0,
            },
        }
    }
}

