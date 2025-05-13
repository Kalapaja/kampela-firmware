//! Everything high-level related to interfacing with user
use alloc::{borrow::ToOwned, string::String};
use kampela_system::{
    devices::{display::Request, psram::read_from_psram}, draw::{Bounds, BoundsTrait, DisplayMode, FrameBuffer, UpdateMode}, parallel::{AsyncOperation, Threads}
};
use crate::{hardware::Hardware, nfc::NfcTransactionPsramAccess, touch::take_touch_point};
use kampela_ui::{
    platform::Platform,
    uistate::{Event, UIState, UpdateRequest, UpdateRequestMutate}
};
/// General status of UI
///
/// There is no sense in reading input while screen processes last event, nor refreshing the screen
/// before touch was parsed
pub enum UIStatus {
    /// Event listening state, default
    UIUpdate(bool),
    /// Screen update started
    BufferUpdate,
}
impl Default for UIStatus {
    fn default() -> Self { UIStatus::UIUpdate(false) }
}
/// UI handler
pub struct UI {
    pub state: UIState<Hardware>,
    update_request: Option<UpdateRequest>,
    ui_threads: Threads<UIStatus, 2>,
    frame_buffer: FrameBufferOperation
}

impl UI {
    pub fn handle_message(&mut self, message: String) {
        self.update_request.propagate(self.state.handle_message(message, &mut ()));
    }

    pub fn handle_transaction(&mut self, transaction: NfcTransactionPsramAccess) {
        let k = read_from_psram(&transaction.sender_public_key_psram_access);
        if self.state.platform.public().map(|p| p.0 != *k).unwrap_or(true) {
            return self.handle_message("Invalid sender address".to_owned());
        }
        self.state.platform.set_transaction(transaction);
        self.update_request.propagate(self.state.handle_transaction(&mut ()));
    }

    pub fn handle_address(&mut self, addr: [u8; 76]) {
        self.update_request.propagate(self.state.handle_address(addr));
    }

    fn update_request_to_display_mode(&mut self, is_tapped: bool) -> Option<UpdateMode> {
        match self.update_request.take() {
            Some(UpdateRequest::Slow) => Some(UpdateMode::new(DisplayMode::Full, Bounds::new_fullscreen(), is_tapped)),
            Some(UpdateRequest::Fast) => Some(UpdateMode::new(DisplayMode::Fast, Bounds::new_fullscreen(), is_tapped)),
            Some(UpdateRequest::UltraFast) => Some(UpdateMode::new(DisplayMode::UltraFast, Bounds::new_fullscreen(), is_tapped)),
            Some(UpdateRequest::Part(r)) => Some(UpdateMode::new(DisplayMode::UltraFastSelective, Bounds::from_rectangle(r), is_tapped)),
            Some(UpdateRequest::UltraFastSelective) => Some(UpdateMode::new(DisplayMode::UltraFastSelective, Bounds::new_fullscreen(), is_tapped)),
            _ => None
        }
    }
}

impl AsyncOperation for UI {
    type Init = ();
    type Input<'a> = ();
    type Output = Option<bool>;
    
    /// Start of UI.
    fn new(_: Self::Init) -> Self {
        let hardware = Hardware::new();
        let state = UIState::new(hardware, &mut ());
        let frame_buffer = FrameBufferOperation::new(());

        return Self {
            state,
            update_request: Some(UpdateRequest::Slow),
            frame_buffer,
            ui_threads: Threads::<UIStatus, 2>::from([
                UIStatus::UIUpdate(false),
                UIStatus::BufferUpdate
            ])
        }
    }
    /// Call in event loop to progress through UI state
    fn advance<'a>(&mut self, _: Self::Input<'a>) -> Self::Output {
        match self.ui_threads.turn() {
            UIStatus::UIUpdate(tapped) => {
                if !*tapped {
                    if let Some(point) = take_touch_point() {
                        let u = self.state.handle_event(Event::Tap(point), &mut ());
                        if u.is_some() { *tapped = true; } // one tap handle per update
                        self.update_request.propagate(u);
                        return None
                    }
                }

                if matches!(self.update_request, Some(UpdateRequest::Invocate)) {
                    let u = self.state.handle_event(Event::Invocation, &mut ());
                    if u.is_none() { *tapped = false; }
                    self.update_request.propagate(u);
                };

                let t = *tapped;
                let m = self.update_request_to_display_mode(t);
                self.frame_buffer.propagate(m);
                if t {
                    self.ui_threads.sync(); // no need to poll
                }
                None
            },
            UIStatus::BufferUpdate => {
                let r = self.frame_buffer.advance((&mut self.state, &mut self.update_request));
                if r == Some(false) {
                    self.ui_threads.hold();
                }
                if r == Some(true) && !self.ui_threads.is_other_running() {
                    self.ui_threads.wind(UIStatus::UIUpdate(false));
                }
                r
            },
        }
    }
}

