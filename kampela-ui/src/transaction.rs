#[cfg(not(feature="std"))]
use alloc::{string::String, boxed::Box, vec::Vec};
#[cfg(feature="std")]
use std::{string::String, boxed::Box, vec::Vec};

use embedded_graphics::{
    draw_target::DrawTarget, geometry::Dimensions, mono_font::{
        ascii::FONT_6X10, MonoFont, MonoTextStyle
    }, pixelcolor::BinaryColor, prelude::{Point, Size}, primitives::{Line, Primitive, PrimitiveStyle, PrimitiveStyleBuilder, Rectangle, StrokeAlignment}, Drawable
};

use embedded_text::{
    alignment::HorizontalAlignment,
    style::{HeightMode, TextBoxStyleBuilder},
    TextBox,
};
use kampela_display_common::display_def::{SCREEN_SIZE_X, SCREEN_SIZE_Y, SCREEN_ZERO};
use crate::parser::{count_lines, scroll_str};
use crate::platform::Platform;
use crate::widget::{
    nav_bar::nav_bar::{NavBar, NavCommand, NAV_BAR_SIZE},
    view::{DrawView, View, ViewScreen, Widget}
};
use crate::uistate::{EventResult, UpdateRequest, UnitScreen, Event};
const TEXT_FONT: &MonoFont = &FONT_6X10;
const MAX_CHARS: usize = ((SCREEN_SIZE_X + TEXT_FONT.character_spacing) / (TEXT_FONT.character_size.width + TEXT_FONT.character_spacing)) as usize;
const PARAGRAPH_SPACING: u32 = 5;
const SCROLL_SCREEN: usize = (SCROLL_AREA.size.height / (TEXT_FONT.character_size.height + PARAGRAPH_SPACING)) as usize;
const SCROLL_STEP: usize = SCROLL_SCREEN / 2;
const TAB_SIZE: u16 = 2;

#[derive(Clone)]
pub enum TransactionPage {
    //Call,
    //Extension,
    Eth,
    EthPayload
}
impl Default for TransactionPage {
    fn default() -> Self {
        TransactionPage::Eth
    }
}

pub struct Transaction<P: Platform> {
    page: TransactionPage,
    navbar: NavBar,
    content_getter: Box<dyn Fn(&TransactionPage, &mut P) -> String>,
    content: String,
    scroll: ScrollWidget,
}

impl<P: Platform> Transaction<P> {
    pub fn new(page: TransactionPage, content_getter: Box<dyn Fn(&TransactionPage, &mut P) -> String>, platform: &mut P) -> Self {
        let navbar = match page {
            //TransactionPage::Call => NavBar::new(("", "next")),
            //TransactionPage::Extension => NavBar::new(("previous", "sign")),
            TransactionPage::Eth => NavBar::new(("", "next")),
            TransactionPage::EthPayload => NavBar::new(("previous", "sign"))
        };
        let mut transaction = Transaction {
            page: TransactionPage::default(),
            navbar,
            content_getter,
            content: String::default(),
            scroll: ScrollWidget::new()
        };
        transaction.change_page(page, platform);
        transaction
    }
    pub fn get_page(&self) -> TransactionPage {
        self.page.clone()
    }
    pub fn change_page(&mut self, page: TransactionPage, platform: &mut P) {
        self.page = page;
        self.scroll.scroll = 0;
        self.content = (self.content_getter)(&self.page, platform);
        self.scroll.scroll_max = max_lines_to_scroll(
            &self.content,
            MAX_CHARS,
            TAB_SIZE,
            TEXT_FONT.character_size.height,
            PARAGRAPH_SPACING,
            SCROLL_AREA.size.height
        );
        self.scroll.showing = true;
    }
}

impl<P: Platform> ViewScreen for Transaction<P> {
    type DrawInput<'a> = () where Self: 'a;
    type DrawOutput = ();
    type EventInput<'a> = &'a mut P where Self: 'a;
    type EventOutput = ();

