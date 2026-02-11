//! Scrollable Ethereum transaction viewer widget.
//!
//! Displays transaction details with scrolling support for long content.

#[cfg(not(feature = "std"))]
use alloc::string::String;
#[cfg(not(feature = "std"))]
use alloc::vec::Vec;
#[cfg(feature = "std")]
use std::string::String;
#[cfg(feature = "std")]
use std::vec::Vec;

use embedded_graphics::{
    draw_target::DrawTarget,
    geometry::Point,
    pixelcolor::BinaryColor,
    mono_font::{ascii::FONT_10X20, ascii::FONT_6X10, MonoTextStyle},
    primitives::{Primitive, PrimitiveStyle, Rectangle},
    Drawable,
};
use embedded_text::{
    alignment::{HorizontalAlignment, VerticalAlignment},
    style::TextBoxStyleBuilder,
    TextBox,
};

use crate::widget::nav_bar::nav_bar::NavBar;
use crate::widget::view::{View, ViewScreen};
use crate::uistate::{EventResult, UpdateRequest};
use crate::display_def::*;

const LINE_HEIGHT: i32 = FONT_6X10.character_size.height as i32;
const FONT_CHAR_WIDTH: u32 = FONT_6X10.character_size.width;
const TITLE_HEIGHT: i32 = 25;
const CONTENT_TOP: i32 = 30;
const CONTENT_LEFT_PADDING: i32 = 5;
const CONTENT_RIGHT_PADDING: i32 = 5;
const CONTENT_BOTTOM_PADDING: i32 = 0;
const SCROLL_BUTTON_WIDTH: i32 = 24;
const SCROLL_BUTTON_HEIGHT: i32 = 40;
const SCROLL_BUTTON_MARGIN: i32 = 3;
const SCROLL_BUTTON_RESERVE: i32 = SCROLL_BUTTON_WIDTH + SCROLL_BUTTON_MARGIN + 5;
const NAV_BAR_HEIGHT: i32 = 32;

/// Scrollable transaction viewer widget
pub struct EthTransactionViewer {
    scroll_offset: i32, // Current scroll position in lines
    total_lines: i32,   // Total lines in the content
    wrapped_lines: Vec<String>,
}

impl EthTransactionViewer {
    pub fn new(content: String) -> Self {
        let reserved = (CONTENT_LEFT_PADDING + CONTENT_RIGHT_PADDING + SCROLL_BUTTON_RESERVE) as u32;
        let content_width = SCREEN_SIZE_X.saturating_sub(reserved);
        let chars_per_line = (content_width / FONT_CHAR_WIDTH).max(1) as usize;
        let wrapped_lines = Self::wrap_content(&content, chars_per_line);
        let total_lines = wrapped_lines.len() as i32;

        Self {
            scroll_offset: 0,
            total_lines,
            wrapped_lines,
        }
    }

    /// Scroll up by one line
    pub fn scroll_up(&mut self) -> bool {
        let step = self.scroll_step_lines();
        let next_offset = (self.scroll_offset - step).max(0);
        if next_offset != self.scroll_offset {
            self.scroll_offset = next_offset;
            return true;
        }
        false
    }

    /// Scroll down by one line
    pub fn scroll_down(&mut self) -> bool {
        let max_scroll = self.max_scroll();
        let step = self.scroll_step_lines();
        let next_offset = (self.scroll_offset + step).min(max_scroll);
        if next_offset != self.scroll_offset {
            self.scroll_offset = next_offset;
            return true;
        }
        false
    }

    /// Check if content can scroll up
    pub fn can_scroll_up(&self) -> bool {
        self.scroll_offset > 0
    }

    /// Check if content can scroll down
    pub fn can_scroll_down(&self) -> bool {
        self.scroll_offset < self.max_scroll()
    }

