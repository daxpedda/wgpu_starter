use winit::{
    error::EventLoopError,
    event_loop::{ControlFlow, EventLoop},
};
pub mod state;

pub mod app;
use app::App;

pub mod vertex;

pub mod utils;

#[cfg(target_arch="wasm32")]
use wasm_bindgen::prelude::wasm_bindgen;

#[cfg_attr(target_arch="wasm32", wasm_bindgen(start))]
fn run() {
    
        cfg_if::cfg_if! {
            if #[cfg(target_arch = "wasm32")] {
                std::panic::set_hook(Box::new(console_error_panic_hook::hook));
                console_log::init_with_level(log::Level::Warn).expect("Couldn't initialize logger");
            } else {
                env_logger::init();
            }
        }
            let event_loop = EventLoop::new().unwrap();
        // ControlFlow::Poll continuously runs the event loop, even if the OS hasn't
        // dispatched any events. This is ideal for games and similar applications.
        event_loop.set_control_flow(ControlFlow::Poll);
    
        // ControlFlow::Wait pauses the event loop if no events are available to process.
        // This is ideal for non-game applications that only update in response to user
        // input, and uses significantly less power/CPU time than ControlFlow::Poll.
        //event_loop.set_control_flow(ControlFlow::Wait);
    
        let mut app = App::default();
        let _ = event_loop.run_app(&mut app);
        //let mut state = State::new(&app);

}
fn main()  {
    run();
}
