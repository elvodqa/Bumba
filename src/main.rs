mod ui;

use sfml::{graphics::*, window::{ContextSettings, Style, Event}, system::Vector2f};
use crate::ui::*;


fn main() {
    let mut window = RenderWindow::new(
        (800, 600),
        "Bumba",
        Style::DEFAULT,
        &ContextSettings::default(),
    );
    window.set_vertical_sync_enabled(true);

    let mut camera = View::new(
        Vector2f::new(0.0, 0.0),
        Vector2f::new(800.0, 600.0)
    );

    let mut some_button = Button::new(0, 0, 100, 50, "Fuck fof".to_string(), Color::RED, Color::GREEN, Color::BLUE, Color::WHITE);

    while window.is_open() {
        while let Some(ev) = window.poll_event() {
            match ev {
                Event::Closed => window.close(),
                Event::Resized { width, height } => {
                    camera.set_size(Vector2f::new(width as f32, height as f32));
                    window.set_view(&camera);
                },
                Event::MouseButtonPressed { button, x, y } => {
                    some_button.poll_event(button, x, y);
                },
                _ => {}
            }
        }

        some_button.update(window.mouse_position());

        if some_button.is_hovered() {
            //println!("Hovered");
        }
        if some_button.is_down() {
            println!("Pressed");
        }

        window.clear(Color::BLACK);
        some_button.draw(&mut window);
        window.display();
    }

}