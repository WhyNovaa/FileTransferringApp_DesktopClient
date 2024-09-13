mod app;
mod styles;
mod utils;
mod ui;
mod handlers;

use iced::{window, Sandbox, Size};
use iced::{Settings};


use crate::app::App;

fn main() -> iced::Result {

    let mut settings = Settings::default();

    let window_settings = window::Settings {
        min_size: Some(Size::new(700.0, 600.0)),
        icon: Some(utils::load_icon("src/resources/icon.ico")),
        ..window::Settings::default()
    };


    settings.window = window_settings;
    App::run(settings)
}





