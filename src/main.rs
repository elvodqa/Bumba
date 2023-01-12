mod ui;

use crate::ui::*;

use sfml::{graphics::*, window::{ContextSettings, Style, Event}, system::Vector2f};

fn main() {
    let mut window = RenderWindow::new(
        (800, 600),
        "Bumba",
        Style::CLOSE,
        &ContextSettings::default(),
    );
    window.set_vertical_sync_enabled(true);

    let mut camera = View::new(
        Vector2f::new(0.0, 0.0),
        Vector2f::new(800.0, 600.0)
    );

    while window.is_open() {
        while let Some(ev) = window.poll_event() {
            match ev {
                Event::Closed => window.close(),
                Event::Resized { width, height } => {
                    // resize the camera
                    camera.set_size(Vector2f::new(width as f32, height as f32));
                    window.set_view(&camera);
                },
                _ => {}
            }
        }

        window.clear(Color::BLACK);
        
        
        window.display();
    }

}