    fn draw_screen<'a, D>(&mut self, target: &mut D, _: Self::DrawInput<'a>) -> Result<(EventResult, ()), D::Error>
    where
        D: DrawTarget<Color = BinaryColor>,
        Self: 'a,
    {
        let state = None;
        let mut request = None;

        let filled = PrimitiveStyle::with_fill(BinaryColor::Off);
        let character_style = MonoTextStyle::new(TEXT_FONT, BinaryColor::On);
        let textbox_style = TextBoxStyleBuilder::new()
            .height_mode(HeightMode::FitToText)
            .alignment(HorizontalAlignment::Left)
            .paragraph_spacing(PARAGRAPH_SPACING)
            .tab_size(embedded_text::style::TabSize::Spaces(TAB_SIZE))
            .build();
        
        let area = target.bounding_box();
        area.into_styled(filled).draw(target)?;
        
        TextBox::with_textbox_style(
            scroll_str(&self.content, MAX_CHARS, TAB_SIZE, self.scroll.scroll),
            area,
            character_style,
            textbox_style
        ).draw(target)?;

        let areas = self.scroll.draw(target, ())?;
        if !areas.is_empty() {
            request = Some(UpdateRequest::Part(areas.to_vec()));
        };

        self.navbar.draw(target, false)?;
        Ok((EventResult{state, request}, ()))
    }

    fn handle_event_screen<'a>(&mut self, event: Event, platform: Self::EventInput<'a>) -> (EventResult, ())
    where
        Self: 'a
    {
        let mut state = None;
        let mut request = None;

        if self.scroll.handle_event(event, ()) == Some(true) {
            request = Some(UpdateRequest::UltraFastSelective);
        };

        if let Some(Some(c)) = self.navbar.handle_event(event, ()) {
            match self.page {/*
                TransactionPage::Call => {
                    match c {
                        NavCommand::Left => {},
                        NavCommand::Right => {
                            self.change_page(TransactionPage::Extension, platform);
                            self.navbar = NavBar::new(("previous", "sign"));
                            request = Some(UpdateRequest::Fast);
                        }
                    }
                },
                TransactionPage::Extension => {
                    match c {
                        NavCommand::Left => {
                            self.change_page(TransactionPage::Call, platform);
                            self.navbar = NavBar::new(("", "next"));
                            request = Some(UpdateRequest::Fast);
                        },
                        NavCommand::Right => {
                            state = Some(UnitScreen::ShowDialog((
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
                            )));
                            request = Some(UpdateRequest::UltraFast);
                        }
                    }
                },*/
                TransactionPage::Eth => {
                    match c {
                        NavCommand::Left => {},
                        NavCommand::Right => {
                            self.change_page(TransactionPage::EthPayload, platform);
                            self.navbar = NavBar::new(("previous", "sign"));
                            request = Some(UpdateRequest::Fast);
                        }
                    }
                },
                TransactionPage::EthPayload => {
                    match c {
                        NavCommand::Left => {
                            self.change_page(TransactionPage::Eth, platform);
                            self.navbar = NavBar::new(("", "next"));
                            request = Some(UpdateRequest::Fast);
                        },
                        NavCommand::Right => {
                            state = Some(UnitScreen::ShowDialog((
                                "Sign the transaction?",
                                ("no", "yes"),
                                (
                                    Box::new(|| EventResult {
                                        request: Some(UpdateRequest::UltraFast),
                                        state: Some(UnitScreen::ShowTransaction(TransactionPage::EthPayload))
                                    }),
                                    Box::new(|| EventResult {
                                        request: Some(UpdateRequest::UltraFast),
                                        state: Some(UnitScreen::QRSignature)
                                    }),
                                ),
                                true
                            )));
                            request = Some(UpdateRequest::UltraFast);
                        }
                    }
                }
            }
        }
        (EventResult{state, request}, ())
    }
}

