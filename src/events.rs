#![deny(missing_docs)]
//! # Event loop
//!
//! See [eventloop::run](`crate::events::EventLoop::run`) for an example.
use std::time::Instant;

use glutin::event::Event as WinitEvent;
use glutin::event::{KeyboardInput, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop as WinitEventLoop};

pub use glutin::event::{
    ElementState as ButtonState, ModifiersState as Modifiers, MouseButton, VirtualKeyCode as Key,
    MouseScrollDelta
};

use crate::Size;

/// An event provided by the event loop.
pub enum Event {
    /// Any kid of key input. `state` is either `Pressed` or `Released`
    Key {
        /// Current key
        key: Key,
        /// The state of the key (pressed or released)
        state: ButtonState,
    },

    /// A `char`, this will happen after the `KeyInput` so they are not exclusive
    Char(char),

    /// Key modifiers (shift, ctrl etc.)
    Modifier(Modifiers),

    /// Mouse cursor position relative to the window.
    /// This means 0,0 is top left of the window.
    MouseMoved {
        /// X
        x: f32,
        /// Y
        y: f32,
    },

    /// Mouse button pressed / released
    MouseButton {
        /// Button state: pressed or released
        state: ButtonState,

        /// Which mouse button 
        button: MouseButton,
    },

    /// Mouse wheel moved
    MouseWheel { 
        /// X
        x: f32, 
        /// Y
        y: f32 
    },

    /// Redraw the screen. Do rendering here.
    Draw(f32),

    /// Window was resized.
    Resize(Size<u32>),
}

/// For every iteration of the loop return one
/// variant of this enum.
pub enum LoopAction {
    /// Continue the loop
    Continue,

    /// Quit
    Quit,
}

/// The event loop. See the [`run`] function for an example.
pub struct EventLoop(pub(crate) WinitEventLoop<()>);

impl EventLoop {
    /// Create a new instance of an event loop,
    /// consuming the underlying Glutin (winit) event loop.
    pub fn new(el: WinitEventLoop<()>) -> Self {
        Self(el)
    }

    /// Start the event loop.
    /// This will never return.
    ///
    /// ```
    /// use nightmaregl::events::{LoopAction, EventLoop, Event};
    /// # fn run(loopy: EventLoop) {
    /// loopy.run(|event| {
    ///     match event {
    ///         Event::Char('q') => return LoopAction::Quit,
    ///         _ => {}
    ///     }
    ///
    ///     LoopAction::Continue
    /// })
    /// # }
    /// ```
    pub fn run<F>(self, mut event_handler: F) -> !
    where
        F: 'static + FnMut(Event) -> LoopAction,
    {
        let mut time = Instant::now();

        self.0.run(move |event, _window_id, control_flow| {
            let loop_action = match event {
                WinitEvent::WindowEvent { event, .. } => match event {
                    WindowEvent::ReceivedCharacter(c) => event_handler(Event::Char(c)),
                    WindowEvent::ModifiersChanged(modifiers) => {
                        event_handler(Event::Modifier(modifiers))
                    }
                    WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                virtual_keycode: Some(keycode),
                                state,
                                ..
                            },
                        ..
                    } => event_handler(Event::Key {
                        key: keycode,
                        state,
                    }),
                    WindowEvent::CursorMoved { position, .. } => event_handler(Event::MouseMoved {
                        x: position.x as f32,
                        y: position.y as f32,
                    }),
                    WindowEvent::MouseWheel { delta, .. } => {
                        let (x, y) = match delta {
                            MouseScrollDelta::LineDelta(x, y) => (x, y),
                            MouseScrollDelta::PixelDelta(pos) => (pos.x as f32, pos.y as f32),
                        };
                        event_handler(Event::MouseWheel { x, y })
                    }
                    WindowEvent::MouseInput { state, button, .. } => {
                        event_handler(Event::MouseButton { state, button })
                    }
                    WindowEvent::Resized(new_size) => {
                        event_handler(Event::Resize(Size::new(new_size.width, new_size.height)))
                    }
                    WindowEvent::CloseRequested => {
                        *control_flow = ControlFlow::Exit;
                        LoopAction::Quit
                    }
                    _ => LoopAction::Continue,
                },
                WinitEvent::RedrawEventsCleared => {
                    let dt = time.elapsed().as_secs_f32();
                    time = Instant::now();
                    event_handler(Event::Draw(dt))
                }
                _ => LoopAction::Continue,
            };

            match loop_action {
                LoopAction::Quit => *control_flow = ControlFlow::Exit,
                LoopAction::Continue => {}
            }
        });
    }
}

impl From<WinitEventLoop<()>> for EventLoop {
    fn from(el: WinitEventLoop<()>) -> Self {
        Self(el)
    }
}
