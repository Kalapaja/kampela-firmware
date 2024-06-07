#[cfg(not(feature="std"))]
use alloc::{string::String, boxed::Box};
#[cfg(feature="std")]
use std::{string::String, boxed::Box};

use embedded_graphics::{
    draw_target::DrawTarget,
    pixelcolor::BinaryColor,
    geometry::Point,
    mono_font::{
        ascii::FONT_6X10,
        MonoTextStyle,
    },
    primitives::{Primitive, PrimitiveStyle},
    Drawable
};

use embedded_text::{
    alignment::HorizontalAlignment,
    style::{HeightMode, TextBoxStyleBuilder},
    TextBox,
};

use crate::widget::{nav_bar::nav_bar::{NavBar, NavCommand}, view::{View, ViewScreen}};
use crate::uistate::{EventResult, UpdateRequest, UnitScreen};

#[derive(Clone)]
pub enum TransactionPage {
    Call,
    Extension,
}

pub struct Transaction {
    page: TransactionPage,
    navbar: NavBar,
}

impl Transaction {
    pub fn new(page: TransactionPage) -> Self {
        let navbar = match page {
            TransactionPage::Call => NavBar::new(("", "next")),
            TransactionPage::Extension => NavBar::new(("previous", "sign")),
        };
        Transaction {
            page,
            navbar,
        }
    }
    pub fn get_page(&self) -> TransactionPage {
        self.page.clone()
    }
}

impl ViewScreen for Transaction {
    type DrawInput<'a> = Box<dyn FnOnce(&TransactionPage) -> String + 'a>;
    type DrawOutput = ();
    type TapInput<'a> = ();
    type TapOutput = ();

    fn draw_screen<'a, D>(&mut self, target: &mut D, get_content: Self::DrawInput<'a>) -> Result<(EventResult, ()), D::Error>
    where
        D: DrawTarget<Color = BinaryColor>,
        Self: 'a,
    {
        let state = None;
        let request = None;

        let filled = PrimitiveStyle::with_fill(BinaryColor::Off);
        let character_style = MonoTextStyle::new(&FONT_6X10, BinaryColor::On);
        let textbox_style = TextBoxStyleBuilder::new()
            .height_mode(HeightMode::FitToText)
            .alignment(HorizontalAlignment::Left)
            .paragraph_spacing(5)
            .build();
        
        let area = target.bounding_box();
        area.into_styled(filled).draw(target)?;
        
        TextBox::with_textbox_style(
            &get_content(&self.page),
            area,
            character_style,
            textbox_style
        ).draw(target)?;

        self.navbar.draw(target, false)?;
        Ok((EventResult{state, request}, ()))
    }

    fn handle_tap_screen<'a>(&mut self, point: Point, _: Self::TapInput<'a>) -> (EventResult, ())
    where
        Self: 'a
    {
        let mut state = None;
        let mut request = None;

        if let Some(Some(c)) = self.navbar.handle_tap(point, ()) {
            match self.page {
                TransactionPage::Call => {
                    match c {
                        NavCommand::Left => {},
                        NavCommand::Right => {
                            self.page = TransactionPage::Extension;
                            self.navbar = NavBar::new(("previous", "sign"));
                            request = Some(UpdateRequest::Fast);
                        }
                    }
                },
                TransactionPage::Extension => {
                    match c {
                        NavCommand::Left => {
                            self.page = TransactionPage::Call;
                            self.navbar = NavBar::new(("", "next"));
                            request = Some(UpdateRequest::Fast);
                        },
                        NavCommand::Right => {
                            state = Some(UnitScreen::ShowDialog(
                                "Sign the transaction?",
                                ("no", "yes"),
                                (
                                    Box::new(|| EventResult {
                                        request: Some(UpdateRequest::UltraFast),
                                        state: Some(UnitScreen::ShowTransaction(TransactionPage::Extension))
                                    }),
                                    Box::new(|| EventResult {
                                        request: Some(UpdateRequest::UltraFast),
                                        state: Some(UnitScreen::QRSignature)
                                    }),
                                ),
                                true
                            ));
                            request = Some(UpdateRequest::UltraFast);
                        }
                    }
                },
            }
        }
        (EventResult{state, request}, ())
    }
}
