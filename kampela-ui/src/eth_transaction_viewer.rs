//! Scrollable Ethereum transaction viewer widget.
//!
//! Displays transaction details with scrolling support for long content.

#[cfg(not(feature = "std"))]
use alloc::string::String;
#[cfg(feature = "std")]
use std::string::String;

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

const LINE_HEIGHT: i32 = 12; // Height of FONT_6X10 + spacing
const VISIBLE_LINES: i32 = 16; // Approximate lines visible in content area

/// Scrollable transaction viewer widget
pub struct EthTransactionViewer {
    scroll_offset: i32, // Current scroll position in lines
    total_lines: i32,   // Total lines in the content
    content: String,    // Transaction display text
}

impl EthTransactionViewer {
    pub fn new(content: String) -> Self {
        // Estimate total lines based on content length and screen width
        // This is approximate - actual line count depends on text wrapping
        let chars_per_line = 28; // Approximate for FONT_6X10 at screen width

        // Integer division with ceiling: (a + b - 1) / b
        let estimated_lines = ((content.len() + chars_per_line - 1) / chars_per_line) as i32;

        Self {
            scroll_offset: 0,
            total_lines: estimated_lines.max(VISIBLE_LINES),
            content,
        }
    }

    /// Scroll up by one line
    pub fn scroll_up(&mut self) -> bool {
        if self.scroll_offset > 0 {
            self.scroll_offset -= 1;
            true
        } else {
            false
        }
    }

    /// Scroll down by one line
    pub fn scroll_down(&mut self) -> bool {
        let max_scroll = (self.total_lines - VISIBLE_LINES).max(0);
        if self.scroll_offset < max_scroll {
            self.scroll_offset += 1;
            true
        } else {
            false
        }
    }

    /// Check if content can scroll up
    pub fn can_scroll_up(&self) -> bool {
        self.scroll_offset > 0
    }

    /// Check if content can scroll down
    pub fn can_scroll_down(&self) -> bool {
        let max_scroll = (self.total_lines - VISIBLE_LINES).max(0);
        self.scroll_offset < max_scroll
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
            embedded_graphics::prelude::Size::new(bounds.size.width, 25),
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

        // Adjust content bounds to account for scroll offset and leave space for scroll buttons
        // Reserve 25px on the right for scroll buttons (18px button + 3px margin + 4px padding)
        const SCROLL_BUTTON_RESERVE: u32 = 25;
        let y_offset = 30 - (self.scroll_offset * LINE_HEIGHT);
        let content_bounds = Rectangle::new(
            embedded_graphics::prelude::Point::new(5, y_offset),
            embedded_graphics::prelude::Size::new(
                bounds.size.width - 10 - SCROLL_BUTTON_RESERVE,
                bounds.size.height - 60
            ),
        );

        TextBox::with_textbox_style(
            &self.content,
            content_bounds,
            content_style,
            content_textbox_style,
        )
        .draw(display)?;

        // Draw scroll buttons on the edges if needed
        if self.can_scroll_up() {
            // Draw up button on left edge
            self.draw_scroll_button(display, true, on)?;
        }

        if self.can_scroll_down() {
            // Draw down button on right edge
            self.draw_scroll_button(display, false, on)?;
        }

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

        const BUTTON_WIDTH: u32 = 18;
        const BUTTON_HEIGHT: u32 = 30;
        const BUTTON_MARGIN: i32 = 3;

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
            30 + BUTTON_MARGIN // Just below title
        } else {
            SCREEN_SIZE_Y as i32 - 40 - BUTTON_HEIGHT as i32 - BUTTON_MARGIN // Above nav bar
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

        const BUTTON_WIDTH: i32 = 18;
        const BUTTON_HEIGHT: i32 = 30;
        const BUTTON_MARGIN: i32 = 3;

        // Calculate button positions (right edge)
        let button_x = SCREEN_SIZE_X as i32 - BUTTON_WIDTH - BUTTON_MARGIN;
        let up_button_y = 30 + BUTTON_MARGIN;
        let down_button_y = SCREEN_SIZE_Y as i32 - 40 - BUTTON_HEIGHT - BUTTON_MARGIN;

        // Check if tap is on scroll up button (top right)
        if self.can_scroll_up()
            && point.x >= button_x
            && point.x <= button_x + BUTTON_WIDTH
            && point.y >= up_button_y
            && point.y <= up_button_y + BUTTON_HEIGHT
        {
            if self.scroll_up() {
                request = Some(UpdateRequest::Fast);
            }
        }
        // Check if tap is on scroll down button (bottom right)
        else if self.can_scroll_down()
            && point.x >= button_x
            && point.x <= button_x + BUTTON_WIDTH
            && point.y >= down_button_y
            && point.y <= down_button_y + BUTTON_HEIGHT
        {
            if self.scroll_down() {
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
