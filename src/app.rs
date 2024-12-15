use crate::{state::State, utils::load_icon};
use std::sync::Arc;
use winit::{
    application::ApplicationHandler,
    event::*,
    event_loop::ActiveEventLoop,
    keyboard::{KeyCode, PhysicalKey},
    window::{Window, WindowId},
};



//  Window struct
#[derive(Default)]
pub struct App<'window> {
    window: Option<Arc<Window>>,
    state: Option<State<'window>>,
    window_id: Option<WindowId>,
}

impl<'window> ApplicationHandler for App<'window> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window_icon: Option<winit::window::Icon> = Some(load_icon("assets/icon.png"));
        let window_attributes = Window::default_attributes()
            .with_title("Ubik says Learn WGPU")
            .with_window_icon(window_icon);

        let window = Arc::new(
            event_loop
                .create_window(window_attributes)
                .expect("create window error"),
        );
        self.window_id = Some(window.id());
        let state = State::new(window.clone());
        self.state = Some(state);
        self.window = Some(window.clone());

        #[cfg(target_arch = "wasm32")]
        {
            // Winit prevents sizing with CSS, so we have to set
            // the size manually when on web.
            use winit::dpi::PhysicalSize;
            let _ = window.clone().request_inner_size(PhysicalSize::new(450, 400));
            
            use winit::platform::web::WindowExtWebSys;
            web_sys::window()
                .and_then(|win| win.document())
                .and_then(|doc| {
                    let dst = doc.get_element_by_id("wasm-example")?;
                    let canvas = web_sys::Element::from(window.clone().canvas()?);
                    dst.append_child(&canvas).ok()?;
                    Some(())
                })
                .expect("Couldn't append canvas to document body.");
        }

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
