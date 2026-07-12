use iced::theme::Palette;
use iced::widget::container;
use iced::{Background, Border, Color, Theme};

pub fn dark() -> Theme {
    Theme::custom(
        "Petunia",
        Palette {
            background: Color::from_rgb8(0x1e, 0x1b, 0x22),
            text: Color::from_rgb8(0xe4, 0xe1, 0xe7),
            primary: Color::from_rgb8(0xc0, 0x83, 0xdc),
            success: Color::from_rgb8(0xa6, 0xd1, 0x89),
            warning: Color::from_rgb8(0xe5, 0xc8, 0x90),
            danger: Color::from_rgb8(0xe7, 0x82, 0x84),
        },
    )
}

pub fn sidebar(theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(
            theme.extended_palette().background.weak.color,
        )),
        ..container::Style::default()
    }
}

pub fn error_banner(theme: &Theme) -> container::Style {
    let palette = theme.extended_palette();
    container::Style {
        background: Some(Background::Color(palette.danger.weak.color)),
        text_color: Some(palette.danger.weak.text),
        ..container::Style::default()
    }
}

pub fn pane_title_bar(theme: &Theme, focused: bool) -> container::Style {
    let palette = theme.extended_palette();
    let (background, text) = if focused {
        (palette.primary.weak.color, palette.primary.weak.text)
    } else {
        (palette.background.weak.color, palette.background.weak.text)
    };
    container::Style {
        background: Some(Background::Color(background)),
        text_color: Some(text),
        ..container::Style::default()
    }
}

pub fn pane_body(theme: &Theme, focused: bool) -> container::Style {
    let palette = theme.extended_palette();
    container::Style {
        background: Some(Background::Color(palette.background.base.color)),
        border: Border {
            width: 1.0,
            color: if focused {
                palette.primary.strong.color
            } else {
                palette.background.strong.color
            },
            ..Border::default()
        },
        ..container::Style::default()
    }
}
