//! Screen for seed phrase display

#[cfg(not(feature="std"))]
use alloc::vec::Vec;
#[cfg(feature="std")]
use std::vec::Vec;

use patches::entropy_to_phrase;
use embedded_graphics::{
    mono_font::{
        ascii::FONT_8X13_BOLD,
        MonoTextStyle,
    },
    primitives::Rectangle,
    Drawable,
};
use embedded_graphics_core::{
    draw_target::DrawTarget,
    geometry::{Point, Size},
    pixelcolor::BinaryColor,
};
use embedded_text::{
    alignment::{HorizontalAlignment, VerticalAlignment},
    style::TextBoxStyleBuilder,
    TextBox,
};

use crate::{display_def::*, message, uistate::UnitScreen};

use crate::widget::view::ViewScreen;

use crate::uistate::{EventResult, UpdateRequest};


enum BackupState {
    ShowSeed,
    Message,
    Storing,
}

pub struct Backup {
    state: BackupState,
    entropy: Vec<u8>,
}

impl Backup {
    pub fn new(e: Vec<u8>) -> Self {
        Self {
            state: BackupState::ShowSeed,
            entropy: e,
        }
    }
    
    fn draw_backup_screen<D: DrawTarget<Color = BinaryColor>>(&self, display: &mut D) -> Result<(), D::Error> {
        let character_style = MonoTextStyle::new(&FONT_8X13_BOLD, BinaryColor::On);
        let textbox_style = TextBoxStyleBuilder::new()
            .alignment(HorizontalAlignment::Center)
            .vertical_alignment(VerticalAlignment::Middle)
            .build();
        let header = Rectangle::new(Point::new(0, 4), Size::new(SCREEN_SIZE_X, 24));
        let body = Rectangle::new(Point::new(0, 28), Size::new(SCREEN_SIZE_X, 100));
        let bottom = Rectangle::new(Point::new(0, 132), Size::new(SCREEN_SIZE_X, 50));
    
        match entropy_to_phrase(&self.entropy) {
            Ok(ref seed) => {
                TextBox::with_textbox_style("Please write down seed phrase", header, character_style, textbox_style).draw(display)?;
                TextBox::with_textbox_style(seed, body, character_style, textbox_style).draw(display)?;
                TextBox::with_textbox_style("touch the screen when done", bottom, character_style, textbox_style).draw(display)?;
            },
            Err(_e) => {
                TextBox::with_textbox_style("System error! Seed storage corrupted; if this persists, please destroy the device", body, character_style, textbox_style).draw(display)?;
            },
        };
        
        Ok(())
    }
}

impl ViewScreen for Backup {
    type DrawInput<'a> = ();
    type DrawOutput = Option<Vec<u8>>;
    type TapInput<'a> = ();
    type TapOutput = ();

    fn draw_screen<'a, D>(&mut self, target: &mut D, _: ()) -> Result<(EventResult, Self::DrawOutput), D::Error>
    where
        D: DrawTarget<Color = BinaryColor>,
    {
        let mut request = UpdateRequest::new();
        let mut state = None;
        let mut entropy = None;
        
        match self.state {
            BackupState::ShowSeed => {
                self.draw_backup_screen(target)?;
            },
            BackupState::Message => {
                message::draw(target, "Storing into flash...")?;
                request.set_hidden();
                self.state = BackupState::Storing;
            },
            BackupState::Storing => {
                entropy = Some(self.entropy.clone());
                state = Some(UnitScreen::QRAddress);
                request.set_slow();
            },
        }

        Ok((EventResult { request, state }, entropy))
    }
    fn handle_tap_screen<'a>(&mut self, _: Point, _: ()) -> (EventResult, ()) {
        let state = None;
        let mut request = UpdateRequest::new();

        if matches!(self.state, BackupState::ShowSeed) {
            self.state = BackupState::Message;
            request.set_ultrafast();
        }

        (EventResult{ request, state }, ())
    }
}