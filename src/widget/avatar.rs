use iced::widget::{container, image, text};
use iced::{Background, Color, ContentFit, border};

use super::Element;
use crate::theme;

pub fn view<'a, M: 'a>(
    name: &str,
    accent: Color,
    size: f32,
    picture: Option<&image::Handle>,
) -> Element<'a, M> {
    if let Some(handle) = picture {
        return image(handle.clone())
            .width(size)
            .height(size)
            .content_fit(ContentFit::Cover)
            .border_radius(size / 2.0)
            .into();
    }
    container(
        text(initials(name))
            .size(size * 0.38)
            .font(theme::FONT_BOLD)
            .color(theme::colors().on_accent),
    )
    .center(size)
    .style(move |_theme| container::Style {
        background: Some(Background::Color(accent)),
        border: border::rounded(size / 2.0),
        ..container::Style::default()
    })
    .into()
}

fn initials(name: &str) -> String {
    name.split_whitespace()
        .filter_map(|word| word.chars().next())
        .take(2)
        .collect::<String>()
        .to_uppercase()
}