    /// Draw the transaction viewer
    pub fn draw<D>(&self, display: &mut D, n: bool) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = BinaryColor>,
    {
        let (on, off) = if n {
            (BinaryColor::Off, BinaryColor::On)
        } else {
            (BinaryColor::On, BinaryColor::Off)
        };

        let filled = PrimitiveStyle::with_fill(off);
        let bounds = display.bounding_box();

        // Clear screen
        bounds.into_styled(filled).draw(display)?;

        // Draw title
        let title_style = MonoTextStyle::new(&FONT_10X20, on);
        let title_textbox_style = TextBoxStyleBuilder::new()
            .alignment(HorizontalAlignment::Center)
            .build();

        let title_bounds = Rectangle::new(
            bounds.top_left,
            embedded_graphics::prelude::Size::new(bounds.size.width, TITLE_HEIGHT as u32),
        );

        TextBox::with_textbox_style(
            "Sign Transaction?",
            title_bounds,
            title_style,
            title_textbox_style,
        )
        .draw(display)?;

        // Draw transaction details with vertical offset based on scroll
        let content_style = MonoTextStyle::new(&FONT_6X10, on);
        let content_textbox_style = TextBoxStyleBuilder::new()
            .alignment(HorizontalAlignment::Left)
            .vertical_alignment(VerticalAlignment::Top)
            .build();

        // Keep the viewport fixed and scroll by rendering only the visible lines.
        let content_bounds = Rectangle::new(
            embedded_graphics::prelude::Point::new(CONTENT_LEFT_PADDING, CONTENT_TOP),
            embedded_graphics::prelude::Size::new(
                bounds
                    .size
                    .width
                    .saturating_sub((CONTENT_LEFT_PADDING + CONTENT_RIGHT_PADDING + SCROLL_BUTTON_RESERVE) as u32),
                self.content_height().max(0) as u32,
            ),
        );

        let visible_text = self.visible_text();
        TextBox::with_textbox_style(
            &visible_text,
            content_bounds,
            content_style,
            content_textbox_style,
        )
        .draw(display)?;

        // Draw scroll buttons on the right edge (always visible).
        self.draw_scroll_button(display, true, on)?;
        self.draw_scroll_button(display, false, on)?;

        // Draw navigation bar (left = reject, right = approve)
        let mut nav = NavBar::new(("Reject", "Approve"));
        nav.draw(display, n)?;

        Ok(())
    }

    /// Draw scroll button on right edge of screen
    /// is_up: true = top button (scroll up), false = bottom button (scroll down)
    fn draw_scroll_button<D>(
        &self,
        display: &mut D,
        is_up: bool,
        color: BinaryColor,
    ) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = BinaryColor>,
    {
        use embedded_graphics::primitives::{Line, PrimitiveStyleBuilder};

        const BUTTON_WIDTH: u32 = SCROLL_BUTTON_WIDTH as u32;
        const BUTTON_HEIGHT: u32 = SCROLL_BUTTON_HEIGHT as u32;
        const BUTTON_MARGIN: i32 = SCROLL_BUTTON_MARGIN;

        let style = PrimitiveStyleBuilder::new()
            .stroke_color(color)
            .stroke_width(2)
            .build();

        let fill_style = PrimitiveStyleBuilder::new()
            .fill_color(BinaryColor::Off)
            .stroke_color(color)
            .stroke_width(2)
            .build();

        // Position button on right edge
        // Up button at top, down button at bottom (above nav bar)
        let button_x = SCREEN_SIZE_X as i32 - BUTTON_WIDTH as i32 - BUTTON_MARGIN;
        let button_y = if is_up {
            CONTENT_TOP + BUTTON_MARGIN // Just below title
        } else {
            SCREEN_SIZE_Y as i32 - NAV_BAR_HEIGHT - BUTTON_HEIGHT as i32 - BUTTON_MARGIN // Above nav bar
        };

        // Draw button background (simple rectangle)
        let button_rect = Rectangle::new(
            Point::new(button_x, button_y),
            embedded_graphics::prelude::Size::new(BUTTON_WIDTH, BUTTON_HEIGHT),
        );
        button_rect.into_styled(fill_style).draw(display)?;

        // Draw arrow inside button (smaller arrows for smaller buttons)
        let arrow_center_x = button_x + (BUTTON_WIDTH as i32 / 2);
        let arrow_center_y = button_y + (BUTTON_HEIGHT as i32 / 2);

        if is_up {
            // Draw upward arrow: ^
            Line::new(
                Point::new(arrow_center_x - 4, arrow_center_y + 3),
                Point::new(arrow_center_x, arrow_center_y - 3),
            )
            .into_styled(style)
            .draw(display)?;
            Line::new(
                Point::new(arrow_center_x, arrow_center_y - 3),
                Point::new(arrow_center_x + 4, arrow_center_y + 3),
            )
            .into_styled(style)
            .draw(display)?;
        } else {
            // Draw downward arrow: v
            Line::new(
                Point::new(arrow_center_x - 4, arrow_center_y - 3),
                Point::new(arrow_center_x, arrow_center_y + 3),
            )
            .into_styled(style)
            .draw(display)?;
            Line::new(
                Point::new(arrow_center_x, arrow_center_y + 3),
                Point::new(arrow_center_x + 4, arrow_center_y - 3),
            )
            .into_styled(style)
            .draw(display)?;
        }

        Ok(())
    }

