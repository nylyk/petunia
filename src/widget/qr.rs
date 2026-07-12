use iced::widget::image;
use qrcode::{Color, QrCode};

const MODULE_SIZE: usize = 6;
const QUIET_ZONE: usize = 4;

pub fn handle(data: &str) -> Option<image::Handle> {
    let code = QrCode::new(data).ok()?;
    let width = code.width();
    let size = (width + 2 * QUIET_ZONE) * MODULE_SIZE;
    let mut pixels = vec![0xff_u8; size * size * 4];

    for (index, color) in code.to_colors().into_iter().enumerate() {
        if color != Color::Dark {
            continue;
        }
        let x = (index % width + QUIET_ZONE) * MODULE_SIZE;
        let y = (index / width + QUIET_ZONE) * MODULE_SIZE;
        for row in y..y + MODULE_SIZE {
            for column in x..x + MODULE_SIZE {
                let offset = (row * size + column) * 4;
                pixels[offset..offset + 3].fill(0x00);
            }
        }
    }

    Some(image::Handle::from_rgba(size as u32, size as u32, pixels))
}
