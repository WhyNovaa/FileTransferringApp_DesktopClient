use iced::{Background, Border, Color, Shadow, Theme, Vector};
use iced::widget::{button, container};

pub enum ButtonStyle {
    Standard,
    ThemeButton,
    Transparent,
    DeleteButton,
}

impl button::StyleSheet for ButtonStyle {
    type Style = Theme;

    fn active(&self, theme: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: match self {
                Self::Standard => Some(Background::Color(Color::from_rgb(0.059, 0.463, 0.702))),
                Self::ThemeButton => Some(Background::Color(Color::default())),
                Self::Transparent => None,
                Self::DeleteButton => Some(Background::Color(Color::from_rgb(0.9, 0.1, 0.3))),
            },
            border: match self {
                Self::Standard => Border::with_radius(5),
                Self::ThemeButton => Border::default(),
                Self::Transparent => Border::with_radius(100),
                Self::DeleteButton => Border::with_radius(5),
            },
            shadow_offset: match self {
                Self::Standard => Vector::new(0.0, 2.0),
                Self::ThemeButton => Vector::new(0.0, 0.0),
                Self::Transparent => Vector::new(0.0, 0.0),
                Self::DeleteButton => Vector::new(0.0, 2.0),
            },
            shadow: match self {
                Self::Standard => Shadow {
                    color: Color::BLACK,
                    offset: Vector::new(0.0, 4.0),
                    blur_radius: 20.0,
                },
                Self::ThemeButton => Shadow::default(),
                Self::Transparent => Shadow::default(),
                Self::DeleteButton => Shadow {
                    color: Color::BLACK,
                    offset: Vector::new(0.0, 4.0),
                    blur_radius: 20.0,
                },
            },
            text_color: {
                if theme == &Theme::Light {
                    match self {
                        Self::Standard => Color::WHITE,
                        Self::ThemeButton => Color::BLACK,
                        Self::Transparent => Color::BLACK,
                        Self::DeleteButton => Color::WHITE,
                    }
                } else {
                    match self {
                        Self::Standard => Color::WHITE,
                        Self::ThemeButton => Color::WHITE,
                        Self::Transparent => Color::WHITE,
                        Self::DeleteButton => Color::WHITE,
                    }
                }
            },
        }

    }
    fn hovered(&self, theme: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: match self {
                Self::Standard => Some(Background::Color(Color::from_rgb(0.4, 0.7, 1.0))),
                Self::ThemeButton => None,
                Self::Transparent => None,
                Self::DeleteButton => Some(Background::Color(Color::from_rgb(0.98, 0.6, 0.6))),
            },
            ..self.active(theme)
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
                color: Color::BLACK,
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
                color: Color::BLACK,
                offset: Vector::new(0.0, 2.0),
                blur_radius: 40.0,
            },
        }
    }
}

