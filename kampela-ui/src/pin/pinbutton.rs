#[cfg(not(feature="std"))]
use alloc::string::ToString;
#[cfg(feature="std")]
use std::string::ToString;

use embedded_graphics::{
	pixelcolor::BinaryColor,
	prelude::{DrawTarget, Point, Primitive, Dimensions, Size},
	Drawable,
	mono_font::{
        ascii::FONT_10X20,
        MonoTextStyle,
        MonoFont,
    },
    primitives::{
        CornerRadii, PrimitiveStyle, PrimitiveStyleBuilder, Rectangle, RoundedRectangle, StrokeAlignment,
    },
};

use embedded_text::{
    alignment::{HorizontalAlignment, VerticalAlignment},
    style::TextBoxStyleBuilder,
    TextBox,
};

use crate::widget::view::{View, Widget, DrawView};

const BUTTON_FONT: MonoFont = FONT_10X20;
const BUTTON_RADIUS: u32 = 6;
const BUTTON_BORDER_OFFSET: i32 = -2;

#[derive(Debug)]
pub struct PinButton {
	num: u8,
	widget: &'static Widget,
    this_tapped: bool,
}

impl PinButton {
	pub fn new(num: u8, widget: &'static Widget) -> Self {
		Self {
			num,
			widget,
            this_tapped: false,
		}
	}
    fn reset_tapped(&mut self) -> bool {
        if self.this_tapped {
            self.this_tapped = false;
            true
        } else {
            false
        }
    }
    pub fn num(&self) -> u8 {
        self.num
    }
}

impl View for PinButton {
    type DrawInput<'a> = bool;
    type DrawOutput = ();
    type TapInput<'a> = ();
    type TapOutput = bool;
    fn bounding_box(&self) -> Rectangle {
        self.widget.bounding_box()
    }
    fn bounding_box_absolut(&self) -> Rectangle {
        self.widget.bounding_box_absolute()
    }
	fn draw_view<'a, D: DrawTarget<Color = BinaryColor>>(&mut self, target: &mut DrawView<D>, t: Self::DrawInput<'_>) -> Result<Self::DrawOutput, D::Error>
    where Self: 'a {
        let this_tapped = self.reset_tapped();
        if this_tapped {
            self.draw_tapped(target)?;
        } else {
            self.draw_initial(target, t)?;
        }
        Ok(())
	}
    fn handle_tap_view<'a>(&mut self, _point: Point, _: ()) -> bool where Self: 'a, {
        self.this_tapped = true;
        true
    }
}

impl PinButton {
    fn draw_initial<D: DrawTarget<Color = BinaryColor>>(&self, target: &mut D, t: bool) -> Result<(), D::Error> {
        let (on, off) = if t {
            (BinaryColor::Off, BinaryColor::On)
        } else {
            (BinaryColor::On, BinaryColor::Off)
        };
        let filled = PrimitiveStyle::with_fill(off);
        let thin_stroke = PrimitiveStyleBuilder::new()
            .stroke_color(on)
            .stroke_width(2)
            .stroke_alignment(StrokeAlignment::Inside)
            .build();

        let bounds = self.bounding_box_view();
        let rounded = RoundedRectangle::new(
            bounds.offset(BUTTON_BORDER_OFFSET),
            CornerRadii::new(Size::new(BUTTON_RADIUS, BUTTON_RADIUS))
        );
        rounded.into_styled(filled).draw(target)?;
        rounded.into_styled(thin_stroke).draw(target)?;

        if t == false {
            let character_style = MonoTextStyle::new(&BUTTON_FONT, BinaryColor::On);
            let textbox_style = TextBoxStyleBuilder::new()
                .alignment(HorizontalAlignment::Center)
                .vertical_alignment(VerticalAlignment::Middle)
                .build();
    
            TextBox::with_textbox_style(
                &self.num.to_string(),
                bounds,
                character_style,
                textbox_style,
            )
            .draw(target)?;
        }

		Ok(())
    }
    fn draw_tapped<D: DrawTarget<Color = BinaryColor>>(&self, target: &mut DrawView<D>) -> Result<(), D::Error> {
        let filled = PrimitiveStyle::with_fill(BinaryColor::Off);

        let area = self.bounding_box_view();
        let rounded = RoundedRectangle::new(
            area.offset(BUTTON_BORDER_OFFSET),
            CornerRadii::new(Size::new(BUTTON_RADIUS, BUTTON_RADIUS))
        );
        rounded.into_styled(filled).draw(target)?;
    
        let character_style = MonoTextStyle::new(&BUTTON_FONT, BinaryColor::On);
        let textbox_style = TextBoxStyleBuilder::new()
            .alignment(HorizontalAlignment::Center)
            .vertical_alignment(VerticalAlignment::Middle)
            .build();
    
        TextBox::with_textbox_style(
            &self.num.to_string(),
            area,
            character_style,
            textbox_style,
        )
        .draw(target)?;
        Ok(())
    }
}