use embedded_graphics::{
	pixelcolor::BinaryColor,
	prelude::{DrawTarget, Point, Dimensions},
	Drawable,
	mono_font::{
        ascii::FONT_10X20,
        MonoTextStyleBuilder,
        MonoFont,
    },
    primitives::Rectangle,
};

use embedded_text::{
    alignment::{HorizontalAlignment, VerticalAlignment},
    style::TextBoxStyleBuilder,
    TextBox,
};

use crate::widget::view::{View, Widget, DrawView};

const BUTTON_FONT: MonoFont = FONT_10X20;

pub struct NavButton {
	label: &'static str,
	widget: &'static Widget,
}

impl NavButton {
	pub fn new(label: &'static str, widget: &'static Widget) -> Self {
		Self {
			label,
			widget,
		}
	}
}

impl View for NavButton {
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
	fn draw_view<D: DrawTarget<Color = BinaryColor>>(&mut self, target: &mut DrawView<D>, t: Self::DrawInput<'_>) -> Result<Self::DrawOutput, D::Error> {
        let (on, _) = if t {
            (BinaryColor::Off, BinaryColor::On)
        } else {
            (BinaryColor::On, BinaryColor::Off)
        };
        let area = self.bounding_box_view();

        let character_style = MonoTextStyleBuilder::new()
            .font(&BUTTON_FONT)
            .text_color(on)
            .underline()
            .build();
        let textbox_style = TextBoxStyleBuilder::new()
            .alignment(HorizontalAlignment::Center)
            .vertical_alignment(VerticalAlignment::Middle)
            .build();

        TextBox::with_textbox_style(
            self.label,
            area,
            character_style,
            textbox_style,
        )
        .draw(target)?;

        Ok(())
	}
    fn handle_tap_view(&mut self, _point: Point, _: ()) -> bool {
        true
    }
}