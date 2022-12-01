use crate::app::MyApp;

use std::error::Error;

mod app;
mod persistent_state;
mod app_state;
mod js_runner;
mod syntax_highlighting;
mod code_editor;

fn main() -> Result<(), Box<dyn Error>> {
    // Initialize V8.
    let platform = v8::new_default_platform(0, false).make_shared();
    v8::V8::initialize_platform(platform);
    v8::V8::initialize();

    let options = eframe::NativeOptions {
        drag_and_drop_support: true,
        initial_window_size: Some([800.0, 600.0].into()),
        #[cfg(feature = "wgpu")]
        renderer: eframe::Renderer::Wgpu,
        ..Default::default()
    };
    let my_app = MyApp::default();
    eframe::run_native("Javascript Runner", options, Box::new(|_cc| {
        Box::new(my_app)
    }));
    Ok(())
}
