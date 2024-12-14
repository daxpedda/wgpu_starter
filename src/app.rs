use std::sync::Arc;
use winit::{
    application::ApplicationHandler,
    event::*,
    event_loop::ActiveEventLoop,
    keyboard::{KeyCode, PhysicalKey},
    window::{Window, WindowId},
};

use crate::state::State;

//  Window struct
#[derive(Default)]
pub struct App<'window> {
    window: Option<Arc<Window>>,
    state: Option<State<'window>>,
    window_id: Option<WindowId>,
}

impl<'window> ApplicationHandler for App<'window> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window_attributes = Window::default_attributes().with_title("Ubik says Learn WGPU");

        let window = Arc::new(
            event_loop
                .create_window(window_attributes)
                .expect("create window error"),
        );
        self.window_id = Some(window.id());
        let state = State::new(window.clone());
        self.state = Some(state);
        self.window = Some(window.clone());
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        let id = self.window.as_ref().unwrap().id();

        if window_id == id {
            match event {
                WindowEvent::CloseRequested => {
                    println!("The close button was pressed; stopping");
                    event_loop.exit();
                }
                WindowEvent::Resized(new_size) => {
                    if let (Some(state), Some(window)) = (self.state.as_mut(), self.window.as_ref())
                    {
                        state.resize(new_size);
                        window.request_redraw();
                    }
                }
                WindowEvent::RedrawRequested => {
                    // Redraw the application.
                    //
                    // It's preferable for applications that do not render continuously to render in
                    // this event rather than in AboutToWait, since rendering in here allows
                    // the program to gracefully handle redraws requested by the OS.

                    // Draw.
                    // Queue a RedrawRequested event.
                    //
                    // You only need to call this if you've determined that you need to redraw in
                    // applications which do not always need to. Applications that redraw continuously
                    // can render here instead.

                    if let Some(state) = self.state.as_mut() {
                        let _ = state.render();
                    }
                    //self.window.as_ref().unwrap().request_redraw();
                }
                _ => (),
            }
        }
    }
}
