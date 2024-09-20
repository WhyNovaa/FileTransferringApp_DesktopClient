use std::env;
use dotenv::dotenv;
use iced::{Element, Sandbox, Theme};
use reqwest::blocking::Client;

use crate::ui;
use crate::handlers::{handle_update, is_token_expired, clear_login_field};

pub struct App {
    pub theme: Theme,
    pub page: Page,
    pub login_field: LoginField,
    pub token: String,
    pub token_exp: i64,
    pub client: Client,
    pub login_error: Option<String>,
    pub packages: Vec<ui::PackageRow>,
    pub server: Server,
    pub search_text: String,
}

pub struct LoginField {
    pub login: String,
    pub password: String,
}

pub struct Server {
    pub url: String,
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
    DeleteFile(usize),
    ToggleCheck(usize),
    SelectAll(bool),
    DeleteSelected,
    Refresh,
    DownloadFile(String),
    UploadFiles,
    SearchFieldChanged(String),
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
                password: String::new(),
            },
            token: String::new(),
            token_exp: 0,
            client: Client::new(),
            login_error: None,
            packages: (1..=100)
                .map(|_| ui::PackageRow::new("filename".to_string()))
                .collect(),
            server: Server {
                url: env::var("SERVER_URL").expect("SERVER_URL must be set").to_string(),
            },
            search_text: String::new(),
        }
    }

    fn title(&self) -> String {
        String::from("FTA")
    }

    fn update(&mut self, message: Message) {
        if self.page == Page::Login {
            handle_update(self, message);
        }
        else if is_token_expired(self.token_exp) {
            self.page = Page::Login;
            clear_login_field(&mut self.login_field);
        }
        else {
            handle_update(self, message);
        }
    }

    fn view(&self) -> Element<Message> {
        ui::view(self)
    }

    fn theme(&self) -> Theme {
        self.theme.clone()
    }
}