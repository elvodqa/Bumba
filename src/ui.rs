use sfml::{graphics::{RenderWindow, Color, RenderTarget, Transformable, Shape}, system::{Vector2i, Vector2f}};



pub struct Button {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub text: String,
    pub color: Color,
    pub hover_color: Color,
    pub pressed_color: Color,
    pub text_color: Color,
    pub pressed: bool,
    pub hover: bool,
    pub down: bool,
}

pub trait TButton {
    fn new(x: i32, y: i32, width: i32, height: i32, text: String, color: Color, hover_color: Color, pressed_color: Color, text_color: Color) -> Button;
    fn draw(&mut self, window: &mut RenderWindow);
    fn update(&mut self, mouse_pos: Vector2i);
    fn event_press(&mut self, button: sfml::window::mouse::Button, x :i32, y: i32);
    fn event_release(&mut self, button: sfml::window::mouse::Button, x :i32, y: i32);
    fn is_pressed(&self) -> bool;
    fn is_hovered(&self) -> bool;
    fn is_down(&self) -> bool;
}

impl TButton for Button {
    fn new(x: i32, y: i32, width: i32, height: i32, text: String, color: Color, hover_color: Color, pressed_color: Color, text_color: Color) -> Button {
        Button {
            x,
            y,
            width,
            height,
            text,
            color,
            hover_color,
            pressed_color,
            text_color,
            pressed: false,
            hover: false,
            down: false,
        }
    }

    fn draw(&mut self, window: &mut RenderWindow) {
        let mut rect = sfml::graphics::RectangleShape::new();
        rect.set_size((self.width as f32, self.height as f32));
        rect.set_position((self.x as f32, self.y as f32));
        rect.set_fill_color(if self.pressed { self.pressed_color } else if self.hover { self.hover_color } else { self.color });
        let binding = sfml::graphics::Font::from_file("assets/fonts/Roboto-Regular.ttf").unwrap();
        let mut text = sfml::graphics::Text::new(&self.text, &binding, 20);
        text.set_fill_color(self.text_color);
        text.set_position((self.x as f32 + self.width as f32 / 2.0 - text.local_bounds().width / 2.0, self.y as f32 + self.height as f32 / 2.0 - text.local_bounds().height / 2.0));

        window.draw(&rect);
        window.draw(&text);
        
        self.pressed = false
    }

    fn update(&mut self, mouse_pos: Vector2i) {
        if mouse_pos.x > self.x && mouse_pos.x < self.x + self.width && mouse_pos.y > self.y && mouse_pos.y < self.y + self.height {
            self.hover = true;
            if sfml::window::mouse::Button::Left.is_pressed() {
                self.down = true;
            } else {
                self.down = false;
            }
        } else {
            self.down = false;
            self.hover = false
        }
    }

    fn event_press(&mut self, button: sfml::window::mouse::Button, x :i32, y: i32) {
        if button == sfml::window::mouse::Button::Left && x > self.x && x < self.x + self.width && y > self.y && y < self.y + self.height {
            self.pressed = true;
        }
    }

    fn event_release(&mut self, button: sfml::window::mouse::Button, x :i32, y: i32) {
        if button == sfml::window::mouse::Button::Left && x > self.x && x < self.x + self.width && y > self.y && y < self.y + self.height {
            self.pressed = false;
        }
    }

    fn is_down(&self) -> bool {
        self.down
    }

    fn is_pressed(&self) -> bool {
        self.pressed
    }

    fn is_hovered(&self) -> bool {
        self.hover
    }
    
}