const STROKE_WIDTH: u32 = 3;
const SCROLL_AREA: Rectangle = Rectangle{
    top_left: SCREEN_ZERO,
    size: Size{
        width: SCREEN_SIZE_X,
        height: SCREEN_SIZE_Y - NAV_BAR_SIZE.height,
    },
};

const SCROLL_WIDGET: Widget = Widget::new(SCROLL_AREA, SCREEN_ZERO);

const SCROLL_BUTTON_SIZE: Size = Size {
    width: 26,
    height: 10,
};
const SCROLL_BUTTON_TAP_SIZE: Size = Size {
    width: SCROLL_AREA.size.width,
    height: SCROLL_AREA.size.height / 2
};
const SCROLL_BUTTON_UP_TAP_AREA: Rectangle = Rectangle {
    top_left: Point::zero(),
    size: SCROLL_BUTTON_TAP_SIZE
};
const SCROLL_BUTTON_DOWN_TAP_AREA: Rectangle = Rectangle {
    top_left: Point { x: 0, y: SCROLL_AREA.size.height as i32 / 2 },
    size: SCROLL_BUTTON_TAP_SIZE
};
const SCROLL_BUTTON_UP_WIDGET: Widget = Widget::new(Rectangle{
    top_left: Point {
        x: (SCROLL_AREA.size.width - SCROLL_BUTTON_SIZE.width) as i32 / 2,
        y: STROKE_WIDTH as i32,
    },
    size: SCROLL_BUTTON_SIZE
}, SCROLL_AREA.top_left);

const SCROLL_BUTTON_DOWN_WIDGET: Widget = Widget::new(Rectangle{
    top_left: Point {
        x: (SCROLL_AREA.size.width - SCROLL_BUTTON_SIZE.width) as i32 / 2,
        y: (SCROLL_AREA.size.height - STROKE_WIDTH - SCROLL_BUTTON_SIZE.height) as i32,
    },
    size: SCROLL_BUTTON_SIZE
}, SCROLL_AREA.top_left);

struct ScrollWidget {
    pub scroll: usize,
    pub scroll_max: usize,
    button_up: ScrollButton,
    button_down: ScrollButton,
    showing: bool,
}

impl ScrollWidget {
    fn new() -> Self {
        Self {
            scroll: 0,
            scroll_max: 0,
            button_up: ScrollButton::new(&SCROLL_BUTTON_UP_WIDGET, ScrollButtonDirection::Up),
            button_down: ScrollButton::new(&SCROLL_BUTTON_DOWN_WIDGET, ScrollButtonDirection::Down),
            showing: true,
        }
    }
}

impl View for ScrollWidget {
    type DrawInput<'a> = ();
    type DrawOutput = Vec<Rectangle>;
    type EventInput<'a> = ();
    type TapOutput = bool;
    fn bounding_box(&self) -> Rectangle {
        SCROLL_WIDGET.bounding_box()
    }
    fn bounding_box_absolut(&self) -> Rectangle {
        SCROLL_WIDGET.bounding_box_absolute()
    }
	fn draw_view<'a, D: DrawTarget<Color = BinaryColor>>(&mut self, target: &mut DrawView<D>, _: Self::DrawInput<'_>) -> Result<Self::DrawOutput, D::Error>
    where Self: 'a {
        let mut areas = Vec::new();
        if self.showing {
            if self.scroll > 0 {
                self.button_up.draw(target, ())?;
                areas.push(self.button_up.bounding_box_absolut().offset(STROKE_WIDTH as i32));
            }
            if self.scroll < self.scroll_max {
                self.button_down.draw(target, ())?;
                areas.push(self.button_down.bounding_box_absolut().offset(STROKE_WIDTH as i32));
            }
            self.showing = false;
        }
        Ok(areas)
	}
    fn handle_event_view<'a>(&mut self, event: Event, _: ()) -> Self::TapOutput where Self: 'a, {
        match event {
            Event::Tap(point) => {
                self.showing = true;
                if SCROLL_BUTTON_UP_TAP_AREA.contains(point) {
                    self.scroll = self.scroll.saturating_sub(SCROLL_STEP);
                    return true
                }
                if SCROLL_BUTTON_DOWN_TAP_AREA.contains(point) {
                    let stepped = self.scroll.saturating_add(SCROLL_STEP);
                    self.scroll = if stepped > self.scroll_max { self.scroll_max } else { stepped };
                    return true
                }
            },
            _ => ()
        }
        false
    }
}