/// A virtual display that could be written to EPD simultaneously
pub struct FrameBufferOperation {
    frame_buffer: FrameBuffer,
    state: Threads<BufferState, 1>,
    update_request: Option<UpdateRequest>
}

impl FrameBufferOperation {
    fn propagate(&mut self, new_update_request: Option<UpdateMode>) {
        self.frame_buffer.propagate(new_update_request);
    }
}
pub enum BufferState {
    UIRender(UIRender),
    DisplaySend,
}

impl Default for BufferState {
    fn default() -> Self { BufferState::UIRender(UIRender::new(())) }
}

impl AsyncOperation for FrameBufferOperation {
    type Init = ();
    type Input<'a> = (&'a mut UIState<Hardware>, &'a mut Option<UpdateRequest>);
    type Output = Option<bool>;

    fn new(_: ()) -> Self {
        Self {
            frame_buffer: FrameBuffer::new_white(),
            state: Threads::new(BufferState::UIRender(UIRender::new(()))),
            update_request: None
        }
    }

    /// Move through display update progress
    fn advance<'a>(&mut self, (ui_state, new_update_request): Self::Input<'a>) -> Self::Output {
        match self.state.turn() {
            BufferState::UIRender(ui_render) => {
                let r = ui_render.advance((ui_state, &mut self.frame_buffer, &mut self.update_request));
                if r == Some(true) && matches!(self.update_request, Some(UpdateRequest::Invocate)) {
                    new_update_request.try_add(self.update_request.take()); // no need to wait for invocate request
                }
                if r == Some(true) && self.frame_buffer.can_send() {
                    self.state.change(BufferState::DisplaySend);
                }
                r
            },
            BufferState::DisplaySend => {
                let r = self.frame_buffer.advance(());
                if r == Some(true) {
                    self.frame_buffer.end_send();
                    // lower priority for render update request
                    // propagate update request once per update
                    new_update_request.try_add(self.update_request.take());
                    self.state.change(BufferState::UIRender(UIRender::new(())));
                    return Some(false)
                }
                r
            },
        }
    }
}

/// A virtual display that could be written to EPD simultaneously
pub struct UIRender {
    state: Threads<UIRenderState, 2>,
}

pub enum UIRenderState {
    UIRender,
    DisplayUpdate(Option<Request>),
}

impl Default for UIRenderState {
    fn default() -> Self { UIRenderState::DisplayUpdate(None) }
}

impl AsyncOperation for UIRender {
    type Init = ();
    type Input<'a> = (&'a mut UIState<Hardware>, &'a mut FrameBuffer, &'a mut Option<UpdateRequest>);
    type Output = Option<bool>;

    fn new(_: ()) -> Self {
        Self {
            state: Threads::from([
                UIRenderState::DisplayUpdate(None)
            ])
        }
    }

    /// Move through display update progress
    fn advance<'a>(&mut self, (ui_state, frame_buffer, new_update_request): Self::Input<'a>) -> Self::Output {
        match self.state.turn() {
            UIRenderState::UIRender => {
                self.state.sync();
                if !frame_buffer.has_request() {
                    return None
                }
                let Ok(u) = ui_state.render(frame_buffer, &mut ());
                new_update_request.propagate(u);
                if frame_buffer.end_render() {
                    return Some(true)
                }
                None
            },
            UIRenderState::DisplayUpdate(state) => {
                match state {
                    None => {
                        if let Some(m) = frame_buffer.has_update_request() {
                            *state = Some(Request::new(m));
                            Some(false)
                        } else {
                            self.state.wind(UIRenderState::UIRender);
                            None
                        }
                    },
                    Some(a) => {
                        match a.advance(()) {
                            Some(Some(true)) => {
                                *state = None;
                                frame_buffer.end_update();
                                Some(true)
                            },
                            Some(Some(false)) => { // some long awaiting
                                self.state.wind(UIRenderState::UIRender);
                                Some(false)
                            },
                            Some(None) => {
                                Some(false)
                            },
                            None => None
                        }
                    }
                }
            },
        }
    }
}
