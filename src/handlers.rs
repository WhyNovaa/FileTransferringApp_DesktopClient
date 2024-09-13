use std::collections::HashMap;
use crate::app::{App, Message, Page};

pub fn handle_update(app: &mut App, message: Message) {
    match message {
        Message::ToggleTheme => {
            app.theme = if app.theme == iced::Theme::Light {
                iced::Theme::Dark
            } else {
                iced::Theme::Light
            };
        }
        Message::LoginSubmit => {
            log_in_request(app);
        }
        Message::LoginFieldChanged(login, password) => {
            app.login_field.login = login;
            app.login_field.password = password;
        }
        Message::DeleteFileClicked(filename) => {
            println!("{}", filename);
        }
        Message::EditFileClicked(filename) => {
            println!("{}", filename);
        }
        Message::ToggleCheck(index) => {
            if let Some(row) = app.packages.get_mut(index) {
                row.checked = !row.checked;
            }
        }
    }
}


pub fn log_in_request(app: &mut App) {
    let mut params = HashMap::new();
    params.insert("username", app.login_field.login.to_string());
    params.insert("password", app.login_field.password.to_string());

    let result = app.client.post(app.server.url.to_string() + "/login")
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