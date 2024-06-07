use embedded_graphics::{
    draw_target::DrawTarget,
    geometry::Point,
    pixelcolor::BinaryColor,
    Pixel,
    Drawable,
    primitives::{Primitive, PrimitiveStyle},
};

use crate::display_def::*;
use qrcodegen_noheap::{QrCode, QrCodeEcc, Version};

pub fn draw<D>(data_to_qr: &[u8], display: &mut D) -> Result<(), D::Error>
where
    D: DrawTarget<Color = BinaryColor>,
{
    let len = data_to_qr.len();

    let mut outbuffer = [0u8; Version::new(18).buffer_len()].to_vec();
    let mut dataandtemp = [0u8; Version::new(18).buffer_len()].to_vec();
    
    dataandtemp[..len].copy_from_slice(data_to_qr);
    
    let qr_code = QrCode::encode_binary(&mut dataandtemp, len, &mut outbuffer, QrCodeEcc::Low, Version::MIN, Version::new(18), None, true).unwrap();

    let scaling = {
        if qr_code.version() == Version::new(18) {2}
        else {SCREEN_SIZE_Y as i32/qr_code.size()}
    };

    let filled = PrimitiveStyle::with_fill(BinaryColor::Off);
    display.bounding_box().into_styled(filled).draw(display)?;

    let size = qr_code.size() * scaling;
    for y in 0..size {
        for x in 0..size {
            let color = {
                if qr_code.get_module(x / scaling, y / scaling) {BinaryColor::On}
                else {BinaryColor::Off}
            };
            let x_point = SCREEN_SIZE_X as i32/2 - size/2 + x;
            let y_point = SCREEN_SIZE_Y as i32/2 - size/2 + y;
            let point = Point::new(x_point, y_point);
            let pixel = Pixel::<BinaryColor>(point, color);
            pixel.draw(display)?;
        }
    }
    Ok(())
}

