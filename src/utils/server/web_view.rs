use std::path::Path;
use std::io::Result;
use tao::event::{Event, WindowEvent};
use tao::event_loop::{ControlFlow, EventLoop};
use tao::platform::windows::IconExtWindows;
use tao::window::{Icon, WindowBuilder};
use wry::WebViewBuilder;
use crate::utils::env;

pub fn run() -> Result<()> {
    // Initialize the tao event loop
    let event_loop = EventLoop::new();

    // Create a window with tao's WindowBuilder
    let window = WindowBuilder::new()
        .with_title("Event Manager")
        .with_inner_size(tao::dpi::LogicalSize::new(1000.0, 600.0)) // Sets the initial window size
        .with_resizable(true)                                  // Allows the window to be resizable
        .with_decorations(true)                                // Enables window decorations (title bar, close button)
        .with_always_on_top(true)                             // Keeps window on top of other windows
        .with_transparent(false)                               // Sets the window background to be transparent
        .with_fullscreen(None)                                 // Enables fullscreen mode (use options for specific display)
        .with_maximized(false)                                 // Opens the window in maximized state
        .with_visible(true)                                    // Controls the initial visibility of the window
        .with_window_icon(load_icon())                         // Sets a custom window icon
        .with_transparent(true)                                // Makes window background transparent
        .build(&event_loop)
        .expect("Failed to create window");


    // Initialize the WebView attached to the tao window
    let _webview = WebViewBuilder::new()
        // Use format! to build the URL with a dynamic value
        .with_url(&format!("http://{}", env::get("APP_WEBVIEW_SERVER_URL")))  // Set your local server URL // Set your local server URL
        .build(&window)  // Attach to the tao window
        .expect("Failed to build WebView");

    // Run the event loop to handle window events
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        if let Event::WindowEvent {
            event: WindowEvent::CloseRequested, ..
        } = event
        {
            *control_flow = ControlFlow::Exit;
        }
    });
}

fn load_icon() -> Option<Icon> {
    let icon_path = Path::new("resources/images/icons/favicon.ico");
    Icon::from_path(icon_path, None).ok()
}