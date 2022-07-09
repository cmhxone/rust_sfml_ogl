use log::{debug, info};
use sfml::graphics::RenderWindow;
use sfml::window::{ContextSettings, Event, Style};

fn main() {
    dotenv::dotenv().ok();
    env_logger::init();

    // Configure window preferences
    let width = dotenv::var("WIDTH")
        .unwrap_or("1024".to_string())
        .parse::<u32>()
        .unwrap();
    let height = dotenv::var("HEIGHT")
        .unwrap_or("768".to_string())
        .parse::<u32>()
        .unwrap();
    let title = dotenv::var("TITLE").unwrap_or("SFML Window".to_string());
    let vsync = dotenv::var("VSYNC")
        .unwrap_or("false".to_string())
        .to_lowercase()
        .parse::<bool>()
        .unwrap();
    let framerate = dotenv::var("FRAMERATE")
        .unwrap_or("60".to_string())
        .parse::<u32>()
        .unwrap();

    let major_version = dotenv::var("MAJOR_VERSION")
        .unwrap_or("3".to_string())
        .parse::<u32>()
        .unwrap();
    let minor_version = dotenv::var("MINOR_VERSION")
        .unwrap_or("3".to_string())
        .parse::<u32>()
        .unwrap();
    let antialias = dotenv::var("ANTIALIAS")
        .unwrap_or("0".to_string())
        .parse::<u32>()
        .unwrap();

    // Configure OpenGL
    let mut context = ContextSettings::default();
    context.set_attribute_flags(ContextSettings::ATTRIB_CORE);
    context.set_major_version(major_version);
    context.set_minor_version(minor_version);
    context.set_antialiasing_level(antialias);

    // Create window
    let mut window = RenderWindow::new((width, height), &title, Style::CLOSE, &context);

    // VSync or frame capping
    if vsync {
        window.set_vertical_sync_enabled(vsync);
    } else {
        window.set_framerate_limit(framerate);
    }
    info!(
        "Create SFML Window ({}, {}, {}, {}, {}, {}, {})",
        width, height, vsync, framerate, major_version, minor_version, antialias
    );

    // GL Init
    gl_loader::init_gl();
    gl::load_with(|symbol| gl_loader::get_proc_address(symbol) as *const _);

    // Main Loop
    while window.is_open() {
        while let Some(event) = window.poll_event() {
            match event {
                Event::Closed => window.close(),
                _ => debug!("{:?}", event),
            }
        }
        window.set_active(true);
        unsafe {
            gl::ClearColor(0.0, 0.0, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        window.display();
    }

    // Free GL
    gl_loader::end_gl();
}
