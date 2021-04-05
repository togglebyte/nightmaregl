#![deny(missing_docs)]
//! # Event loop
//! 
//! See [eventloop::run](`crate::events::EventLoop::run`) for an example.
use std::time::Instant;

use glutin::event::Event as WinitEvent;
use glutin::event::{KeyboardInput, WindowEvent};
use glutin::event_loop::ControlFlow;

pub use glutin::event::{ModifiersState as Modifiers, VirtualKeyCode as Key, ElementState as KeyState};

use crate::Size;

/// For every iteration of the loop return one
/// variant of this enum.
pub enum LoopAction {
    /// Continue the loop
    Continue,

    /// Quit
    Quit,
}

/// The event loop. See the [`run`] function for an example.
pub struct EventLoop(pub(crate) glutin::event_loop::EventLoop<()>);

/// An event provided by the event loop.
pub enum Event {
    /// Any kid of key input. `state` is either `Pressed` or `Released`
    Key { 
        /// Current key
        key: Key,
        /// The state of the key (pressed or released)
        state: KeyState 
    },

    /// A `char`, this will happen after the `KeyInput` so they are not exclusive
    Char(char),

    /// Key modifiers (shift, ctrl etc.)
    Modifier(Modifiers),

    /// Mouse input (not yet done)
    MouseInput,

    /// Redraw the screen. Do rendering here.
    Draw(f32),

    /// Window was resized.
    Resize(Size<u32>),
}

impl EventLoop {
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
                    WindowEvent::ModifiersChanged(modifiers) => event_handler(Event::Modifier(modifiers)),
                    WindowEvent::KeyboardInput {
                        input: KeyboardInput { virtual_keycode: Some(keycode), state, ..  }, ..
                    } => event_handler(Event::Key { key: keycode, state }),
                    WindowEvent::Resized(new_size) => event_handler(Event::Resize(Size::new(new_size.width, new_size.height))),
                    WindowEvent::CloseRequested => {
                        *control_flow = ControlFlow::Exit;
                        LoopAction::Quit
                    }
                    _ => { LoopAction::Continue }
                }
                WinitEvent::RedrawEventsCleared => {
                    let dt = time.elapsed().as_secs_f32();
                    time = Instant::now();
                    event_handler(Event::Draw(dt))
                }
                _ => { LoopAction::Continue }
            };

            match loop_action {
                LoopAction::Quit => *control_flow = ControlFlow::Exit,
                LoopAction::Continue => {}
            }
        });
    }
}
