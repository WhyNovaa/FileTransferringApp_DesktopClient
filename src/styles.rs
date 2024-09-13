use iced::{Background, Border, Shadow, Theme, Vector};
use iced::widget::{button, container};

pub enum ButtonStyle {
    Standard,
    ThemeButton,
    Transparent,
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

pub struct ContainerStyle;

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

pub struct FileStyle;

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