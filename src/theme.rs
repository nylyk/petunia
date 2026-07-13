use iced::theme::Palette;
use iced::widget::{button, container, rule, text, text_input};
use iced::{Background, Border, Color, Font, Theme, border, font};

pub struct Colors {
    pub background: Color,
    pub surface: Color,
    pub sunken: Color,
    pub border: Color,
    pub text: Color,
    pub dim: Color,
    pub muted: Color,
    pub accent: Color,
    pub on_accent: Color,
    pub success: Color,
    pub warning: Color,
    pub danger: Color,
    pub accents: [Color; 8],
}

const DARK: Colors = Colors {
    background: Color::from_rgb8(0x18, 0x18, 0x25),
    surface: Color::from_rgb8(0x1e, 0x1e, 0x2e),
    sunken: Color::from_rgb8(0x11, 0x11, 0x1b),
    border: Color::from_rgb8(0x31, 0x32, 0x44),
    text: Color::from_rgb8(0xcd, 0xd6, 0xf4),
    dim: Color::from_rgb8(0x6c, 0x70, 0x86),
    muted: Color::from_rgb8(0x58, 0x5b, 0x70),
    accent: Color::from_rgb8(0x89, 0xb4, 0xfa),
    on_accent: Color::from_rgb8(0x18, 0x18, 0x25),
    success: Color::from_rgb8(0xa6, 0xe3, 0xa1),
    warning: Color::from_rgb8(0xf9, 0xe2, 0xaf),
    danger: Color::from_rgb8(0xf3, 0x8b, 0xa8),
    accents: [
        Color::from_rgb8(0x89, 0xb4, 0xfa),
        Color::from_rgb8(0xa6, 0xe3, 0xa1),
        Color::from_rgb8(0xf9, 0xe2, 0xaf),
        Color::from_rgb8(0xfa, 0xb3, 0x87),
        Color::from_rgb8(0xcb, 0xa6, 0xf7),
        Color::from_rgb8(0x94, 0xe2, 0xd5),
        Color::from_rgb8(0xf5, 0xc2, 0xe7),
        Color::from_rgb8(0xf3, 0x8b, 0xa8),
    ],
};

pub fn colors() -> &'static Colors {
    &DARK
}

pub const FONT_BOLD: Font = Font {
    weight: font::Weight::Bold,
    ..Font::MONOSPACE
};

pub fn accent(seed: &[u8]) -> Color {
    let colors = colors();
    let sum: usize = seed.iter().map(|byte| *byte as usize).sum();
    colors.accents[sum % colors.accents.len()]
}

pub fn dark() -> Theme {
    let colors = colors();
    Theme::custom(
        "Petunia",
        Palette {
            background: colors.background,
            text: colors.text,
            primary: colors.accent,
            success: colors.success,
            warning: colors.warning,
            danger: colors.danger,
        },
    )
}

pub fn pane(_theme: &Theme, focused: bool) -> container::Style {
    let colors = colors();
    container::Style {
        background: Some(Background::Color(colors.surface)),
        border: Border {
            width: 1.0,
            color: if focused { colors.accent } else { colors.border },
            radius: border::radius(9),
        },
        ..container::Style::default()
    }
}

pub fn separator(_theme: &Theme) -> rule::Style {
    rule::Style {
        color: colors().border,
        radius: border::radius(0),
        fill_mode: rule::FillMode::Full,
        snap: true,
    }
}

pub fn message_input(_theme: &Theme, status: text_input::Status) -> text_input::Style {
    let colors = colors();
    text_input::Style {
        background: Background::Color(colors.sunken),
        border: Border {
            width: 1.0,
            color: match status {
                text_input::Status::Focused { .. } => colors.accent,
                _ => colors.border,
            },
            radius: border::radius(6),
        },
        icon: colors.dim,
        placeholder: colors.dim,
        value: colors.text,
        selection: Color {
            a: 0.3,
            ..colors.accent
        },
    }
}

pub fn sidebar_entry(_theme: &Theme, status: button::Status) -> button::Style {
    let colors = colors();
    button::Style {
        background: match status {
            button::Status::Hovered | button::Status::Pressed => {
                Some(Background::Color(colors.border))
            }
            _ => None,
        },
        text_color: colors.text,
        border: border::rounded(6),
        ..button::Style::default()
    }
}

pub fn pane_control(_theme: &Theme, status: button::Status) -> button::Style {
    let colors = colors();
    button::Style {
        text_color: match status {
            button::Status::Hovered | button::Status::Pressed => colors.text,
            _ => colors.muted,
        },
        ..button::Style::default()
    }
}

pub fn unread_badge(_theme: &Theme) -> container::Style {
    let colors = colors();
    container::Style {
        background: Some(Background::Color(colors.danger)),
        text_color: Some(colors.on_accent),
        border: border::rounded(8),
        ..container::Style::default()
    }
}

pub fn error_banner(_theme: &Theme) -> container::Style {
    let colors = colors();
    container::Style {
        background: Some(Background::Color(colors.danger)),
        text_color: Some(colors.on_accent),
        ..container::Style::default()
    }
}

pub fn text_dim(_theme: &Theme) -> text::Style {
    text::Style {
        color: Some(colors().dim),
    }
}