enum ScrollButtonDirection {
    Up,
    Down
}

struct ScrollButton {
	widget: &'static Widget,
    direction: ScrollButtonDirection
}

impl ScrollButton {
    fn new(widget: &'static Widget, direction: ScrollButtonDirection) -> Self {
        Self {
            widget,
            direction
        }
    }
}

impl View for ScrollButton {
    type DrawInput<'a> = ();
    type DrawOutput = ();
    type EventInput<'a> = ();
    type TapOutput = ();
    fn bounding_box(&self) -> Rectangle {
        self.widget.bounding_box()
    }
    fn bounding_box_absolut(&self) -> Rectangle {
        self.widget.bounding_box_absolute()
    }
	fn draw_view<'a, D: DrawTarget<Color = BinaryColor>>(&mut self, target: &mut DrawView<D>, _: Self::DrawInput<'_>) -> Result<Self::DrawOutput, D::Error>
    where Self: 'a {
        let stroke = PrimitiveStyleBuilder::new()
            .stroke_color(BinaryColor::On)
            .stroke_width(STROKE_WIDTH)
            .stroke_alignment(StrokeAlignment::Center)
            .build();

        match self.direction {
            ScrollButtonDirection::Up => {
                let line_asc = Line::new(
                    Point { x: 0, y: self.widget.bounds.size.height as i32},
                    Point { x: self.widget.bounds.size.width as i32 / 2, y: 0 }
                );
                line_asc.into_styled(stroke).draw(target)?;
                let line_desc = Line::new(
                    Point { x: self.widget.bounds.size.width as i32 / 2, y: 0 },
                    Point { x: self.widget.bounds.size.width as i32, y: self.widget.bounds.size.height as i32 },
                );
                line_desc.into_styled(stroke).draw(target)?;
            },
            ScrollButtonDirection::Down => {
                let line_desc = Line::new(
                    Point::zero(),
                    Point { x: self.widget.bounds.size.width as i32 / 2, y: self.widget.bounds.size.height as i32 },
                );
                line_desc.into_styled(stroke).draw(target)?;
                let line_asc = Line::new(
                    Point { x: self.widget.bounds.size.width as i32 / 2, y: self.widget.bounds.size.height as i32 },
                    Point { x: self.widget.bounds.size.width as i32, y: 0 }
                );
                line_asc.into_styled(stroke).draw(target)?;
            }
        }
        Ok(())
	}
    fn handle_event_view<'a>(&mut self, event: Event, _: ()) -> () where Self: 'a, {
        match event {
            Event::Tap(_) => {
            },
            _ => ()
        }
    }
}

fn max_lines_to_scroll(
    input: &str,
    max_chars_in_line: usize,
    tab_size: u16,
    line_height: u32,
    paragraph_spacing: u32,
    screen_height: u32,
) -> usize {
    let mut total_height = 0;
    let mut line_count = 0;
    for (i, paragraph) in input.lines().rev().enumerate() {
        let chunk_lines = count_lines(paragraph, max_chars_in_line, tab_size);
        // If it's not the first paragraph, add paragraph spacing
        if i > 0 {
            total_height += paragraph_spacing;
        }

        for _ in 0..chunk_lines {
            total_height += line_height;
            if total_height < screen_height {
                continue;
            }
            line_count += 1;
        }
    }
    line_count
}
