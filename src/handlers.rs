use std::collections::HashMap;
use reqwest::blocking::multipart;
use std::fs::{metadata, File};
use std::io::{copy, Read};
use std::path::Path;
use native_dialog::FileDialog;
use reqwest::blocking::multipart::Part;
use chrono::Utc;

use crate::app::{App, LoginField, Message, Page};
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
        Message::DeleteFile(index) => {
            delete_file_request(app, index);

            println!("{}", index);
        }
        Message::ToggleCheck(index) => {
            if let Some(row) = app.packages.get_mut(index) {
                row.checked = !row.checked;
            }
        }
        Message::SelectAll(checked) => {
            select_all(&mut app.packages, checked);
        }
        Message::DeleteSelected => {
            if delete_selected(app) {
                files_request(app);
            }
            else {
                println!("Delete files error");
            }
        }
        Message::Refresh => {
            files_request(app);
        }
        Message::DownloadFile(filename) => {
            download_request(app, filename);
        }
        Message::UploadFiles => {
            if upload_request(app) {
                files_request(app);
            }
            else {
                println!("Upload files error");
            }
        }
        Message::SearchFieldChanged(search) => {
            app.search_text = search;
            select_all(&mut app.packages, false);
        }
    }
}


pub fn is_token_expired(token_exp: i64) -> bool {
    let cur_time = Utc::now().timestamp();

    token_exp < cur_time
}
fn select_all(packages: &mut Vec<PackageRow>, checked: bool) {
    for package_row in packages {
        package_row.checked = checked;
    }
}

fn upload_request(app: &App) -> bool {
    let result = FileDialog::new()
        .set_location("~")
        .show_open_multiple_file();

    match result
    {
        Ok(file_paths) => {
            let mut form = multipart::Form::new();

            for file_path in file_paths {
                let path = Path::new(&file_path);

                let file_data = metadata(path).unwrap();
                if file_data.len() > 524288000 {
                    println!("File {:?} size is bigger than 500MB", file_path);
                    continue;
                }
                let file_name = path.file_name().unwrap_or_default().to_str().unwrap_or_default();

                let mut file = File::open(path).unwrap();
                let mut file_content = Vec::new();

                file.read_to_end(&mut file_content).unwrap();

                let part = Part::bytes(file_content).file_name(file_name.to_string());
                form = form.part("files", part);
            }

            let url = format!("{}/files/upload", app.server.url);
            let response = app.client.post(&url)
                .multipart(form)
                .header("Authorization", format!("Bearer {}", app.token))
                .send();

            println!("{:?}", response);

            response.unwrap().status().is_success()
        }
        Err(e) => {
            eprintln!("Error {}", e);
            false
        }

    }

}


fn download_request(app: &App, filename: String) -> bool {
    let result = FileDialog::new().set_location("~")
        .show_open_single_dir();

    match result {
        Ok(dir_path) => {
            if let Some(dir_path) = dir_path {
                let file_path = Path::new(&dir_path).join(filename.clone());

                let file = File::create(file_path);
                match file
                {
                    Ok(mut file) => {
                        let url = format!("{}/files/", app.server.url);

                        let response = app.client.get(&url)
                            .json(&filename)
                            .header("Authorization", format!("Bearer {}", app.token))
                            .send();

                        match response
                        {
                            Ok(response) => {
                                let content = response.bytes();

                                match content
                                {
                                    Ok(content) => {
                                        match copy(&mut content.as_ref(), &mut file)
                                        {
                                            Ok(_) => {
                                                true
                                            }
                                            Err(e) => {
                                                eprintln!("{}", e);
                                                false
                                            }
                                        }
                                    }
                                    Err(e) => {
                                        eprintln!("{}", e);
                                        false
                                    }
                                }

                            }

                            Err(e) => {
                                eprintln!("{}", e);
                                false
                            }
                        }

                    }

                    Err(e) => {
                        eprintln!("{}", e);
                        false
                    }
                }
            }
            else {
                eprintln!("Dir path error");
                false
            }
        }
        Err(e) => {
            eprintln!("{}", e);
            false
        }
    }
}
fn delete_selected(app: &App) -> bool {
    let mut file_list: Vec<String> = vec![];

    for package in &app.packages {
        if package.checked {
            file_list.push(package.filename.clone());
        }
    }
    if file_list.len() > 0
    {
        let url = format!("{}/files/", app.server.url);
        let response = app.client.delete(&url)
            .json(&file_list)
            .header("Authorization", format!("Bearer {}", app.token))
            .send();

        println!("{:?}", response);
        response.unwrap().status().is_success()
    }
    else {
        false
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

    match response
    {
        Ok(response) => {
            let json_result: Result<HashMap<String, String>, _> = response.json();
            if let Ok(json) = json_result {
                if let Some(token) = json.get("token") {
                    app.token = token.clone();
                    app.page = Page::Main;
                    if let Some(seconds) = json.get("jwt_exp_seconds") {
                        app.token_exp = Utc::now().timestamp() + seconds.parse::<i64>().expect("jwt seconds parse error");
                        app.login_error = Some(String::from("JWT expired, log in again"));
                    }
                    return true;
                }
            }

            app.login_error = Some(String::from("Wrong username or password"));
            false
        }
        Err(e) => {
            app.login_error = Some(String::from("Server connection error"));
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
            match resp.text()
            {
                Ok(data) => {
                    match serde_json::from_str::<Vec<String>>(&data)
                    {
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

        match response
        {
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


pub fn clear_login_field(login_field: &mut LoginField) {
    login_field.login = String::from("");
    login_field.password = String::from("");
}