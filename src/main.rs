use log::{debug, info};
use sfml::graphics::RenderWindow;
use sfml::window::{ContextSettings, Event, Style};

fn main() {
    dotenv::dotenv().ok();
    env_logger::init();

    // Configure window preferences
    let width = dotenv::var("WIDTH").unwrap().parse::<u32>().unwrap_or(1024);
    let height = dotenv::var("HEIGHT").unwrap().parse::<u32>().unwrap_or(768);
    let title = dotenv::var("TITLE").unwrap_or("SFML Window".to_string());
    let vsync = dotenv::var("VSYNC")
        .unwrap()
        .to_lowercase()
        .parse::<bool>()
        .unwrap_or(false);
    let framerate = dotenv::var("FRAMERATE")
        .unwrap()
        .parse::<u32>()
        .unwrap_or(60);

    let major_version = dotenv::var("MAJOR_VERSION")
        .unwrap()
        .parse::<u32>()
        .unwrap_or(3);
    let minor_version = dotenv::var("MINOR_VERSION")
        .unwrap()
        .parse::<u32>()
        .unwrap_or(3);
    let antialias = dotenv::var("ANTIALIAS")
        .unwrap()
        .parse::<u32>()
        .unwrap_or(0);

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
