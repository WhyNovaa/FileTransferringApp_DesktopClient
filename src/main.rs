mod app;
mod utils;
mod handlers;
mod ui;
mod styles;

use iced::{window, Sandbox, Size};
use iced::{Settings};


use crate::app::App;
use crate::utils::load_icon;

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





