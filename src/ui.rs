use sfml::{graphics::{RenderWindow, Color}, system::Vector2i};



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
}

pub trait TButton {
    fn new(x: i32, y: i32, width: i32, height: i32, text: String, color: Color, hover_color: Color, pressed_color: Color, text_color: Color) -> Button;
    fn draw(&self, window: &mut RenderWindow);
    fn update(&mut self, mouse_pos: Vector2i);
    fn is_pressed(&self) -> bool;
}

impl TButton for Button {
    fn new(x: i32, y: i32, width: i32, height: i32, text: String, color: Color, hover_color: Color, pressed_color: Color, text_color: Color) -> Button {
        
    }
}