    fn content_height(&self) -> i32 {
        SCREEN_SIZE_Y as i32 - CONTENT_TOP - NAV_BAR_HEIGHT - CONTENT_BOTTOM_PADDING
    }

    fn visible_lines(&self) -> i32 {
        (self.content_height() / LINE_HEIGHT).max(1)
    }

    fn scroll_step_lines(&self) -> i32 {
        (self.visible_lines() / 2).max(1)
    }

    fn max_scroll(&self) -> i32 {
        (self.total_lines - self.visible_lines()).max(0)
    }

    fn visible_text(&self) -> String {
        let len = self.wrapped_lines.len();
        let start = self.scroll_offset.max(0) as usize;
        if start >= len {
            return String::new();
        }
        let end = (start + self.visible_lines() as usize).min(len);
        let mut out = String::new();
        for (idx, line) in self.wrapped_lines[start..end].iter().enumerate() {
            if idx > 0 {
                out.push('\n');
            }
            out.push_str(line);
        }
        out
    }

    fn wrap_content(content: &str, chars_per_line: usize) -> Vec<String> {
        let mut lines = Vec::new();
        let mut current = String::new();
        let mut count = 0usize;

        for ch in content.chars() {
            if ch == '\n' {
                lines.push(current);
                current = String::new();
                count = 0;
                continue;
            }

            current.push(ch);
            count += 1;

            if count >= chars_per_line {
                lines.push(current);
                current = String::new();
                count = 0;
            }
        }

        if !current.is_empty() || lines.is_empty() {
            lines.push(current);
        }

        lines
    }
}

impl ViewScreen for EthTransactionViewer {
    type DrawInput<'a> = ();
    type DrawOutput = ();
    type TapInput<'a> = ();
    type TapOutput = ();

    fn draw_screen<'a, D>(
        &mut self,
        target: &mut D,
        _: (),
    ) -> Result<(EventResult, Self::DrawOutput), D::Error>
    where
        D: DrawTarget<Color = BinaryColor>,
        Self: 'a,
    {
        self.draw(target, false)?;
        Ok((
            EventResult {
                request: None,
                state: None,
            },
            (),
        ))
    }

    fn handle_tap_screen<'a>(
        &mut self,
        point: Point,
        _input: Self::TapInput<'a>,
    ) -> (EventResult, Self::TapOutput)
    where
        Self: 'a,
    {
        let mut request = None;

        const BUTTON_WIDTH: i32 = SCROLL_BUTTON_WIDTH;
        const BUTTON_HEIGHT: i32 = SCROLL_BUTTON_HEIGHT;
        const BUTTON_MARGIN: i32 = SCROLL_BUTTON_MARGIN;

        // Calculate button positions (right edge)
        let button_x = SCREEN_SIZE_X as i32 - BUTTON_WIDTH - BUTTON_MARGIN;
        let up_button_y = CONTENT_TOP + BUTTON_MARGIN;
        let down_button_y = SCREEN_SIZE_Y as i32 - NAV_BAR_HEIGHT - BUTTON_HEIGHT - BUTTON_MARGIN;

        // Check if tap is on scroll up button (top right)
        if point.x >= button_x
            && point.x <= button_x + BUTTON_WIDTH
            && point.y >= up_button_y
            && point.y <= up_button_y + BUTTON_HEIGHT
        {
            if self.can_scroll_up() && self.scroll_up() {
                request = Some(UpdateRequest::Fast);
            }
        }
        // Check if tap is on scroll down button (bottom right)
        else if point.x >= button_x
            && point.x <= button_x + BUTTON_WIDTH
            && point.y >= down_button_y
            && point.y <= down_button_y + BUTTON_HEIGHT
        {
            if self.can_scroll_down() && self.scroll_down() {
                request = Some(UpdateRequest::Fast);
            }
        }

        (
            EventResult {
                request,
                state: None,
            },
            (),
        )
    }
}
