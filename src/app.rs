use std::env;
use dotenv::dotenv;
use iced::{Element, Sandbox, Theme};
use reqwest::blocking::Client;

use crate::ui;
use crate::handlers::handle_update;

pub struct App {
    pub theme: Theme,
    pub page: Page,
    pub login_field: LoginField,
    pub token: String,
    pub client: Client,
    pub login_error: Option<String>,
    pub packages: Vec<ui::PackageRow>,
    pub server: Server
}

pub struct LoginField {
    pub login: String,
    pub password: String
}

pub struct Server {
    pub url: String,
    pub grant_type: String,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Page{
    Login,
    Main
}

#[derive(Debug, Clone)]
pub enum Message {
    ToggleTheme,
    LoginSubmit,
    LoginFieldChanged(String, String),
    DeleteFileClicked(String),
    EditFileClicked(String),
    ToggleCheck(usize),
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
                .map(|_| ui::PackageRow::new("filename".to_string()))
                .collect(),
            server: Server {
                url: env::var("SERVER_URL").expect("SERVER_URL must be set").to_string(),
                grant_type: "urn:ietf:params:oauth:grant-type:jwt-bearer".to_owned()
            }
        }
    }

    fn title(&self) -> String {
        String::from("FTA")
    }

    fn update(&mut self, message: Message) {
        handle_update(self, message);
    }

    fn view(&self) -> Element<Message> {
        ui::view(&self)
    }

    fn theme(&self) -> Theme {
        self.theme.clone()
    }
}