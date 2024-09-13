use iced::{Alignment, Element, Length, Padding};
use iced::alignment::{Horizontal, Vertical};
use iced::widget::{button, container, text, Button, Checkbox, Column, Container, Image, Row, Scrollable, Space, TextInput};

use crate::styles::{ContainerStyle, ButtonStyle, FileStyle};
use crate::app::{App, LoginField, Message, Page};

pub fn view(app: &App) -> Element<Message> {
    let content =
        match app.page {
            Page::Login => log_in_page(&app.login_field, app.login_error.clone()),
            Page::Main => main_page(&app)
        };


    let wrapper =  Column::new();

    let wrapper =
        match app.page {
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

    let container = match app.page {
        Page::Login => temp_container.center_y(),
        Page::Main => temp_container.align_y(Vertical::Top),
    };
    container.width(Length::Fill).height(Length::Fill).into()
}

#[derive(Debug, Clone)]
pub struct PackageRow {
    pub checked: bool,
    pub filename: String,
}

impl PackageRow {
    pub fn new(filename: String) -> Self {
        PackageRow {
            checked: false,
            filename,
        }
    }
    pub fn view(&self, index: usize) -> Container<'static, Message> {
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

pub fn page_footer() -> Container<'static, Message> {
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
pub fn log_in_page(login_field: &LoginField, login_error: Option<String>) -> Container<Message> {
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
        .push(submit_btn("Log In", Message::LoginSubmit))
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

pub fn main_page(app: &App) -> Container<'static, Message> {
    let mut column = Column::new()
        .width(Length::Fill)
        .spacing(15);

    column = column.push(Space::with_height(0));


    for (index, package) in app.packages.iter().enumerate() {
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

pub fn input_field(_placeholder: &str, _value: &str, ) -> TextInput<'static, Message> {
    TextInput::new(_placeholder, _value)
        .width(Length::Fixed(500.0))
        .padding(Padding::from(10))
        .line_height(text::LineHeight::Relative(1.75))
}

pub fn del_btn(event: Message) -> Button<'static, Message> {
    let image = Image::new("src/resources/delete.png");

    Button::new(image)
        .on_press(event)
        .width(32)
        .height(32)
        .style(iced::theme::Button::Custom(Box::new(ButtonStyle::Transparent)))
}

pub fn edit_btn(event: Message) -> Button<'static, Message> {
    let image = Image::new("src/resources/edit.png");

    Button::new(image)
        .on_press(event)
        .width(32)
        .height(32)
        .style(iced::theme::Button::Custom(Box::new(ButtonStyle::Transparent)))
}


pub fn submit_btn(name: &str, event: Message) -> Button<Message> {
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
