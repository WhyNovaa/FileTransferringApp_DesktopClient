use std::collections::HashMap;

use crate::app::{App, Message, Page};
use crate::ui::PackageRow;

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
            if log_in_request(app) {
                files_request(app);
            }


        }
        Message::LoginFieldChanged(login, password) => {
            app.login_field.login = login;
            app.login_field.password = password;
        }
        Message::DeleteFileClicked(index) => {
            delete_file_request(app, index);

            println!("{}", index);
        }
        Message::EditFileClicked(index) => {
            println!("{}", index);
        }
        Message::ToggleCheck(index) => {
            if let Some(row) = app.packages.get_mut(index) {
                row.checked = !row.checked;
            }
        }
        Message::SelectAll(checked) => {
            for package_row in &mut app.packages {
                package_row.checked = checked;
            }
        }
        Message::DeleteSelected => {

        }
        Message::Refresh => {
            files_request(app);
        }

    }
}

pub fn log_in_request(app: &mut App) -> bool {
    let params = [
        ("username", app.login_field.login.as_str()),
        ("password", app.login_field.password.as_str())
    ];

    let url = format!("{}/login", app.server.url);

    let response = app.client.post(&url)
        .form(&params)
        .send();

    match response {
        Ok(response) => {
            let json_result: Result<HashMap<String, String>, _> = response.json();
            if let Ok(json) = json_result {
                if let Some(token) = json.get("token") {
                    app.token = token.clone();
                    app.page = Page::Main;
                    return true;
                }
            }

            app.login_error = Some("Wrong username or password".to_string());
            false
        }
        Err(e) => {
            app.login_error = Some("Server connection error".to_string());
            eprintln!("Error sending request: {}", e);
            false
        }
    }
}

pub fn files_request(app: &mut App) {
    let response = app.client
        .get(format!("{}/files/", app.server.url))
        .header("Authorization", format!("Bearer {}", app.token))
        .send();

    match response {
        Ok(resp) => {
            match resp.text() {
                Ok(data) => {
                    match serde_json::from_str::<Vec<String>>(&data) {
                        Ok(files) => {
                            app.packages = files
                                .into_iter()
                                .map(PackageRow::new)
                                .collect();
                        }
                        Err(err) => {
                            eprintln!("Ошибка при разборе JSON: {}", err);
                            app.packages = vec![];
                        }
                    }
                }
                Err(err) => {
                    eprintln!("Ошибка при получении текста ответа: {}", err);
                    app.packages = vec![];
                }
            }
        }
        Err(err) => {
            eprintln!("Ошибка при выполнении запроса: {}", err);
            app.packages = vec![];
        }
    }
}


pub fn delete_file_request(app: &mut App, index: usize) {
    if index < app.packages.len() {
        let package_row = &app.packages[index];
        let filename = &package_row.filename;

        let response = app.client
            .delete(format!("{}/files/{}", app.server.url, filename))
            .header("Authorization", format!("Bearer {}", app.token))
            .send();

        match response {
            Ok(response) => {
                if response.status().is_success() {
                    app.packages.remove(index);
                    println!("File deleted successfully");
                } else {
                    eprintln!("Failed to delete file. Status: {}", response.status());
                }
            }
            Err(err) => {
                eprintln!("Error sending delete request: {}", err);
            }
        }
    }
    else {
        eprintln!("Index out of bounds");
    }
}