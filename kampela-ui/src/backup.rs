//! Screen for seed phrase display

#[cfg(not(feature="std"))]
use alloc::vec::Vec;
use core::marker::PhantomData;
#[cfg(feature="std")]
use std::vec::Vec;

use embedded_graphics::{
    draw_target::DrawTarget,
    geometry::{Point, Size},
    pixelcolor::BinaryColor,
    mono_font::{
        ascii::FONT_8X13_BOLD,
        MonoTextStyle,
    },
    primitives::Rectangle,
    Drawable,
};
use embedded_text::{
    alignment::{HorizontalAlignment, VerticalAlignment},
    style::TextBoxStyleBuilder,
    TextBox,
};
use mnemonic_external::{AsWordList, WordListElement, WordSet, Bits11};

use crate::{display_def::*, message, platform::Platform, uistate::UnitScreen, widget::nav_bar::nav_bar::NavCommand};

use crate::widget::{view::{ViewScreen, View, Widget}, nav_bar::nav_bar::{NavBar, NAV_BAR_WIDGET}};

use crate::uistate::{EventResult, UpdateRequest};

const VERTICAL_GAP: u32 = 4;

const HEADER_WIDGET: Widget = Widget::new(
    Rectangle{
        top_left: Point{
            x: 0,
            y: VERTICAL_GAP as i32,
        },
        size: Size{
            width: SCREEN_SIZE_X,
            height: 24,
        }
    },
    SCREEN_ZERO
);

const BODY_TOP_LEFT: Point = Point{
    x: 0,
    y: (VERTICAL_GAP + HEADER_WIDGET.bounds.size.height + VERTICAL_GAP) as i32,
};
const BODY_WIDGET: Widget = Widget::new(
    Rectangle{
        top_left: BODY_TOP_LEFT,
        size: Size{
            width: SCREEN_SIZE_X,
            height: SCREEN_SIZE_Y - BODY_TOP_LEFT.y as u32 - NAV_BAR_WIDGET.bounds.size.height - VERTICAL_GAP ,
        }
    },
    SCREEN_ZERO
);

enum BackupState {
    ShowSeed,
    Message,
    Error,
    Storing,
}

pub struct Backup<P> where
    P: Platform
{
    state: BackupState,
    phrase: Vec<WordListElement<P::AsWordList>>,
    navbar: NavBar,
    prev_screen: UnitScreen,
    platform_type: PhantomData<P>,
}

impl<P: Platform> Backup<P> {
    pub fn new(e: Vec<u8>, prev_screen: UnitScreen) -> Self
    where <P as Platform>::AsWordList: Sized{
        let wordlist = P::get_wordlist();
        let phrase_result = WordSet::from_entropy(&e).map(|ws| {
        ws.bits11_set.iter().map(|&b| {
                WordListElement{ word: wordlist.get_word(b).unwrap(), bits11: b }
            }).collect()
        });
        let (state, phrase) = match phrase_result {
            Ok(w) => (BackupState::ShowSeed, w),
            Err(_) => (BackupState::Error, Vec::new())
        };
        Backup {
            state,
            phrase,
            navbar: NavBar::new(("back", "store")),
            prev_screen,
            platform_type: PhantomData::<P>::default(),
        }
    }

    pub fn get_entropy(&self) -> Option<Vec<u8>> {
        WordSet{
            bits11_set: self.phrase.iter().map(|w| w.bits11).collect::<Vec<Bits11>>()
        }
            .to_entropy()
            .ok()
    }
    
    fn draw_backup_screen<D: DrawTarget<Color = BinaryColor>>(&mut self, target: &mut D) -> Result<(), D::Error> {
        let character_style = MonoTextStyle::new(&FONT_8X13_BOLD, BinaryColor::On);
        let textbox_style = TextBoxStyleBuilder::new()
            .alignment(HorizontalAlignment::Center)
            .vertical_alignment(VerticalAlignment::Middle)
            .build();
    
        TextBox::with_textbox_style(
            "Please write down seed phrase",
            HEADER_WIDGET.bounds,
            character_style,
            textbox_style
        ).draw(target)?;
        TextBox::with_textbox_style(
            &self.phrase.iter().map(|w| w.word.as_ref()).collect::<Vec<&str>>().join(" "),
            BODY_WIDGET.bounds,
            character_style,
            textbox_style
        ).draw(target)?;
        self.navbar.draw(target, false)?;
        
        Ok(())
    }
}

impl<P: Platform> ViewScreen for Backup<P> {
    type DrawInput<'a> = () where P: 'a;
    type DrawOutput = Option<Vec<u8>>;
    type TapInput<'a> = () where P: 'a;
    type TapOutput = ();

    fn draw_screen<'a, D>(&mut self, target: &mut D, _: ()) -> Result<(EventResult, Self::DrawOutput), D::Error>
    where
        D: DrawTarget<Color = BinaryColor>,
        Self: 'a,
    {
        let mut request = None;
        let mut state = None;
        let mut entropy = None;
        
        match self.state {
            BackupState::ShowSeed => {
                self.draw_backup_screen(target)?;
            },
            BackupState::Message => {
                message::draw(target, "Storing into flash...", true)?;
                request = Some(UpdateRequest::Hidden);
                self.state = BackupState::Storing;
            },
            BackupState::Error => {
                message::draw(
                    target,
                    "System error! Seed storage corrupted; if this persists, please destroy the device",
                    true
                )?;
            },
            BackupState::Storing => {
                entropy = Some(self.get_entropy().unwrap());
                state = Some(UnitScreen::QRAddress);
                request = Some(UpdateRequest::Slow);
            },
        }

        Ok((EventResult { request, state }, entropy))
    }
    fn handle_tap_screen<'a>(&mut self, point: Point, _: ()) -> (EventResult, ()) 
    where
        Self: 'a
    {
        let mut state = None;
        let mut request = None;

        if matches!(self.state, BackupState::ShowSeed) {
            if let Some(Some(c)) = self.navbar.handle_tap(point, ()) {
                match c {
                    NavCommand::Left => {
                        state = Some(core::mem::take(&mut self.prev_screen));
                        request = Some(UpdateRequest::Fast);
                    },
                    NavCommand::Right => {
                        self.state = BackupState::Message;
                        request = Some(UpdateRequest::UltraFast);
                    }
                }
            }
        }

        (EventResult{ request, state }, ())
    }
}