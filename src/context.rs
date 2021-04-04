#![deny(missing_docs)]
use num_traits::cast::NumCast;
use gl33::global_loader::*;
use gl33::*;
use glutin::event_loop::EventLoop as WinitEventLoop;
use glutin::window::Window;
use glutin::window::WindowBuilder;
use glutin::{
    Api, ContextBuilder as GlutinContextBuilder, ContextWrapper, GlRequest, PossiblyCurrent,
};

use crate::{Color, Result, Size, events::EventLoop};

/// Vertex array object
#[derive(Debug, PartialEq)]
pub(crate) struct Vao(pub(crate) u32);

impl Vao {
    pub(crate) fn id(&self) -> u32 {
        self.0
    }
}

impl Drop for Vao {
    fn drop(&mut self) {
        unsafe { glDeleteVertexArrays(1, &mut self.0) };
    }
}

// -----------------------------------------------------------------------------
//     - Context builder -
// -----------------------------------------------------------------------------
/// Context builder sets the window title,
/// enables / disables vsync,
/// enables / disables hardware acceleration,
/// and finally provides a [`Context`].
pub struct ContextBuilder {
    title: String,
    vsync: bool,
    hardware_acceleration: bool,
    size: Option<Size<i32>>,
    resizable: bool,
    maximized: bool,
    visible: bool,
    decorations: bool,
    always_on_top: bool,
}

impl ContextBuilder {
    fn new(title: String) -> Self {
        Self {
            title,
            vsync: true,
            hardware_acceleration: true,
            size: None,
            resizable: true,
            maximized: false,
            visible: true,
            decorations: true,
            always_on_top: false,
        }
    }

    /// Enable / disable vsync
    pub fn vsync(&mut self, on: bool) -> &mut Self {
        self.vsync = on;
        self
    }

    /// Enable / disable hardware acceleration
    pub fn hardware_acceleration(&mut self, on: bool) -> &mut Self {
        self.hardware_acceleration = on;
        self
    }

    /// Set inner size of the window
    pub fn with_size(&mut self, size: Size<i32>) -> &mut Self {
        self.size = Some(size);
        self
    }

    /// Make the window resizable or not.
    /// True by default.
    pub fn resizable(&mut self, resizable: bool) -> &mut Self {
        self.resizable = resizable;
        self
    }

    /// Set fullscreen.
    /// False by default.
    pub fn fullscreen(&mut self, _fullscreen: bool) -> &mut Self {
        unimplemented!();
    }

    /// Toggle window maximized.
    /// False by default.
    pub fn maximized(&mut self, maximized: bool) -> &mut Self {
        self.maximized = maximized;
        self
    }

    /// Toggle decorations.
    /// True by default.
    pub fn decorations(&mut self, decorations: bool) -> &mut Self {
        self.decorations = decorations;
        self
    }

    /// Toggle always on top.
    /// False by default.
    pub fn always_on_top(&mut self, on_top: bool) -> &mut Self {
        self.always_on_top = on_top;
        self
    }

    pub fn from_builder(&self, win_builder: WindowBuilder) -> Result<(EventLoop, Context)> {
        let event_loop = WinitEventLoop::<()>::new();

        // Set this to 3.3
        let context = GlutinContextBuilder::new()
            .with_gl(GlRequest::Specific(Api::OpenGl, (3, 3)))
            .with_vsync(self.vsync)
            .with_hardware_acceleration(Some(self.hardware_acceleration))
            .build_windowed(win_builder, &event_loop)
            .unwrap();

        let context = unsafe {
            let context = match context.make_current() {
                Ok(c) => c,
                Err((_, e)) => return Err(e.into()),
            };

            load_global_gl(&|ptr| {
                let c_str = std::ffi::CStr::from_ptr(ptr as *const i8);
                let r_str = c_str.to_str().unwrap();
                context.get_proc_address(r_str) as _
            });

            context
        };

        // Setup alpha blending and depth testing
        unsafe {
            glEnable(GL_BLEND);
            glEnable(GL_DEPTH_TEST);
            glDepthFunc(GL_LESS);
            glBlendFunc(GL_SRC_ALPHA, GL_ONE_MINUS_SRC_ALPHA);
        }

        let inst = Context {
            inner: context,
            current_vao_id: 0,
        };

        Ok((EventLoop(event_loop), inst))
    }

    /// Finalise the context builder and produce a [`Context`]
    pub fn build(&mut self) -> Result<(EventLoop, Context)> {
        let mut window_builder = WindowBuilder::new()
            .with_title(&self.title)
            .with_resizable(self.resizable)
            .with_maximized(self.maximized)
            .with_visible(self.visible)
            .with_always_on_top(self.always_on_top);



        // Window size
        if let Some(size) = self.size {
            let size = glutin::dpi::PhysicalSize {
                width: size.width,
                height: size.height
            };
            window_builder = window_builder.with_inner_size(size);
        }

        self.from_builder(window_builder)
    }
}

// -----------------------------------------------------------------------------
//     - Context -
// -----------------------------------------------------------------------------
/// Context holds the open gl context and an event loop.
/// ```ignore
/// use nightmaregl::Context;
///
/// let (event_loop, context) = Context::builder("window title")
///     .vsync(true)                 // true by default
///     .hardware_acceleration(true) // true by default
///     .build()
///     .unwrap();
///
/// // Draw 
///
/// context.swap_buffers();
/// ```
pub struct Context {
    inner: ContextWrapper<PossiblyCurrent, Window>,
    current_vao_id: u32,
}

impl Context {
    pub(crate) fn bind_vao(&mut self, vao: &Vao) {
        if self.current_vao_id != vao.id() {
            glBindVertexArray(vao.id());
            self.current_vao_id = vao.id();
        }
    }

    /// Swap the buffer on the current window, making all changes visible.
    pub fn swap_buffers(&self) {
        let _ = self.inner.swap_buffers().unwrap();
    }

    /// Create a context builder. The title is the window title.
    pub fn builder(title: impl Into<String>) -> ContextBuilder {
        ContextBuilder::new(title.into())
    }

    /// Get the current window size.
    /// Useful when creating a [Viewport](crate::Viewport).
    pub fn window_size<T : Copy + NumCast>(&self) -> Size<T> {
        let size = self.inner.window().inner_size();
        Size::new(size.width, size.height).cast()
    }

    /// Get the current window handle
    pub fn window(&self) -> &Window {
        self.inner.window()
    }

    /// Clear the frame buffer.
    /// ```
    /// use nightmaregl::Color;
    /// # use nightmaregl::Context;
    /// # fn run(context: &mut Context) {
    /// loop {
    ///     context.clear(Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 });
    ///     context.swap_buffers();
    /// }
    /// # }
    /// ```
    pub fn clear(&self, color: Color) {
        unsafe {
            glClearColor(color.r, color.g, color.b, color.a);
            glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);
        }
    }

    pub(crate) fn next_vao(&mut self) -> Vao {
        let mut vao = 0;

        unsafe { glGenVertexArrays(1, &mut vao) };

        Vao(vao)
    }
}
